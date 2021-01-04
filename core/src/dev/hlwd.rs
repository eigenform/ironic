
use std::sync::{Arc, RwLock};

use crate::bus::*;
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
/// Interrupt controller.
pub mod irq;

/// The inter-processor communication interface.
#[derive(Default, Debug, Clone)]
pub struct IpcInterface {
    pub ppc_msg: u32,
    pub ppc_ctrl: u32,
    pub arm_msg: u32,
    pub arm_ctrl: u32,
}

/// The timer/alarm interface.
#[derive(Default, Debug, Clone)]
pub struct TimerInterface {
    pub timer: u32,
    pub alarm: u32,
}
impl TimerInterface {
    pub fn step(&mut self) {
        self.timer += 1;
        if self.timer == self.alarm {
            println!("HLWD alarm interrupt {:08x} == {:08x}", 
                self.timer, self.alarm);
            panic!("Timer/alarm interrupts are unimplemented");
        }
    }
}

/// Various clocking registers.
#[derive(Default, Debug, Clone)]
pub struct ClockInterface {
    pub sys: u32,       // 0x1b0
    pub sys_ext: u32,   // 0x1b4
    pub ddr: u32,       // 0x1bc
    pub ddr_ext: u32,   // 0x1c0
    pub vi_ext: u32,    // 0x1c8
    pub ai: u32,        // 0x1cc
    pub ai_ext: u32,    // 0x1d0
    pub usb_ext: u32,   // 0x1d8
}

/// Various bus control registers (?)
#[derive(Default, Debug, Clone)]
pub struct BusCtrlInterface {
    pub srnprot: u32,
    pub ahbprot: u32,
    pub aipprot: u32,
}

#[derive(Default, Debug, Clone)]
pub struct ArbCfgInterface {
    pub m0: u32,
    pub m1: u32,
    pub m2: u32,
    pub m3: u32,
    pub m4: u32,
    pub m5: u32,
    pub m6: u32,
    pub m7: u32,
    pub m8: u32,
    pub m9: u32,
    pub ma: u32,
    pub mb: u32,
    pub mc: u32,
    pub md: u32,
    pub me: u32,
    pub mf: u32,
    pub cpu: u32,
    pub dma: u32,
}
impl ArbCfgInterface {
    fn read_handler(&mut self, off: usize) -> u32 {
        match off {
            0x00 => self.m0,
            0x04 => self.m1,
            0x08 => self.m2,
            0x0c => self.m3,
            0x10 => self.m4,
            0x14 => self.m5,
            0x18 => self.m6,
            0x1c => self.m7,
            0x20 => self.m8,
            0x24 => self.m9,
            0x30 => 0x0000_0400,
            0x34 => self.md,
            0x38 => 0x0000_0400,
            _ => panic!("ARB_CFG read to undefined offset {:x}", off),
        }
    }
    fn write_handler(&mut self, off: usize, val: u32) {
        match off {
            0x00 => self.m0 = val, 
            0x04 => self.m1 = val, 
            0x08 => self.m2 = val, 
            0x0c => self.m3 = val, 
            0x10 => self.m4 = val, 
            0x14 => self.m5 = val, 
            0x18 => self.m6 = val, 
            0x1c => self.m7 = val, 
            0x20 => self.m8 = val, 
            0x24 => self.m9 = val, 
            0x30 => {},
            0x34 => self.md = val, 
            _ => panic!("ARB_CFG write {:08x} to undefined offset {:x}", val, off),
        }
    }
}


/// Unknown interface (probably related to the AHB).
#[derive(Default, Debug, Clone)]
pub struct AhbInterface {
    pub unk_08: u32,
    pub unk_10: u32,
}
impl MmioDevice for AhbInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x08 => 0,
            0x10 => self.unk_10,
            _ => panic!("AHB read to undefined offset {:x}", off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x08 => {
                self.unk_08 = val;
            },
            0x10 => self.unk_10 = val,
            _ => panic!("AHB write {:08x} to undefined offset {:x}", val, off),
        }
        None
    }
}


/// Hollywood memory-mapped registers
pub struct Hollywood {
    pub task: Option<HlwdTask>,

    pub ipc: IpcInterface,
    pub timer: TimerInterface,
    pub busctrl: BusCtrlInterface,
    pub pll: ClockInterface,
    pub otp: otp::OtpInterface,
    pub gpio: gpio::GpioInterface,
    pub irq: irq::IrqInterface,

    pub exi: compat::EXInterface,
    pub di: compat::DriveInterface,
    pub mi: compat::MemInterface,
    pub ahb: AhbInterface,
    pub ddr: ddr::DdrInterface,

    pub arb: ArbCfgInterface,
    pub clocks: u32,
    pub resets: u32,
    pub compat: u32,
    pub spare0: u32,
    pub spare1: u32,

    pub io_str_ctrl0: u32,
    pub io_str_ctrl1: u32,

    pub usb_frc_rst: u32,
}
impl Hollywood {
    pub fn new() -> Self {
        // TODO: Where do the initial values for these registers matter?
        let mut res = Hollywood {
            task: None,
            ipc: IpcInterface::default(),
            busctrl: BusCtrlInterface::default(),
            timer: TimerInterface::default(),
            irq: irq::IrqInterface::default(),
            otp: otp::OtpInterface::new(),
            gpio: gpio::GpioInterface::new(),
            pll: ClockInterface::default(),

            ahb: AhbInterface::default(),
            di: compat::DriveInterface::default(),
            exi: compat::EXInterface::default(),
            mi: compat::MemInterface::new(),
            ddr: ddr::DdrInterface::new(),

            usb_frc_rst: 0,
            arb: ArbCfgInterface::default(),
            resets: 0,
            clocks: 0,
            compat: 0,
            spare0: 0,
            spare1: 0,
            io_str_ctrl0: 0,
            io_str_ctrl1: 0,
        };
        res
    }
}


impl MmioDevice for Hollywood {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x00c           => self.ipc.arm_ctrl,
            0x010           => self.timer.timer,
            0x014           => self.timer.alarm,
            0x030..=0x05c   => self.irq.read_handler(off - 0x30),
            0x060           => self.busctrl.srnprot,
            0x064           => self.busctrl.ahbprot,
            0x070           => self.busctrl.aipprot,
            0x0c0..=0x0d8   => self.gpio.ppc.read_handler(off - 0xc0),
            0x0dc..=0x0fc   => self.gpio.arm.read_handler(off - 0xdc),
            0x100..=0x13c   => self.arb.read_handler(off - 0x100),
            0x180           => self.compat,
            0x188           => self.spare0,
            0x18c           => self.spare1,
            0x190           => self.clocks,
            0x194           => self.resets,
            0x1b0           => 0x0040_11c0, //self.pll.sys,
            0x1b4           => 0x1800_0018, //self.pll.sys_ext,
            0x1bc           => self.pll.ddr,
            0x1c0           => self.pll.ddr_ext,
            0x1c8           => self.pll.vi_ext,
            0x1cc           => self.pll.ai,
            0x1d0           => self.pll.ai_ext,
            0x1d8           => self.pll.usb_ext,
            0x1e0           => self.io_str_ctrl0,
            0x1e4           => self.io_str_ctrl1,
            0x1ec           => self.otp.cmd,
            0x1f0           => self.otp.out,
            0x214           => 0x0000_0000,
            _ => panic!("Unimplemented Hollywood read at {:x}", off),
        };
        //println!("HLWD read {:08x} at off {:03x}", val, off);
        BusPacket::Word(val)
    }

    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x00c => self.ipc.arm_ctrl = val,
            0x014 => {
                println!("HLWD alarm set to {:08x}", val);
                self.timer.alarm = val;
            },
            0x030..=0x05c => self.irq.write_handler(off - 0x30, val),
            0x060 => {
                println!("HLWD SRNPROT={:08x}", val);
                let diff = self.busctrl.srnprot ^ val;
                self.busctrl.srnprot = val;
                let task = if (diff & 0x0000_0020) != 0 {
                    Some(BusTask::SetMirrorEnabled((val & 0x0000_0020) != 0))
                } else {
                    None
                };
                return task;
            }
            0x064 => self.busctrl.ahbprot = val,
            0x070 => self.busctrl.aipprot = val,
            0x088 => self.usb_frc_rst = val,
            0x0c0..=0x0d8 => self.gpio.ppc.write_handler(off - 0xc0, val),
            0x0dc..=0x0fc => {
                self.task = self.gpio.arm.write_handler(off - 0xdc, val);
            },
            0x100..=0x13c => self.arb.write_handler(off - 0x100, val),
            0x180 => self.compat = val,
            0x188 => {

                self.spare0 = val;

                // AHB flushing code seems to check these bits?
                if (val & 0x0001_0000) != 0 {
                    self.spare1 &= 0xffff_fff6;
                } else {
                    self.spare1 |= 0x0000_0009;
                }
            },
            0x18c => {
                println!("HLWD SPARE1={:08x}", val);
                // Potentially toggle the boot ROM mapping
                let diff = self.spare1 ^ val;
                self.spare1 = val;
                let task = if (diff & 0x0000_1000) != 0 {
                    Some(BusTask::SetRomDisabled((val & 0x0000_1000) != 0))
                } else { 
                    None
                };
                return task;
            },
            0x190 => self.clocks = val,
            0x194 => {
                println!("HLWD resets={:08x}", val);
                self.resets = val;
            },
            0x1b0 => self.pll.sys = val,
            0x1b4 => self.pll.sys_ext = val,
            0x1bc => self.pll.ddr = val,
            0x1c0 => self.pll.ddr_ext = val,
            0x1c8 => self.pll.vi_ext = val,
            0x1cc => self.pll.ai = val,
            0x1d0 => self.pll.ai_ext = val,
            0x1d8 => self.pll.usb_ext = val,
            0x1e0 => self.io_str_ctrl0 = val,
            0x1e4 => self.io_str_ctrl1 = val,
            0x1ec => self.otp.write_handler(val),
            _ => panic!("Unimplemented Hollywood write at {:x}", off),
        }
        None
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HlwdTask { GpioOutput(u32) }

impl Bus {
    pub fn handle_step_hlwd(&mut self) {
        let local_ref = self.dev.clone();
        let mut dev = local_ref.write().unwrap();
        let hlwd = &mut dev.hlwd;

        if hlwd.task.is_some() {
            match hlwd.task.unwrap() {
                HlwdTask::GpioOutput(val) => hlwd.gpio.handle_output(val),
            }
            hlwd.task = None;
        }
        hlwd.timer.step();
    }
}


