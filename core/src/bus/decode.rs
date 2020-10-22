
use crate::dev::*;
use crate::bus::*;
use crate::bus::prim::*;

/// Decode physical addresses into some token for a device.
impl Bus {
    pub fn decode_phys_addr(&self, addr: u32) -> Option<DeviceHandle> {
        use IoDevice::*;
        use MemDevice::*;
        let hi_bits = (addr & 0xffff_0000) >> 16;
        match hi_bits {

            0x0d40 |
            0x0d41 |
            0xfff0 |
            0xfff1 |
            0xfffe |
            0xffff => self.resolve_sram(addr),

            0x0d01 => 
                Some(DeviceHandle { dev: Device::Io(Nand), base: NAND_BASE }),
            0x0d02 => 
                Some(DeviceHandle { dev: Device::Io(Aes), base: AES_BASE }),
            0x0d03 => 
                Some(DeviceHandle { dev: Device::Io(Sha), base: SHA_BASE }),

            0x0d80 |
            0x0d8b => self.resolve_hlwd(addr),

            0x0000..=0x017f => 
                Some(DeviceHandle { dev: Device::Mem(Mem1), base: MEM1_BASE }),
            0x1000..=0x13ff => 
                Some(DeviceHandle { dev: Device::Mem(Mem2), base: MEM2_BASE }),

            _ => None,
        }
    }
}

/// Helper functions for decoding physical addresses.
impl Bus {

    /// Read the current state of any arguments which might change the layout
    /// of the physical memory map.
    fn get_physmap_args(&self) -> (bool, bool) {
        (true, false)
    }

    /// Resolve a physical address associated with the SRAM/mask ROM region.
    fn resolve_sram(&self, addr: u32) -> Option<DeviceHandle> {
        let (mrom, mirror) = self.get_physmap_args();
        if mirror {
            self.__resolve_mirror_on(addr, mrom)
        } else {
            self.__resolve_mirror_off(addr, mrom)
        }
    }

    /// Resolve a physical address associated with the Hollywood MMIO region.
    fn resolve_hlwd(&self, addr: u32) -> Option<DeviceHandle> {
        use IoDevice::*;
        match addr {
            HLWD_BASE..=HLWD_TAIL => 
                Some(DeviceHandle { dev: Device::Io(Hlwd), base: HLWD_BASE }),
            DI_BASE..=DI_TAIL =>
                Some(DeviceHandle { dev: Device::Io(Di), base: DI_BASE }),
            AHB_BASE..=AHB_TAIL =>
                Some(DeviceHandle { dev: Device::Io(Ahb), base: AHB_BASE }),
            MEM_BASE..=MEM_TAIL => 
                Some(DeviceHandle { dev: Device::Io(Mi), base: MEM_BASE }),
            DDR_BASE..=DDR_TAIL =>
                Some(DeviceHandle { dev: Device::Io(Ddr), base: DDR_BASE }),
            _ => None,
        }
    }

    /// Mapping for the SRAM region when the mirror is enabled.
    fn __resolve_mirror_on(&self, addr: u32, _mrom: bool) -> Option<DeviceHandle> {
        use MemDevice::*;
        match addr {
            SRAM_BASE_A..=0x0d40_7fff => 
                Some(DeviceHandle { dev: Device::Mem(Sram1), base: SRAM_BASE_A }),
            SRAM_BASE_B..=0x0d41_ffff => 
                Some(DeviceHandle { dev: Device::Mem(Sram0), base: SRAM_BASE_B }),
            SRAM_BASE_C..=0xfff0_7fff => 
                Some(DeviceHandle { dev: Device::Mem(Sram1), base: SRAM_BASE_C }),
            SRAM_BASE_D..=0xfff1_ffff => 
                Some(DeviceHandle { dev: Device::Mem(Sram0), base: SRAM_BASE_D }),
            SRAM_BASE_E..=0xfffe_ffff => 
                Some(DeviceHandle { dev: Device::Mem(Sram1), base: SRAM_BASE_E }),
            SRAM_BASE_F..=0xffff_ffff => 
                Some(DeviceHandle { dev: Device::Mem(Sram0), base: SRAM_BASE_F }),
            _ => None,
        }
    }

    /// Mapping for the SRAM region when the mirror is disabled.
    fn __resolve_mirror_off(&self, addr: u32, mrom: bool) -> Option<DeviceHandle> {
        use MemDevice::*;
        match addr {
            SRAM_BASE_A..=0x0d40_ffff => 
                Some(DeviceHandle { dev: Device::Mem(Sram0), base: SRAM_BASE_A }),
            SRAM_BASE_B..=0x0d41_7fff => 
                Some(DeviceHandle { dev: Device::Mem(Sram1), base: SRAM_BASE_B }),
            SRAM_BASE_C..=0xfff0_ffff => 
                Some(DeviceHandle { dev: Device::Mem(Sram0), base: SRAM_BASE_C }),
            SRAM_BASE_D..=0xfff1_7fff => 
                Some(DeviceHandle { dev: Device::Mem(Sram1), base: SRAM_BASE_D }),
            SRAM_BASE_E..=0xfffe_ffff => 
                Some(DeviceHandle { dev: Device::Mem(Sram0), base: SRAM_BASE_E }),

            SRAM_BASE_F..=0xffff_7fff => { if mrom && addr <= MROM_TAIL {
                Some(DeviceHandle { dev: Device::Mem(MaskRom), base: MROM_BASE })
            } else {
                Some(DeviceHandle { dev: Device::Mem(Sram1), base: SRAM_BASE_F })
            }},
            _ => None,
        }
    }
}
