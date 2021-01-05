use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

#[derive(Default)]
pub struct SDInterface {
    /// Destination address for DMA.
    pub dma_addr: u32,
    /// SDHC Block Control Register
    pub bcon: u32,
    /// SDHC Argument Register
    pub arg: u32,
    /// SDHC Mode Register
    pub mode: u32,
    /// SDHC Response Register
    pub resp: [u32; 4],
    /// SDHC Data Register
    pub data: u32,
    /// SDHC Status Register 1
    pub stat1: u32,
    /// SDHC Control Register 1
    pub ctrl1: u32,
    /// SDHC Control Register 2
    pub ctrl2: u32,
    /// SDHC Interrupt Status Register
    pub intstat: u32,
    /// SDHC Interrupt Flag Enable Register
    pub inten: u32,
    /// SDHC Interrupt Signal Enable Register
    pub intsen: u32,
    /// SDHC Status Register 2
    pub stat2: u32,
    /// SDHC Capabilities Register
    pub cap: u32,
    /// SDHC Maximum Current Capabilities Register 
    pub maxcap: u32,
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
            0x24 => self.unk_24,
            //0x24 => 0x0001_0000, //self.unk_24,
            //0x40 => 0x0040_0000, //self.unk_24,
            //0xfc => self.unk_fc,
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
