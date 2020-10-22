use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;
use crate::bus::Bus;

/// Disc drive interface.
#[derive(Default, Debug, Clone)]
pub struct DriveInterface {
    disr: u32,
    dicvr: u32,
    dicmdbuf: [u32; 3],
    dimar: u32,
    dilength: u32,
    dicr: u32,
    diimmbuf: u32,
    dicfg: u32,
}
impl MmioDevice for DriveInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x24 => self.dicfg,
            _ => panic!("DI read to undefined offset {:x}", off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            _ => panic!("DI write {:08x} to undefined offset {:x}", val, off),
        }
        None
    }
}



const MEM_SPACE_LEN: usize = 0x40;
/// Legacy memory interface.
#[derive(Clone)]
pub struct MemInterface {
    pub reg: [u16; MEM_SPACE_LEN],
    pub ddr_data: u16,
    pub ddr_addr: u16,
}
impl MemInterface {
    pub fn new() -> Self {
        MemInterface {
            reg: [0; MEM_SPACE_LEN],
            ddr_data: 0,
            ddr_addr: 0,
        }
    }
}
impl MmioDevice for MemInterface {
    type Width = u16;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x74 => self.ddr_addr,
            0x76 => self.ddr_data,
            _ => self.reg[off / 2],
        };
        BusPacket::Half(val)
    }
    fn write(&mut self, off: usize, val: u16) -> Option<BusTask> {
        let task = match off {
            0x74 => Some(BusTask::Mi { kind: TaskType::Read, data: val }),
            0x76 => Some(BusTask::Mi { kind: TaskType::Write, data: val }),
            _ => { self.reg[off / 2] = val; None }
        };
        task
    }
}

