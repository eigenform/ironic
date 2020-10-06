
use std::collections::VecDeque;

use crate::bus::*;
use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

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
}
impl From<u32> for AesCommand {
    fn from(x: u32) -> Self {
        AesCommand {
            len: (((x & 0x0000_0fff) + 1) * 0x10) as usize,
            decrypt: (x & 0x1000_0000) != 0,
            use_aes: (x & 0x0800_0000) != 0,
            chain_iv: (x & 0x0000_1000) != 0,
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
    /// Reset the AES interface.
    fn reset(&mut self) {
        self.ctrl = 0;
        self.src = 0;
        self.dst = 0;
        self.key_fifo.clear();
        self.iv_fifo.clear();
        self.iv_buffer = [0; 0x10];
    }
}

impl MmioDevice for AesInterface {
    type Width = u32;

    fn read(&mut self, off: usize) -> BusPacket {
        match off {
            0x00 => BusPacket::Word(self.ctrl),
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
                    self.key_fifo.clear();
                }
                for b in val.to_be_bytes().iter() {
                    self.key_fifo.push_back(*b);
                }
            },
            0x10 => {
                if self.iv_fifo.len() == 0x10 {
                    self.iv_fifo.clear();
                }
                for b in val.to_be_bytes().iter() {
                    self.iv_fifo.push_back(*b);
                }
            }
            _ => panic!("Unimplemented AES write to offset {:x}", off),
        }
        None
    }
}

impl Bus {
    pub fn handle_task_aes(&mut self, val: u32) {
        panic!("AES task handler unimplemented")
    }
}


