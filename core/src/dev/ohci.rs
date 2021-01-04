
use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

#[derive(Default)]
pub struct OhcInterface {
    pub idx: usize,
    pub unk_00: u32,
    pub unk_14: u32,
}

impl MmioDevice for OhcInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x00 => self.unk_00,
            _ => panic!("OHCI#{} read at {:x} unimpl", self.idx, off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x14 => self.unk_14 = val,
            _ => panic!("OHCI#{} write {:08x} at {:x} unimpl", self.idx, val, off),
        }
        None
    }
}
