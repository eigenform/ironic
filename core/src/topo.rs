//! Implementation of a physical memory map.

use std::sync::{Arc,RwLock};

use crate::bus::*;
use crate::hlwd::*;
use crate::mem::*;
use crate::dbg::*;

// Sizes of physical memory devices.
pub const MEM1_SIZE:    u32 = 0x0180_0000;
pub const MEM2_SIZE:    u32 = 0x0400_0000;
pub const MROM_SIZE:    u32 = 0x0000_2000;
pub const SRM0_SIZE:    u32 = 0x0001_0000;
pub const SRM1_SIZE:    u32 = 0x0001_0000;
pub const COREDEV_SIZE: u32 = 0x0000_0020;
pub const IODEV_SIZE:   u32 = 0x0000_0200;
pub const HLWDEV_SIZE:  u32 = 0x0000_0400;
pub const MEMDEV_SIZE:  u32 = 0x0000_0200;
pub const AHB_SIZE:     u32 = 0x0000_4000;

// Base addresses for physical memory devices.
pub const MEM1_BASE:    u32 = 0x0000_0000;
pub const MEM2_BASE:    u32 = 0x1000_0000;
pub const NAND_BASE:    u32 = 0x0d01_0000;
pub const AES_BASE:     u32 = 0x0d02_0000;
pub const SHA_BASE:     u32 = 0x0d03_0000;
pub const EHCI_BASE:    u32 = 0x0d04_0000;
pub const OH0_BASE:     u32 = 0x0d05_0000;
pub const OH1_BASE:     u32 = 0x0d06_0000;
pub const SD0_BASE:     u32 = 0x0d07_0000;
pub const SD1_BASE:     u32 = 0x0d08_0000;
pub const HLWD_BASE:    u32 = 0x0d80_0000;
pub const DI_BASE:      u32 = 0x0d80_6000;
pub const SI_BASE:      u32 = 0x0d80_6400;
pub const EXI_BASE:     u32 = 0x0d80_6800;
pub const AHB_BASE:     u32 = 0x0d8b_0000;
pub const MEM_BASE:     u32 = 0x0d8b_4000;
pub const DDR_BASE:     u32 = 0x0d8b_4200;
pub const SRAM_BASE_A:  u32 = 0x0d40_0000;
pub const SRAM_BASE_B:  u32 = 0x0d41_0000;
pub const SRAM_BASE_C:  u32 = 0xfff0_0000;
pub const SRAM_BASE_D:  u32 = 0xfff1_0000;
pub const SRAM_BASE_E:  u32 = 0xfffe_0000;
pub const SRAM_BASE_F:  u32 = 0xffff_0000;
pub const MROM_BASE:    u32 = 0xffff_0000;

// Tail addresses for physical memory devices.
pub const MEM1_TAIL:    u32 = MEM1_BASE + MEM1_SIZE - 1;
pub const MEM2_TAIL:    u32 = MEM2_BASE + MEM2_SIZE - 1;
pub const NAND_TAIL:    u32 = NAND_BASE + COREDEV_SIZE - 1;
pub const AES_TAIL:     u32 = AES_BASE + COREDEV_SIZE - 1;
pub const SHA_TAIL:     u32 = SHA_BASE + COREDEV_SIZE - 1;
pub const EHCI_TAIL:    u32 = EHCI_BASE + IODEV_SIZE - 1;
pub const OH0_TAIL:     u32 = OH0_BASE + IODEV_SIZE - 1;
pub const OH1_TAIL:     u32 = OH1_BASE + IODEV_SIZE - 1;
pub const SD0_TAIL:     u32 = SD0_BASE + IODEV_SIZE - 1;
pub const SD1_TAIL:     u32 = SD1_BASE + IODEV_SIZE - 1;
pub const HLWD_TAIL:    u32 = HLWD_BASE + HLWDEV_SIZE - 1;
pub const DI_TAIL:      u32 = DI_BASE + HLWDEV_SIZE - 1;
pub const SI_TAIL:      u32 = SI_BASE + HLWDEV_SIZE - 1;
pub const EXI_TAIL:     u32 = EXI_BASE + HLWDEV_SIZE - 1;
pub const AHB_TAIL:     u32 = AHB_BASE + AHB_SIZE - 1;
pub const MEM_TAIL:     u32 = MEM_BASE + MEMDEV_SIZE - 1;
pub const DDR_TAIL:     u32 = DDR_BASE + MEMDEV_SIZE - 1;
pub const MROM_TAIL:    u32 = MROM_BASE + MROM_SIZE - 1;



/// A unique identifier for each physical memory device.
#[derive(Debug, Clone, Copy)]
pub enum PhysMemDevice {
    MaskRom,
    Sram0,
    Sram1,
    Mem1,
    Mem2,
    Hlwd,
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

    pub hlwd: Box<Hollywood>,
    pub sram_mirror: bool,
    pub mrom_mapped: bool,
}
impl Topology {
    pub fn new(dbg: Arc<RwLock<Debugger>>, rom: &str) -> Self {
        Topology {
            mrom: Box::new(BigEndianMemory::new(0x0000_2000, Some(rom))),
            sram0: Box::new(BigEndianMemory::new(0x0001_0000, None)),
            sram1: Box::new(BigEndianMemory::new(0x0000_8000, None)),
            hlwd: Box::new(Hollywood::new()),
            sram_mirror: false,
            mrom_mapped: true,
            dbg,
        }
    }
}

/// Helper functions for resolving the target of a physical memory access.
impl Topology {

    fn get_physmap_args(&self) -> (bool, bool) {
        (self.mrom_mapped, self.sram_mirror)
    }

    #[inline(always)]
    fn resolve_sram(&self, addr: u32) -> Option<PhysMemHandle> {
        use PhysMemDevice::*;
        let (mrom_on, swap_on) = self.get_physmap_args();
        if swap_on {
            match addr {
                SRAM_BASE_A..=0x0d40_7fff => 
                    Some(PhysMemHandle{ id: Sram1, base: SRAM_BASE_A }),
                SRAM_BASE_B..=0x0d41_ffff =>
                    Some(PhysMemHandle{ id: Sram0, base: SRAM_BASE_B }),
                SRAM_BASE_C..=0xfff0_7fff =>
                    Some(PhysMemHandle{ id: Sram1, base: SRAM_BASE_C }),
                SRAM_BASE_D..=0xfff1_ffff =>
                    Some(PhysMemHandle{ id: Sram0, base: SRAM_BASE_D }),
                SRAM_BASE_E..=0xfffe_ffff =>
                    Some(PhysMemHandle{ id: Sram1, base: SRAM_BASE_E }),
                SRAM_BASE_F..=0xffff_ffff =>
                    Some(PhysMemHandle{ id: Sram0, base: SRAM_BASE_F }),
                _ => None,
            }
        }
        else {
            match addr {
                SRAM_BASE_A..=0x0d40_ffff =>
                    Some(PhysMemHandle{ id: Sram0, base: SRAM_BASE_A }),
                SRAM_BASE_B..=0x0d41_7fff =>
                    Some(PhysMemHandle{ id: Sram1, base: SRAM_BASE_B }),
                SRAM_BASE_C..=0xfff0_ffff =>
                    Some(PhysMemHandle{ id: Sram0, base: SRAM_BASE_C }),
                SRAM_BASE_D..=0xfff1_7fff =>
                    Some(PhysMemHandle{ id: Sram1, base: SRAM_BASE_D }),
                SRAM_BASE_E..=0xfffe_ffff =>
                    Some(PhysMemHandle{ id: Sram0, base: SRAM_BASE_E }),

                // This is probably incorrect behavior; fine for now
                SRAM_BASE_F..=0xffff_7fff => {
                    if mrom_on && addr <= MROM_TAIL {
                        Some(PhysMemHandle{ id: MaskRom, base: MROM_BASE })
                    }
                    else {
                        Some(PhysMemHandle{ id: Sram1, base: SRAM_BASE_F })
                    }
                },
                _ => None,
            }
        }
    }

    #[inline(always)]
    fn resolve_hlwd(&self, addr: u32) -> Option<PhysMemHandle> {
        use PhysMemDevice::*;
        match addr {
            HLWD_BASE..=HLWD_TAIL => 
                Some(PhysMemHandle{ id: Hlwd, base: HLWD_BASE }),
            _ => None,
        }
    }
}




impl PhysMemMap for Topology {}
impl PhysMemDecode for Topology {
    type Addr = u32;
    type Handle = PhysMemHandle;

    fn decode_phys_addr(&self, addr: u32) -> Option<PhysMemHandle> {
        use PhysMemDevice::*;
        let hi_bits = (addr & 0xffff_0000) >> 16;
        match hi_bits {
            0x0d40  | 
            0x0d41  |
            0xfff0  | 
            0xfff1  | 
            0xfffe  | 
            0xffff  => self.resolve_sram(addr),

            0x0d80  => self.resolve_hlwd(addr),

            0x0000..=0x017f => 
                Some(PhysMemHandle { id: Mem1, base: MEM1_BASE }),
            0x1000..=0x13ff => 
                Some(PhysMemHandle { id: Mem2, base: MEM2_BASE }),
            _ => {
                log(&self.dbg, LogLevel::Bus, &format!(
                    "Couldn't resolve physical address {:08x}", addr));
                panic!("Couldn't resolve physical address {:08x}", addr);
            },
        }
    }

    fn _read32(&mut self, handle: PhysMemHandle, addr: u32) -> u32 { 
        use PhysMemDevice::*;
        let off = (addr - handle.base) as usize;
        match handle.id {
            MaskRom => self.mrom.read::<u32>(off),
            Sram0 => self.sram0.read::<u32>(off),
            Sram1 => self.sram1.read::<u32>(off),
            Hlwd => self.hlwd.read_handler(off),
            _ => panic!("32-bit reads are unimplemented on {:?}", handle.id),
        }
    }
    fn _read16(&mut self, handle: PhysMemHandle, addr: u32) -> u16 { 
        use PhysMemDevice::*;
        let off = (addr - handle.base) as usize;
        match handle.id {
            MaskRom => self.mrom.read::<u16>(off),
            Sram0 => self.sram0.read::<u16>(off),
            Sram1 => self.sram1.read::<u16>(off),
            _ => panic!("16-bit reads are unimplemented on {:?}", handle.id),
        }
    }
    fn _read8(&mut self, handle: PhysMemHandle, addr: u32) -> u8 { 
        use PhysMemDevice::*;
        let off = (addr - handle.base) as usize;
        match handle.id {
            MaskRom => self.mrom.read::<u8>(off),
            Sram0 => self.sram0.read::<u8>(off),
            Sram1 => self.sram1.read::<u8>(off),
            _ => panic!("8-bit reads are unimplemented on {:?}", handle.id),
        }
    }

    fn _write32(&mut self, handle: PhysMemHandle, addr: u32, val: u32) { 
        use PhysMemDevice::*;
        let off = (addr - handle.base) as usize;
        match handle.id {
            Sram0 => self.sram0.write::<u32>(off, val),
            Sram1 => self.sram1.write::<u32>(off, val),
            Hlwd => self.hlwd.write_handler(off, val),
            _ => panic!("32-bit writes are unimplemented on {:?}", handle.id),
        }
    }
    fn _write16(&mut self, handle: PhysMemHandle, addr: u32, val: u16) { 
        use PhysMemDevice::*;
        let off = (addr - handle.base) as usize;
        match handle.id {
            Sram0 => self.sram0.write::<u16>(off, val),
            Sram1 => self.sram1.write::<u16>(off, val),
            _ => panic!("16-bit writes are unimplemented on {:?}", handle.id),
        }
    }
    fn _write8(&mut self, handle: PhysMemHandle, addr: u32, val: u8) { 
        use PhysMemDevice::*;
        let off = (addr - handle.base) as usize;
        match handle.id {
            Sram0 => self.sram0.write::<u8>(off, val),
            Sram1 => self.sram1.write::<u8>(off, val),
            _ => panic!("8-bit writes are unimplemented on {:?}", handle.id),
        }
    }
}


