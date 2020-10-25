
use std::sync::{Arc, RwLock};
use crate::dbg::*;

use crate::mem::*;
use crate::bus::*;
use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

/// Length of pages in NAND flash.
const NAND_PAGE_LEN: usize = 0x0000_0840;

/// Number of pages in NAND flash.
//const NUM_NAND_PAGES: usize = 0x0010_0000;
const NUM_NAND_PAGES: usize = 0x0040_000;

/// Types of NAND interface commands.
#[derive(Debug, Clone, Copy)]
pub enum NandCommand {
    /// Reset command (idempotent).
    Reset(u32),
    /// Read some data from memory (used in bootloaders).
    ReadBoot(u32),
    /// Read the NAND chip ID (?).
    ReadId(u32),
    /// Unknown command (does nothing?)
    Ignore(u32),
}
impl From<u32> for NandCommand {
    fn from(x: u32) -> Self {
        match (x & 0x00ff_0000) >> 16 {
            0x00 => NandCommand::Ignore(x),
            0x30 => NandCommand::ReadBoot(x),
            0x90 => NandCommand::ReadId(x),
            0xff => NandCommand::Reset(x),
            _ => panic!("Unhandled NandCommand {:08x}", x),
        }
    }
}


/// Representing the state of the NAND interface.
pub struct NandInterface {
    pub dbg: Arc<RwLock<Debugger>>,
    pub data: Box<BigEndianMemory>,
    pub ctrl: u32,
    pub cfg: u32,
    pub addr1: u32, 
    pub addr2: u32,
    pub databuf: u32,
    pub eccbuf: u32,
    pub unk: u32,
}
impl NandInterface {
    /// Create a new instance of the NAND interface.
    pub fn new(dbg: Arc<RwLock<Debugger>>, filename: &str) -> Self {
        NandInterface {
            dbg, 
            data: Box::new(BigEndianMemory::new(
                NAND_PAGE_LEN * NUM_NAND_PAGES, Some(filename))),
            ctrl: 0,
            cfg: 0,
            addr1: 0,
            addr2: 0,
            databuf: 0,
            eccbuf: 0,
            unk: 0,
        }
    }
    ///// Reset the state of the NAND interface.
    //fn reset(&mut self) {
    //    self.ctrl = 0;
    //    self.cfg = 0;
    //    self.addr1 = 0;
    //    self.addr2 = 0;
    //    self.databuf = 0;
    //    self.eccbuf = 0;
    //    self.unk = 0;
    //}
}

impl MmioDevice for NandInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off { 
            0x00 => self.ctrl,
            0x04 => self.cfg,
            0x08 => self.addr1,
            0x0c => self.addr2,
            0x10 => self.databuf,
            0x14 => self.eccbuf,
            _ => panic!("Unhandled AES read at {:x} ", off),
        };
        BusPacket::Word(val)
    }

    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x00 => {
                self.ctrl = val;
                if val & 0x8000_0000 != 0 {
                    return Some(BusTask::Nand(val));
                }
            },
            0x04 => self.cfg = val,
            0x08 => self.addr1 = val,
            0x0c => self.addr2 = val,
            0x10 => self.databuf = val,
            0x14 => self.eccbuf = val,
            _ => panic!("Unhandled write32 on {:08x}", off),
        }
        None
    }
}

impl Bus {
    pub fn handle_task_nand(&mut self, val: u32) {

        let local_ref = self.dev.clone();
        let mut dev = local_ref.write().unwrap();
        let nand = &mut dev.nand;

        // Perform the scheduled command
        let cmd = NandCommand::from(val);
        match cmd {
            NandCommand::ReadBoot(cmd) => {
                let irq_req     = cmd & 0x4000_0000 != 0;
                let ecc_flag    = cmd & 0x0000_1000 != 0;
                let read_flag   = cmd & 0x0000_2000 != 0;
                let _write_flag  = cmd & 0x0000_4000 != 0;
                let data_len    = cmd & 0x0000_0fff;

                assert!(read_flag);
                assert!(!irq_req);
                assert!(data_len == NAND_PAGE_LEN as u32);
                assert!(ecc_flag);

                println!("NAND page {:08x}, DMA write data={:08x} ecc={:08x}",
                    nand.addr2, nand.databuf, nand.eccbuf);
              
                let nand_offset = nand.addr2 as usize * NAND_PAGE_LEN;
                let mut buf = vec![0; data_len as usize];

                nand.data.read_buf(nand_offset, &mut buf);

                self.dma_write(nand.databuf, &buf[..0x800]);
                self.dma_write(nand.eccbuf, &buf[0x800..]);
                for i in 0..4 {
                    let addr = (nand.eccbuf ^ 0x40) + (i as u32 * 4);
                    let old_ecc = self.read32(addr);
                    let new_ecc = calc_ecc(&mut buf[(i * 0x200)..]);
                    println!("NAND ECC write addr={:08x} old={:08x} new={:08x}",
                        addr, old_ecc, new_ecc);
                    self.write32(addr, new_ecc);
                }

            },
            NandCommand::Reset(_)  => {},
            NandCommand::Ignore(_) => {},
            _ => panic!("Unhandled NAND command {:?}", cmd),
        }

        // Mark the command as completed.
        nand.ctrl &= 0x7fff_ffff;
    }
}

pub fn parity(input: u8) -> u8 { (input.count_ones() % 2) as u8 }

#[allow(unused_assignments)]
pub fn calc_ecc(data: &mut [u8]) -> u32 {
    let mut a = [[0u8; 2]; 12];
    let mut a0 = 0u32;
    let mut a1 = 0u32;
    let mut x = 0u8;

    for i in 0..512 {
        x = data[i];
        for j in 0..9 {
            a[3 + j][(i >> j) & 1] ^= x;
        }
    }

    x = a[3][0] ^ a[3][1];
    a[0][0] = x & 0x55;
    a[0][1] = x & 0xaa;
    a[1][0] = x & 0x33;
    a[1][1] = x & 0xcc;
    a[2][0] = x & 0x0f;
    a[2][1] = x & 0xf0;

    for j in 0..12 {
        a[j][0] = parity(a[j][0]);
        a[j][1] = parity(a[j][1]);
    }
    for j in 0..12 {
        a0 |= (a[j][0] as u32) << j;
        a1 |= (a[j][1] as u32) << j;
    }


    (a0 & 0x0000_00ff) << 24 | (a0 & 0x0000_ff00) << 8 |
    (a1 & 0x0000_00ff) << 8  | (a1 & 0x0000_ff00) >> 8
}


