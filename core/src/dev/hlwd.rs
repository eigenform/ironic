
use std::sync::{Arc, RwLock};
use crate::dbg::*;

use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

/// One-time programmable [fused] memory.
pub mod otp;

/// Interface to GPIO pins.
pub mod gpio;

/// Flipper-compatible interfaces.
pub mod compat;

/// GDDR3 interface.
pub mod ddr;

#[derive(Default, Debug, Clone)]
pub struct ClockInterface {
    pub ddr: u32,
    pub ddr_ext: u32,
}

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
    pub unk_08: u32,
    pub unk_10: u32,
}
impl MmioDevice for AhbInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x08 => self.unk_08,
            0x10 => self.unk_10,
            _ => panic!("AHB read to undefined offset {:x}", off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x08 => self.unk_08 = val,
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
    pub pll: ClockInterface,
    pub otp: otp::OtpInterface,
    pub gpio: gpio::GpioInterface,

    pub di: compat::DriveInterface,
    pub mi: compat::MemInterface,
    pub ahb: AhbInterface,
    pub ddr: ddr::DdrInterface,

    pub arb_cfg_m: [u32; 0x10],
    pub timer: u32,
    pub resets: u32,
    pub compat: u32,
    pub spare0: u32,
    pub spare1: u32,
}
impl Hollywood {
    pub fn new(dbg: Arc<RwLock<Debugger>>) -> Self {
        // TODO: Where do the initial values for these registers matter?
        Hollywood {
            dbg, 
            busctrl: BusCtrlInterface::default(),
            otp: otp::OtpInterface::new(),
            gpio: gpio::GpioInterface::default(),
            pll: ClockInterface::default(),

            ahb: AhbInterface::default(),
            di: compat::DriveInterface::default(),
            mi: compat::MemInterface::new(),
            ddr: ddr::DdrInterface::new(),

            arb_cfg_m: [0; 0x10],
            resets: 0,
            timer: 0,
            compat: 0,
            spare0: 0,
            spare1: 0,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(usize)]
pub enum HollywoodReg {
    TIMER,
    SRNPROT,
    FUSE_CMD,
    FUSE_VAL,
    COMPAT,
    SPARE0,
    SPARE1,
    RESETS,
    PLL_DDR,
    PLL_DDR_EXT,
    VERSION,

    UNNAMED,
}

impl From<usize> for HollywoodReg {
    fn from(x: usize) -> Self {
        use HollywoodReg::*;
        match x {
            0x10 => TIMER,
            0x60 => SRNPROT,
            0x180 => COMPAT,
            0x188 => SPARE0,
            0x18c => SPARE1,

            0x1ec => FUSE_CMD,
            0x1f0 => FUSE_VAL,
            0x214 => VERSION,
            _ => UNNAMED,
        }
    }
}




impl MmioDevice for Hollywood {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x010           => self.timer,
            0x060           => self.busctrl.srnprot,
            0x0c0..=0x0d8   => self.gpio.ppc.read_handler(off - 0xc0),
            0x0dc..=0x0fc   => self.gpio.arm.read_handler(off - 0xdc),
            0x100..=0x13c   => self.arb_cfg_m[(off - 0x100) / 4],
            0x180           => self.compat,
            0x188           => self.spare0,
            0x18c           => self.spare1,
            0x194           => self.resets,
            0x1bc           => self.pll.ddr,
            0x1c0           => self.pll.ddr_ext,
            0x1ec           => self.otp.cmd,
            0x1f0           => self.otp.out,
            0x214           => 0x0000_0000,
            _ => panic!("Unimplemented Hollywood read at {:x}", off),
        };

        let regname = format!("{:?}", HollywoodReg::from(off));
        log(&self.dbg, LogLevel::Hlwd, &format!(
            "Read {:08x} from offset {:03x} ({})", val, off, regname));
        BusPacket::Word(val)
    }

    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        let regname = format!("{:?}", HollywoodReg::from(off));
        log(&self.dbg, LogLevel::Hlwd, &format!(
            "Write {:08x} to offset {:03x} ({})", val, off, regname));

        match off {
            0x060           => self.busctrl.srnprot = val,
            0x0c0..=0x0d8   => self.gpio.ppc.write_handler(off - 0xc0, val),
            0x0dc..=0x0fc   => self.gpio.arm.write_handler(off - 0xdc, val),
            0x100..=0x13c   => self.arb_cfg_m[(off - 0x100) / 4] = val,
            0x180           => self.compat = val,
            0x188           => {
                if (val & 0x0001_0000) != 0 {
                    self.spare1 &= 0xffff_fff6;
                } else {
                    self.spare1 |= 0x0000_0009;
                }
                self.spare0 = val;
            }
            0x194           => self.resets = val,
            0x1bc           => self.pll.ddr = val,
            0x1c0           => self.pll.ddr_ext = val,
            0x1ec           => self.otp.write_handler(val),
            _ => panic!("Unimplemented Hollywood write at {:x}", off),
        }
        None
    }

}






