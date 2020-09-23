//! Implementation of a physical memory map.

use std::sync::{Arc,RwLock};
use crate::bus::*;
use crate::mem::back::*;
use crate::dbg::*;

/// A unique identifier for each physical memory device.
#[derive(Debug, Clone, Copy)]
pub enum PhysMemDevice {
    MaskRom,
}

/// A handle used to dispatch a physical memory access.
#[derive(Debug, Clone, Copy)]
pub struct PhysMemHandle {
    id: PhysMemDevice,
    base: u32,
}


/// Top-level container for emulated devices.
pub struct Topology {
    pub dbg: Arc<RwLock<Debugger>>,

    pub mrom: Box<BigEndianMemory>,
    pub sram0: Box<BigEndianMemory>,
    pub sram1: Box<BigEndianMemory>,
    pub sram_mirror: bool,
    pub mrom_mapped: bool,
}
impl Topology {
    pub fn new(dbg: Arc<RwLock<Debugger>>, rom: &str) -> Self {
        Topology {
            mrom: Box::new(BigEndianMemory::new(0x0000_2000, Some(rom))),
            sram0: Box::new(BigEndianMemory::new(0x0001_0000, None)),
            sram1: Box::new(BigEndianMemory::new(0x0000_8000, None)),
            sram_mirror: false,
            mrom_mapped: true,
            dbg,
        }
    }
}


impl PhysMemMap for Topology {}
impl PhysMemDecode for Topology {
    type Addr = u32;
    type Handle = PhysMemHandle;

    fn decode_phys_addr(&self, addr: u32) -> Option<PhysMemHandle> {
        match addr {
            0xffff_0000..=0xffff_1fff => {
                Some(PhysMemHandle {
                    id: PhysMemDevice::MaskRom, base: 0xffff_0000
                })
            },
            _ => panic!("Couldn't resolve physical address {:08x}", addr),
        }
    }

    fn _read32(&mut self, handle: PhysMemHandle, addr: u32) -> u32 { 
        use PhysMemDevice::*;
        let off = (addr - handle.base) as usize;
        match handle.id {
            MaskRom => self.mrom.read::<u32>(off),
        }
    }
    fn _read16(&mut self, handle: PhysMemHandle, addr: u32) -> u16 { 
        use PhysMemDevice::*;
        let off = (addr - handle.base) as usize;
        match handle.id {
            MaskRom => self.mrom.read::<u16>(off),
        }
    }
    fn _read8(&mut self, handle: PhysMemHandle, addr: u32) -> u8 { 
        use PhysMemDevice::*;
        let off = (addr - handle.base) as usize;
        match handle.id {
            MaskRom => self.mrom.read::<u8>(off),
        }
    }

    fn _write32(&mut self, handle: PhysMemHandle, addr: u32, val: u32) { 
        panic!("Physical writes unimplemented");
    }
    fn _write16(&mut self, handle: PhysMemHandle, addr: u32, val: u16) { 
        panic!("Physical writes unimplemented");
    }
    fn _write8(&mut self, handle: PhysMemHandle, addr: u32, val: u8) { 
        panic!("Physical writes unimplemented");
    }
}


