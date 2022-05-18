
extern crate aes;
extern crate cbc;
extern crate pretty_hex;

use pretty_hex::*;
use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, BlockEncryptMut, KeyIvInit};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

use std::collections::VecDeque;

use crate::bus::*;
use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;
use crate::dev::hlwd::irq::*;

/// Representing a command to the AES interface.
#[derive(Debug)]
pub struct AesCommand {
    /// The length of the request
    len: usize,
    /// Toggle between encryption/decryption modes
    decrypt: bool,
    /// Enable AES functionality
    use_aes: bool,
    /// Enable chained IV mode
    chain_iv: bool,
    /// Fire an IRQ when a command completes
    irq: bool,
}
impl From<u32> for AesCommand {
    fn from(x: u32) -> Self {
        AesCommand {
            irq: (x & 0x4000_0000) != 0,
            use_aes: (x & 0x1000_0000) != 0,
            decrypt: (x & 0x0800_0000) != 0,
            chain_iv: (x & 0x0000_1000) != 0,
            len: (((x & 0x0000_0fff) + 1) * 0x10) as usize,
        }
    }
}


pub struct AesInterface {
    ctrl: u32,
    src: u32,
    dst: u32,
    key_fifo: VecDeque<u8>,
    iv_fifo: VecDeque<u8>,
    iv_buffer: [u8; 0x10],
}
impl AesInterface {
    pub fn new() -> Self {
        AesInterface {
            ctrl: 0, 
            src: 0,
            dst: 0,
            key_fifo: VecDeque::with_capacity(0x10),
            iv_fifo: VecDeque::with_capacity(0x10),
            iv_buffer: [0; 0x10]
        }
    }
    ///// Reset the AES interface.
    //fn reset(&mut self) {
    //    self.ctrl = 0;
    //    self.src = 0;
    //    self.dst = 0;
    //    self.key_fifo.clear();
    //    self.iv_fifo.clear();
    //    self.iv_buffer = [0; 0x10];
    //}
}

impl MmioDevice for AesInterface {
    type Width = u32;

    fn read(&mut self, off: usize) -> BusPacket {
        match off {
            //0x00 => BusPacket::Word(self.ctrl),
            0x00 => BusPacket::Word(0),
            _ => panic!("Unhandled AES interface read {:x}", off),
        }
    }

    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x00 => {
                self.ctrl = val;
                if val & 0x8000_0000 != 0 { 
                    return Some(BusTask::Aes(val));
                }
            },
            0x04 => self.src = val,
            0x08 => self.dst = val,
            0x0c => {
                if self.key_fifo.len() == 0x10 {
                    self.key_fifo.pop_front();
                    self.key_fifo.pop_front();
                    self.key_fifo.pop_front();
                    self.key_fifo.pop_front();
                }
                for b in val.to_be_bytes().iter() {
                    self.key_fifo.push_back(*b);
                }
                self.key_fifo.make_contiguous();
            },
            0x10 => {
                if self.iv_fifo.len() == 0x10 {
                    self.iv_fifo.pop_front();
                    self.iv_fifo.pop_front();
                    self.iv_fifo.pop_front();
                    self.iv_fifo.pop_front();
                }
                for b in val.to_be_bytes().iter() {
                    self.iv_fifo.push_back(*b);
                }
                self.iv_fifo.make_contiguous();
            }
            _ => panic!("Unimplemented AES write to offset {:x}", off),
        }
        None
    }
}

impl Bus {
    pub fn handle_task_aes(&mut self, val: u32) {
        let cmd = AesCommand::from(val);

        // Read data from the source address
        let mut aes_inbuf = vec![0u8; cmd.len];
        self.dma_read(self.aes.src, &mut aes_inbuf);

        if cmd.use_aes {
            // Build the right AES cipher for this request
            let key = self.aes.key_fifo.as_slices().0;
            let mut iv = [0u8; 0x10];
            if cmd.chain_iv {
                iv.copy_from_slice(&self.aes.iv_buffer);
            } else {
                iv.copy_from_slice(self.aes.iv_fifo.as_slices().0);
            }

            //println!("AES key={:02x?}", key);
            //println!("AES iv={:02x?}", iv);
            //println!("AES Decrypt src={:08x} dst={:08x} len={:08x}", 
            //  self.aes.src, self.aes.dst, cmd.len);

            let cipher_dec = Aes128CbcDec::new_from_slices(&key, &iv).unwrap();
            let cipher_enc = Aes128CbcEnc::new_from_slices(&key, &iv).unwrap();
            // Decrypt/encrypt the data, then DMA write to memory
            let aes_outbuf = if cmd.decrypt {
                cipher_dec.decrypt_padded_vec_mut::<NoPadding>(&aes_inbuf).unwrap()
            } else {
                cipher_enc.encrypt_padded_vec_mut::<NoPadding>(&aes_inbuf)
            };
            self.dma_write(self.aes.dst, &aes_outbuf);

            // Update IV buffer with the last 16 bytes of data
            self.aes.iv_buffer.copy_from_slice(&aes_inbuf[(cmd.len - 0x10)..]);
        } else {
            self.dma_write(self.aes.dst, &aes_inbuf);
        }

        // Update the source/destination registers exposed over MMIO
        self.aes.dst += cmd.len as u32;
        self.aes.src += cmd.len as u32;

        // Mark the command as completed
        self.aes.ctrl &= 0x7fff_ffff;

        if cmd.irq { 
            self.hlwd.irq.assert(HollywoodIrq::Aes);
        }

    }
}


