
extern crate pretty_hex;
use pretty_hex::*;

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

/// The length of each block in the NAND flash, in bytes.
const NAND_BLOCK_LEN: usize = NAND_PAGE_LEN * 64;

/// The number of pages in the NAND flash.
const NUM_NAND_PAGES: usize = 0x0040_000;

/// The total length of the NAND flash, in bytes.
const NAND_SIZE: usize = NAND_PAGE_LEN * NUM_NAND_PAGES;

/// NAND device ID.
const NAND_ID: [u8; 4] = [ 0xad, 0xdc, 0x80, 0x95 ]; // HY27UF084G2M


/// NAND command opcodes.
#[derive(Debug)]
pub enum NandOpcd {
    /// Page read setup
    PrefixRead,
    /// Page read command
    Read,

    /// Read status register
    ReadStatus,
    /// Read ID command (manufacturer code, device code, etc).
    ReadId,

    /// Block erase setup
    PrefixErase,
    /// Block erase
    Erase,

    SerialInput,
    RandInput,
    Program,

    /// Reset command
    Reset,
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
            0x00 => PrefixRead, 
            0x10 => Program,
            0x30 => Read, 
            0x60 => PrefixErase,
            0x70 => ReadStatus,
            0x80 => SerialInput,
            0x85 => RandInput,
            0x90 => ReadId, 
            0xd0 => Erase, 
            0xff => Reset,
            _ => panic!("unhandled NAND opcd {:02x}", (x & 0x00ff_0000) >> 16),
        };
        let wait = (x & 0x0000_8000) != 0;
        let wr   = (x & 0x0000_4000) != 0;
        let rd   = (x & 0x0000_2000) != 0;
        let ecc  = (x & 0x0000_1000) != 0;
        let len  =  x & 0x0000_0fff;
        NandCmd { irq, err, addr, opcd, wait, wr, rd, ecc, len }
    }
}

#[derive(Clone, Copy)]
pub enum NandState {
    Wait, Cmd, Addr
}

/// Set of registers exposed by the NAND interface.
#[derive(Clone, Copy)]
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
    /// Internal address bits
    pub current_page: u32,
    pub current_poff: u32,
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
        let reg = NandRegisters {
            ctrl: 0,
            cfg: 0,
            addr1: 0,
            addr2: 0,
            databuf: 0,
            eccbuf: 0,
            unk: 0,
            _cycle: 0,
            current_page: 0,
            current_poff: 0,
        };
        NandInterface {
            data: Box::new(BigEndianMemory::new(NAND_SIZE, Some(filename))),
            reg,
        }
    }
    /// Read data from the specified offset in the NAND flash into some buffer
    pub fn read_data(&self, off: usize, dst: &mut [u8]) {
        self.data.read_buf(off, dst);
    }
    /// Write the provided data to the specified offset in the NAND flash
    pub fn write_data(&mut self, off: usize, src: &[u8]) {
        self.data.write_buf(off, src);
    }
    /// Zero out the provided region in the NAND flash
    pub fn clear_data(&mut self, off: usize, len: usize) {
        self.data.memset(off, len, 0xff);
    }

    pub fn send_addr(&mut self, x: u32) {
        let cmd = NandCmd::new(x);
        let addr2 = self.reg.addr2;
        let addr1 = self.reg.addr1;

        // The top three bits send a particular page (addr2 & 0x00ff_ffff)
        if cmd.addr & 0b00100 != 0 { 
            self.reg.current_page = (addr2 & 0x0000_00ff) | 
                (self.reg.current_page & !0x0000_00ff);
        }
        if cmd.addr & 0b01000 != 0 { 
            self.reg.current_page = (addr2 & 0x0000_ff00) | 
                (self.reg.current_page & !0x0000_ff00);
        }
        if cmd.addr & 0b10000 != 0 { 
            self.reg.current_page = (addr2 & 0x00ff_0000) | 
                (self.reg.current_page & !0x00ff_0000);
        }

        // The bottom two bits send an offset into the page
        if cmd.addr & 0b00001 != 0 { 
            self.reg.current_poff = (addr1 & 0x0000_00ff) |
                (self.reg.current_poff & !0x0000_00ff);
        }
        if cmd.addr & 0b00010 != 0 { 
            self.reg.current_poff = (addr1 & 0x0000_ff00) |
                (self.reg.current_poff & !0x0000_ff00);
        }

    }
}

impl MmioDevice for NandInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off { 
            //0x00 => self.reg.ctrl,
            0x00 => 0x0000_0001,
            0x04 => self.reg.cfg,
            0x08 => self.reg.addr1,
            0x0c => self.reg.addr2,
            0x10 => self.reg.databuf,
            0x14 => self.reg.eccbuf,
            0x18 => {
                println!("NND unimpl read from 0x18");
                self.reg.unk
            },
            _ => panic!("Unhandled NND read at {:x} ", off),
        };
        BusPacket::Word(val)
    }

    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x00 => {
                // When this bit is set, emit command to NAND flash
                if val & 0x8000_0000 != 0 {
                    self.reg.ctrl = val;
                    self.send_addr(val);
                    return Some(BusTask::Nand(val));
                } 
            },
            0x04 => self.reg.cfg = val,
            0x08 => self.reg.addr1 = val,
            0x0c => self.reg.addr2 = val,
            0x10 => self.reg.databuf = val,
            0x14 => self.reg.eccbuf = val,
            0x18 => {
                println!("NND unimpl write to 0x18");
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

    fn nand_erase_page(&mut self, cmd: &NandCmd, reg: &NandRegisters) {
        assert_ne!(cmd.ecc, true);
        assert_ne!(cmd.rd, true);
        let off = reg.addr2 as usize * NAND_PAGE_LEN;
        self.dev.write().unwrap().nand.clear_data(off, NAND_BLOCK_LEN);
        //panic!("nand erase unimpl");
    }

    /// Perform a NAND read into memory
    fn nand_read_page(&mut self, cmd: &NandCmd, reg: &NandRegisters) {
        // Read the source data from the NAND
        let mut local_buf = vec![0; cmd.len as usize];

        let off = reg.addr2 as usize * NAND_PAGE_LEN;
        self.dev.read().unwrap().nand.read_data(off, &mut local_buf);

        //println!("{:?}", local_buf.hex_dump());

        // Do the DMA writes to memory
        self.dma_write(reg.databuf, &local_buf[..0x800]);
        self.dma_write(reg.eccbuf, &local_buf[0x800..]);

        // Compute and write the ECC bytes for the data
        for i in 0..4 {
            let addr = (reg.eccbuf ^ 0x40) + (i as u32 * 4);
            let new_ecc = calc_ecc(&mut local_buf[(i * 0x200)..]);
            let old_ecc = self.read32(addr);
            //println!("NND old_ecc={:08x} new_ecc={:08x}", old_ecc, new_ecc);
            self.write32(addr, new_ecc);
        }
    }

    /// Write a NAND page (its okay that this is a mess, for now..)
    fn nand_write_page(&mut self, cmd: &NandCmd, reg: &NandRegisters) {
        // Read from memory
        let mut local_buf = vec![0; cmd.len as usize];
        self.dma_read(reg.databuf, &mut local_buf);

        let off = (reg.current_page as usize * NAND_PAGE_LEN) + 
            reg.current_poff as usize;
        self.dev.write().unwrap().nand.write_data(off, &local_buf);

        if cmd.ecc {
            assert!(cmd.len == 0x800);
            for i in 0..4 {
                let addr = (reg.eccbuf ^ 0x40) + (i as u32 * 4);
                let new_ecc = calc_ecc(&mut local_buf[(i * 0x200)..]);
                self.write32(addr, new_ecc);
            }
        }


    }

    /// Handle a NAND command
    pub fn handle_task_nand(&mut self, val: u32) {
        use NandOpcd::*;
        let cmd = NandCmd::new(val);
        let reg = self.read_nand_regs();
        let mut next_cycle = 0;

        //println!("NND cmd={:?} addr={:05b} addr1={:08x} addr2={:08x} databuf={:08x} eccbuf={:08x} len={:08x}",
        //    cmd.opcd, cmd.addr, reg.addr1, reg.addr2, reg.databuf, reg.eccbuf, cmd.len);

        // Execute a NAND command.
        // This is kind of messed up because it's nicer to think about some
        // [NandInterface] having state that changes in the MMIO handler,
        // and then *maybe* tells the bus that it needs to do something
        match reg._cycle {
            0 => {
                match cmd.opcd {
                    SerialInput => {
                        next_cycle = reg._cycle + 1;
                        self.nand_write_page(&cmd, &reg);
                    },
                    PrefixRead  => next_cycle = reg._cycle + 1,
                    PrefixErase => next_cycle = reg._cycle + 1,
                    ReadId      => self.dma_write(reg.databuf, &NAND_ID),
                    ReadStatus  => {
                        let status_register: [u8;1] = [0xe0];
                        self.dma_write(reg.databuf, &status_register);
                    },
                    Reset       => {},
                    _ => panic!("NAND unknown cycle 0 opcd {:?}", cmd.opcd),
                }
            },
            1 => match cmd.opcd {
                RandInput => {
                    next_cycle = reg._cycle + 1;
                    self.nand_write_page(&cmd, &reg);
                },
                Read    => self.nand_read_page(&cmd, &reg),
                Erase   => self.nand_erase_page(&cmd, &reg),
                _ => panic!("NAND unknown cycle 1 opcd {:?}", cmd.opcd),
            },
            2 => match cmd.opcd {
                // NOTE: For now, we can probably just do the page programming
                // when the Serial/Random input commands arrive, and just not
                // do anything when we see the actual program command
                Program => {
                },
                _ => panic!("NAND unknown cycle 2 opcd {:?}", cmd.opcd),
            },
            _ => panic!("NAND desync cycle {}", reg._cycle),
        }

        // Get a mutable reference to the system devices and commit any state 
        // that we need to change
        {
            let mut dev = self.dev.write().unwrap();

            // NOTE: Skyeye *always* asserts an IRQ?
            dev.hlwd.irq.assert(HollywoodIrq::Nand);

            // Assert an IRQ if requested in the command
            //if cmd.irq { 
            //    println!("NND IRQ assert by cmd {:?}", cmd.opcd);
            //    dev.hlwd.irq.assert(HollywoodIrq::Nand); 
            //}

            // Mark this command as completed
            dev.nand.reg.ctrl &= 0x7fff_ffff;
            // Increment cycle counter for NAND state machine
            dev.nand.reg._cycle = next_cycle;
        }

    }
}


