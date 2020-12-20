
extern crate pretty_hex;

pub mod util;
use crate::dev::nand::util::*;

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

/// NAND device ID.
const NAND_ID: [u8; 4] = [ 0xad, 0xdc, 0x80, 0x95 ]; // HY27UF084G2M


/// NAND command opcodes.
#[derive(Debug)]
pub enum NandOpcd {
    /// First-cycle prefix command
    Prefix00    = 0x00,
    /// Page read command
    Read        = 0x30,
    /// Read ID command (manufacturer code, device code, etc).
    ReadId      = 0x90,
    /// Reset command
    Reset       = 0xff,
}

/// Represents a NAND command.
#[derive(Debug)]
pub struct NandCmd {
    /// Set when an IRQ should be asserted on command completion.
    pub irq: bool,
    pub err: bool,
    pub addr: u32,
    /// The type of command.
    pub opcd: NandOpcd,
    pub wait: bool,
    /// Write flag.
    pub wr: bool,
    /// Read flag.
    pub rd: bool,
    pub ecc: bool,
    /// Length of the associated read or write.
    pub len: u32,
}
impl NandCmd {
    pub fn new(x: u32) -> Self {
        use NandOpcd::*;
        let irq  = (x & 0x4000_0000) != 0;
        let err  = (x & 0x2000_0000) != 0;
        let addr = (x & 0x1f00_0000) >> 24;
        let opcd = match (x & 0x00ff_0000) >> 16 {
            0x00 => Prefix00, 
            0x30 => Read, 
            0x90 => ReadId, 
            0xff => Reset,
            _ => panic!("unhandled NAND opcd"),
        };
        let wait = (x & 0x0000_8000) != 0;
        let wr   = (x & 0x0000_4000) != 0;
        let rd   = (x & 0x0000_2000) != 0;
        let ecc  = (x & 0x0000_1000) != 0;
        let len  =  x & 0x0000_0fff;
        NandCmd { irq, err, addr, opcd, wait, wr, rd, ecc, len }
    }
}

/// Set of registers exposed by the NAND interface.
#[derive(Default, Clone, Copy)]
pub struct NandRegisters {
    pub ctrl: u32,
    pub cfg: u32,
    pub addr1: u32, 
    pub addr2: u32,
    pub databuf: u32,
    pub eccbuf: u32,
    pub unk: u32,

    /// Naive cycle counter for managing state.
    pub _cycle: usize,
}

/// Representing the state of the NAND interface.
pub struct NandInterface {
    /// Actual backing data for the NAND flash.
    pub data: Box<BigEndianMemory>,
    /// Set of registers associated with this interface.
    pub reg: NandRegisters,
}
impl NandInterface {
    /// Create a new instance of the NAND interface.
    pub fn new(filename: &str) -> Self {
        NandInterface {
            reg: NandRegisters::default(),
            data: Box::new(BigEndianMemory::new(NAND_SIZE, Some(filename))),
        }
    }
}

impl NandInterface {
    /// Read data from the specified offset in NAND flash into a buffer
    pub fn read_data(&self, off: usize, dst: &mut [u8]) {
        self.data.read_buf(off, dst);
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
                // When this bit is set, emit command to NAND flash
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
    fn read_nand_regs(&mut self) -> NandRegisters {
        self.dev.read().unwrap().nand.reg
    }

    /// Perform a NAND read into memory
    fn do_nand_dma(&mut self, cmd: &NandCmd, reg: &NandRegisters) {
        // Read the source data from the NAND
        let mut local_buf = vec![0; cmd.len as usize];

        let off = reg.addr2 as usize * NAND_PAGE_LEN;
        self.dev.read().unwrap().nand.read_data(off, &mut local_buf);

        //println!("NAND DMA write addr1={:08x} addr2={:08x} data={:08x} ecc={:08x}",
        //  reg.addr1, reg.addr2, reg.databuf, reg.eccbuf);

        //let mut tmp = vec![0; 8];
        //tmp.copy_from_slice(&local_buf[0..8]);
        //println!("{:?}", local_buf.hex_dump());

        // Do the DMA writes to memory
        self.dma_write(reg.databuf, &local_buf[..0x800]);
        self.dma_write(reg.eccbuf, &local_buf[0x800..]);

        // Compute and write the ECC bytes for the data
        for i in 0..4 {
            let addr = (reg.eccbuf ^ 0x40) + (i as u32 * 4);
            let new_ecc = calc_ecc(&mut local_buf[(i * 0x200)..]);
            //let old_ecc = self.read32(addr);
            //println!("NAND ECC write addr={:08x} old={:08x} new={:08x}",
            //    addr, old_ecc, new_ecc);
            self.write32(addr, new_ecc);
        }
    }

    /// Handle a NAND command
    pub fn handle_task_nand(&mut self, val: u32) {
        let cmd = NandCmd::new(val);
        let reg = self.read_nand_regs();
        assert!(cmd.wr == false);
        let mut next_cycle = 0;

        // Execute a command
        match reg._cycle {
            0 => {
                match cmd.opcd {
                    NandOpcd::Prefix00 => next_cycle = reg._cycle + 1,
                    NandOpcd::ReadId => self.dma_write(reg.databuf, &NAND_ID),
                    NandOpcd::Reset => {},
                    _ => panic!("NAND unknown cycle 0 opcd {:?}", cmd.opcd),
                }
            },
            1 => match cmd.opcd {
                NandOpcd::Read => self.do_nand_dma(&cmd, &reg),
                _ => panic!("NAND unknown cycle 1 opcd {:?}", cmd.opcd),
            },
            _ => panic!("NAND desync cycle {}", reg._cycle),
        }

        // Get a mutable reference to the system devices and commit any state 
        // that we need to change
        {
            let mut dev = self.dev.write().unwrap();
            // Potentially assert an IRQ
            if cmd.irq { dev.hlwd.irq.assert(HollywoodIrq::Nand); }
            // Mark this command as completed
            dev.nand.reg.ctrl &= 0x7fff_ffff;
            // Increment cycle counter for NAND state machine
            dev.nand.reg._cycle = next_cycle;
        }

    }
}


