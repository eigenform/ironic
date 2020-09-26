

/// Various bus control registers (?)
#[derive(Default, Debug, Clone)]
pub struct BusCtrlInterface {
    pub srnprot: u32,
    pub ahbprot: u32,
}
impl BusCtrlInterface {
    pub fn sram_mirror(&self) -> bool { (self.srnprot & 0x0000_0020) != 0 }
}

/// Hollywood memory-mapped registers
pub struct Hollywood {
    pub busctrl: BusCtrlInterface,
}
impl Hollywood {
    pub fn new() -> Self {
        Hollywood {
            busctrl: BusCtrlInterface::default(),
        }
    }
}

impl Hollywood {
    pub fn write_handler(&mut self, off: usize, val: u32) {
        match off {
            0x60 => self.busctrl.srnprot = val,
            _ => panic!("Unimplemented Hollywood write at {:x}", off),
        }
    }
    pub fn read_handler(&mut self, off: usize) -> u32 {
        match off {
            0x60 => self.busctrl.srnprot,
            _ => panic!("Unimplemented Hollywood write at {:x}", off),
        }
    }
}






