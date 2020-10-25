
pub mod seeprom;
use crate::dev::hlwd::gpio::seeprom::*;

use crate::dev::hlwd::*;
use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;


#[repr(u32)]
pub enum GpioPin {
    Power       = 0x0000_0001,
    Shutdown    = 0x0000_0002,
    Fan         = 0x0000_0004,
    Dcdc        = 0x0000_0008,
    DiSpin      = 0x0000_0010,
    SlotLed     = 0x0000_0020,
    EjectButton = 0x0000_0040,
    SlotIn      = 0x0000_0080,
    SensorBar   = 0x0000_0100,
    DoEject     = 0x0000_0200,
    SeepromCs   = 0x0000_0400,
    SeepromClk  = 0x0000_0800,
    SeepromMosi = 0x0000_1000,
    SeepromMiso = 0x0000_2000,
    AveScl      = 0x0000_4000,
    AveSda      = 0x0000_8000,
}


/// Top-level container for GPIO pin state.
pub struct GpioInterface {
    pub arm: ArmGpio,
    pub ppc: PpcGpio,

    pub seeprom: SeepromState,
}
impl GpioInterface {
    pub fn new() -> Self {
        GpioInterface {
            arm: ArmGpio::default(),
            ppc: PpcGpio::default(),
            seeprom: SeepromState::new(),
        }
    }
}

impl GpioInterface {
    pub fn handle_output(&mut self, val: u32) {
        let diff = self.arm.output ^ val;
        if (diff & 0x0000_1c00) != 0 {
            self.handle_seeprom(val);
        } else if (diff & 0x00ff_0000) != 0 {
            println!("GPIO DEBUG pins [{:02x}]", (val & 0x00ff_0000) >> 16);
        }
    }
}



/// ARM-facing GPIO pin state.
#[derive(Default, Debug, Clone)]
pub struct ArmGpio {
    en: u32,
    output: u32,
    dir: u32,
    input: u32,
    intlvl: u32,
    intflag: u32,
    intmask: u32,
    straps: u32,
    owner: u32,
}
impl ArmGpio {
    pub fn write_handler(&mut self, off: usize, data: u32) -> Option<HlwdTask> {
        match off {
            0x00 => self.en = data,
            0x04 => { 
                let diff = self.output ^ data;
                let task = if (self.output ^ data) != 0 {
                    Some(HlwdTask::GpioOutput(data))
                } else { 
                    None 
                };
                return task;
            },
            0x08 => self.dir = data,
            0x0c => panic!("CPU wrote to GPIO inputs!?"),
            0x10 => self.intlvl = data,
            0x14 => self.intflag = data,
            0x18 => self.intmask = data,
            0x1c => self.straps = data,
            0x20 => self.owner = data,
            _ => panic!("unimplemented ArmGpio write {:08x}", off),
        }
        None
    }
    pub fn read_handler(&self, off: usize) -> u32 {
        match off {
            0x00 => self.en,
            0x04 => self.output,
            0x08 => self.dir,
            0x0c => self.input,
            0x10 => self.intlvl,
            0x14 => self.intflag,
            0x18 => self.intmask,
            0x1c => self.straps,
            0x20 => self.owner,
            _ => panic!("unimplemented ArmGpio read {:08x}", off),
        }
    }
}

/// PowerPC-facing GPIO pin state.
#[derive(Default, Debug, Clone)]
pub struct PpcGpio {
    output: u32,
    dir: u32,
    input: u32,
    intlvl: u32,
    intflag: u32,
    intmask: u32,
    straps: u32,
}
impl PpcGpio {
    pub fn write_handler(&mut self, off: usize, data: u32) {
        match off {
            0x00 => self.output = data,
            0x04 => self.dir = data,
            _ => panic!("unimplemented PpcGpio write {:08x}", off),
        }
    }
    pub fn read_handler(&self, off: usize) -> u32 {
        match off {
            0x00 => self.output,
            0x04 => self.dir,
            _ => panic!("unimplemented PpcGpio read {:08x}", off),
        }
    }
}



