
extern crate pretty_hex;
use pretty_hex::*;

use std::sync::{Arc, RwLock};
use crate::dbg::*;

use crate::mem::*;
use crate::bus::*;
use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;
use crate::dev::hlwd::irq::*;

/// The length of each page in the NAND flash, in bytes.
const NAND_PAGE_LEN: usize = 0x0000_0840;
/// The number of pages in the NAND flash.
const NUM_NAND_PAGES: usize = 0x0040_000;
/// The total length of the NAND flash, in bytes.
const NAND_SIZE: usize = NAND_PAGE_LEN * NUM_NAND_PAGES;

const NAND_ID: [u8; 5] = [ 0xad, 0xdc, 0x80, 0x95, 0x00 ]; // HY27UF084G2M

/// Types of NAND interface commands.
#[derive(Debug, Clone, Copy)]
pub enum NandCommand {
    /// Reset command (idempotent).
    Reset,

    /// Read some data from memory (used in bootloaders).
    ReadBoot { ecc: bool, r: bool, w: bool, len: u32 },

    /// Read the NAND chip ID (?).
    ReadId,

    /// Unknown command (does nothing?)
    Ignore,
}
impl From<u32> for NandCommand {
    fn from(x: u32) -> Self {
        let irq = (x & 0x4000_0000) != 0;
        let ecc = (x & 0x0000_1000) != 0;
        let r   = (x & 0x0000_2000) != 0;
        let w   = (x & 0x0000_4000) != 0;
        let len =  x & 0x0000_0fff;

        match (x & 0x00ff_0000) >> 16 {
            0x00 => NandCommand::Ignore,
            0x30 => NandCommand::ReadBoot { ecc, r, w, len },
            0x90 => NandCommand::ReadId,
            0xff => NandCommand::Reset,
            _ => panic!("Unhandled NandCommand {:08x}", x),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct NandRegisters {
    pub ctrl: u32,
    pub cfg: u32,
    pub addr1: u32, 
    pub addr2: u32,
    pub databuf: u32,
    pub eccbuf: u32,
    pub unk: u32,
}

/// Representing the state of the NAND interface.
pub struct NandInterface {
    pub dbg: Arc<RwLock<Debugger>>,
    pub data: Box<BigEndianMemory>,
    pub reg: NandRegisters,
}
impl NandInterface {
    /// Create a new instance of the NAND interface.
    pub fn new(dbg: Arc<RwLock<Debugger>>, filename: &str) -> Self {
        NandInterface {
            reg: NandRegisters::default(),
            data: Box::new(BigEndianMemory::new(NAND_SIZE, Some(filename))),
            dbg, 
        }
    }
}

impl NandInterface {
    /// Read data from NAND into a buffer, at the offset specified by the
    /// NAND interface registers.
    pub fn read_current_page(&self, dst: &mut [u8]) {
        self.data.read_buf(self.reg.addr2 as usize * NAND_PAGE_LEN, dst);
    }
}

impl MmioDevice for NandInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off { 
            0x00 => self.reg.ctrl,
            0x04 => self.reg.cfg,
            0x08 => self.reg.addr1,
            0x0c => self.reg.addr2,
            0x10 => self.reg.databuf,
            0x14 => self.reg.eccbuf,
            0x18 => {
                println!("NAND unimpl read from 0x18");
                self.reg.unk
            },
            _ => panic!("Unhandled NAND read at {:x} ", off),
        };
        //println!("NAND read {:08x} from {:02x}", val, off);
        BusPacket::Word(val)
    }

    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        //println!("NAND write {:08x} @ {:02x}", val, off);
        match off {
            0x00 => {
                if val & 0x8000_0000 != 0 {
                    self.reg.ctrl = val;
                    return Some(BusTask::Nand(val));
                } 
            },
            0x04 => self.reg.cfg = val,
            0x08 => self.reg.addr1 = val,
            0x0c => self.reg.addr2 = val,
            0x10 => self.reg.databuf = val,
            0x14 => self.reg.eccbuf = val,
            0x18 => {
                println!("NAND unimpl write to 0x18");
                self.reg.unk = val;
            }
            _ => panic!("Unhandled write32 on {:08x}", off),
        }
        None
    }
}

impl Bus {
    pub fn handle_task_nand(&mut self, val: u32) {
        let cmd = NandCommand::from(val);
        let reg = {
            let dev = self.dev.read().unwrap();
            dev.nand.reg
        };
        //println!("NAND cmd {:08x} addr1={:08x} addr2={:08x} data={:08x} ecc={:08x}", 
        //    val, reg.addr1, reg.addr2, reg.databuf, reg.eccbuf);
        match cmd {
            NandCommand::ReadBoot { ecc, r, w, len } => {
                assert!(r && ecc && !w);

                // Read data from the NAND, and get a copy of the registers.
                let mut local_buf = vec![0; len as usize];
                let reg = {
                    let dev = self.dev.read().unwrap();
                    dev.nand.read_current_page(&mut local_buf);
                    dev.nand.reg
                };

                //println!("NAND DMA write addr1={:08x} addr2={:08x} data={:08x} ecc={:08x}",
                //    reg.addr1, reg.addr2, reg.databuf, reg.eccbuf);

                // Do the DMA write
                self.dma_write(reg.databuf, &local_buf[..0x800]);
                self.dma_write(reg.eccbuf, &local_buf[0x800..]);

                // Compute and write the ECC bytes for the data.
                for i in 0..4 {
                    let addr = (reg.eccbuf ^ 0x40) + (i as u32 * 4);
                    let old_ecc = self.read32(addr);
                    let new_ecc = calc_ecc(&mut local_buf[(i * 0x200)..]);
                    //println!("NAND ECC write addr={:08x} old={:08x} new={:08x}",
                    //    addr, old_ecc, new_ecc);
                    self.write32(addr, new_ecc);
                }
            },

            // TODO: Why are these commands submitted with a length?
            NandCommand::ReadId => {
                let reg = { 
                    let dev = self.dev.read().unwrap();
                    dev.nand.reg
                };
                //println!("NAND READ ID to {:08x}", reg.databuf);
                self.dma_write(reg.databuf, &NAND_ID);
                //let mut buf = vec![0; 0x40];
                //self.dma_read(reg.databuf, &mut buf);
                //println!("{:?}", buf.hex_dump());
            },

            NandCommand::Ignore => {},
            NandCommand::Reset => {},
            _ => panic!("Unhandled NAND command {:?}", val),
        }

        // NOTE: `skyeye-starlet` always asserts the NAND IRQ line regardless 
        // of whether or not the command's IRQ bit is set
        self.dev.write().unwrap().hlwd.irq.assert(HollywoodIrq::Nand);

        // Mark the command as completed.
        self.dev.write().unwrap().nand.reg.ctrl &= 0x7fff_ffff;
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


