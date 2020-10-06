

use crate::bus::*;
use crate::bus::prim::*;
use std::sync::{Arc, RwLock};

pub struct Mmu {
    pub bus: Arc<RwLock<Bus>>,
}
impl Mmu {
    pub fn new(bus: Arc<RwLock<Bus>>) -> Self {
        Mmu { bus }
    }
}

impl Mmu {
    pub fn read32(&self, addr: u32) -> u32 {
        self.bus.write().unwrap().read32(addr)
    }
    pub fn read16(&self, addr: u32) -> u16 {
        self.bus.write().unwrap().read16(addr)
    }
    pub fn read8(&self, addr: u32) -> u8 {
        self.bus.write().unwrap().read8(addr)
    }

    pub fn write32(&self, addr: u32, val: u32) {
        self.bus.write().unwrap().write32(addr, val);
    }
    pub fn write16(&self, addr: u32, val: u16) {
        self.bus.write().unwrap().write16(addr, val);
    }
    pub fn write8(&self, addr: u32, val: u8) {
        self.bus.write().unwrap().write8(addr, val);
    }
}
