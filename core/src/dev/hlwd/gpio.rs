
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
    pub fn write_handler(&mut self, off: usize, data: u32) {
        match off {
            0x00 => self.en = data,
            0x04 => self.output = data,
            0x08 => self.dir = data,
            0x10 => self.input = data,
            0x18 => self.intmask = data,
            0x20 => self.owner = data,
            _ => panic!("unimplemented ArmGpio write {:08x}", off),
        }
    }
    pub fn read_handler(&self, off: usize) -> u32 {
        match off {
            0x00 => self.en,
            0x04 => self.output,
            0x08 => self.dir,
            0x10 => self.input,
            0x18 => self.intmask,
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

/// Top-level container for GPIO pin state.
#[derive(Default, Debug, Clone)]
pub struct GpioInterface {
    pub arm: ArmGpio,
    pub ppc: PpcGpio,
}


