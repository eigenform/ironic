use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

#[derive(Default)]
pub struct SDInterface {
}
impl MmioDevice for SDInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            _ => panic!("SDHC0 read at {:x} unimpl", off),
        };
        //BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            _ => panic!("SDHC0 write {:08x} at {:x} unimpl", val, off),
        }
        None
    }
}

#[derive(Default)]
pub struct WLANInterface {
    pub unk_24: u32,
    pub unk_40: u32,
    pub unk_fc: u32,
}

impl MmioDevice for WLANInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x24 => 0x0001_0000, //self.unk_24,
            0x40 => 0x0040_0000, //self.unk_24,
            0xfc => self.unk_fc,
            _ => panic!("SDHC1 read at {:x} unimpl", off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            _ => panic!("SDHC1 write {:08x} at {:x} unimpl", val, off),
        }
        None
    }
}
