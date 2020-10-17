
use std::sync::{Arc, RwLock};
use crate::dbg::*;

use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

/// Fused/one-time programmable memory.
pub mod otp;

/// Interface to GPIO pins.
pub mod gpio;

/// Various bus control registers (?)
#[derive(Default, Debug, Clone)]
pub struct BusCtrlInterface {
    pub srnprot: u32,
    pub ahbprot: u32,
}
impl BusCtrlInterface {
    pub fn sram_mirror(&self) -> bool { (self.srnprot & 0x0000_0020) != 0 }
}

#[derive(Default, Debug, Clone)]
pub struct AhbInterface {
    pub unk_10: u32,
}
impl MmioDevice for AhbInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x10 => self.unk_10,
            _ => panic!("AHB read to undefined offset {:x}", off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x10 => self.unk_10 = val,
            _ => panic!("AHB write {:08x} to undefined offset {:x}", val, off),
        }
        None
    }
}


/// Hollywood memory-mapped registers
pub struct Hollywood {
    pub dbg: Arc<RwLock<Debugger>>,
    pub busctrl: BusCtrlInterface,
    pub ahb: AhbInterface,
    pub otp: otp::OtpInterface,
    pub gpio: gpio::GpioInterface,
}
impl Hollywood {
    pub fn new(dbg: Arc<RwLock<Debugger>>) -> Self {
        // TODO: Where do the initial values for these registers matter?
        Hollywood {
            dbg, 
            busctrl: BusCtrlInterface::default(),
            ahb: AhbInterface::default(),
            otp: otp::OtpInterface::new(),
            gpio: gpio::GpioInterface::default(),
        }
    }
}

impl MmioDevice for Hollywood {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x060           => self.busctrl.srnprot,
            0x0c0..=0x0d8   => self.gpio.ppc.read_handler(off - 0xc0),
            0x0dc..=0x0fc   => self.gpio.arm.read_handler(off - 0xdc),
            0x1ec           => self.otp.cmd,
            0x1f0           => self.otp.out,
            0x214           => 0x0000_0000,
            _ => panic!("Unimplemented Hollywood read at {:x}", off),
        };
        log(&self.dbg, LogLevel::Hlwd, &format!(
            "Read {:08x} from offset {:03x}", val, off));
        BusPacket::Word(val)
    }

    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        log(&self.dbg, LogLevel::Hlwd, &format!(
            "Write {:08x} to offset {:03x}", val, off));
        match off {
            0x060           => self.busctrl.srnprot = val,
            0x0c0..=0x0d8   => self.gpio.ppc.write_handler(off - 0xc0, val),
            0x0dc..=0x0fc   => self.gpio.arm.write_handler(off - 0xdc, val),
            0x1ec           => self.otp.write_handler(val),
            _ => panic!("Unimplemented Hollywood write at {:x}", off),
        }
        None
    }

}






