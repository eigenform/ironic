use crate::bus::mmio::*;
use crate::bus::prim::*;
use crate::bus::task::*;

/// Legacy disc drive interface.
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
            0x00 => self.disr,
            0x04 => self.dicvr,
            0x24 => self.dicfg,
            _ => panic!("DI read to undefined offset {:x}", off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x00 => self.disr = val,
            0x04 => self.dicvr = val,
            _ => panic!("DI write {:08x?} to undefined offset {:x}", val, off),
        }
        None
    }
}


