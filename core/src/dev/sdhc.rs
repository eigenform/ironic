use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

#[derive(Default)]
pub struct SDHCInterface {
    pub idx: usize,
    pub unk_24: u32,
}

impl MmioDevice for SDHCInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x24 => self.unk_24,
            _ => panic!("SDHC#{} read at {:x} unimpl", self.idx, off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            _ => panic!("SDHC#{} write {:08x} at {:x} unimpl", self.idx, val, off),
        }
        None
    }
}
