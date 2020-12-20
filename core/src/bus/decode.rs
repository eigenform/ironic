
use crate::dev::*;
use crate::bus::*;
use crate::bus::prim::*;

/// Declare a constant handle to some memory device.
macro_rules! decl_mem_handle { 
    ($name:ident, $id:ident, $mask:expr) => {
        const $name : DeviceHandle = DeviceHandle {
            dev: Device::Mem(MemDevice::$id), mask: $mask
        };
    }
}

/// Declare a constant handle to some IO device.
macro_rules! decl_io_handle { 
    ($name:ident, $id:ident, $mask:expr) => {
        const $name : DeviceHandle = DeviceHandle {
            dev: Device::Io(IoDevice::$id), mask: $mask
        };
    }
}

// These are declarations of all the constant DeviceHandle structures whose 
// parameters (base address, size, etc.) will never change during runtime.

decl_mem_handle!(MEM1_HANDLE, Mem1, 0x017f_ffff);
decl_mem_handle!(MEM2_HANDLE, Mem2, 0x03ff_ffff);

decl_io_handle!(NAND_HANDLE, Nand,  0x0000_001f);
decl_io_handle!(AES_HANDLE, Aes,    0x0000_001f);
decl_io_handle!(SHA_HANDLE, Sha,    0x0000_001f);
decl_io_handle!(EHCI_HANDLE, Ehci,  0x0000_00ff);

decl_io_handle!(HLWD_HANDLE, Hlwd,  0x0000_03ff);
decl_io_handle!(AHB_HANDLE, Ahb,    0x0000_3fff);
decl_io_handle!(MI_HANDLE, Mi,      0x0000_01ff);
decl_io_handle!(DDR_HANDLE, Ddr,    0x0000_01ff);
decl_io_handle!(DI_HANDLE, Di,      0x0000_03ff);
//decl_io_handle!(SI_HANDLE, Si,      0x0000_03ff);
//decl_io_handle!(EXI_HANDLE, Exi,    0x0000_03ff);


impl Bus {
    /// Decode a physical address into some handle for a particlar device.
    pub fn decode_phys_addr(&self, addr: u32) -> Option<DeviceHandle> {
        let hi_bits = (addr & 0xffff_0000) >> 16;
        match hi_bits {
            0x0d40 |
            0x0d41 |
            0xfff0 |
            0xfff1 |
            0xfffe |
            0xffff => self.resolve_sram(addr),

            0x0d01 => Some(NAND_HANDLE),
            0x0d02 => Some(AES_HANDLE),
            0x0d03 => Some(SHA_HANDLE),
            0x0d04 => Some(EHCI_HANDLE),

            0x0d80 |
            0x0d8b => self.resolve_hlwd(addr),

            0x0000..=0x017f => Some(MEM1_HANDLE),
            0x1000..=0x13ff => Some(MEM2_HANDLE),

            _ => None,
        }
    }
}

/// These are helper functions for decoding physical addresses.
impl Bus {
    /// Resolve a physical address associated with the Hollywood MMIO region.
    fn resolve_hlwd(&self, addr: u32) -> Option<DeviceHandle> {
        match addr {
            HLWD_BASE..=HLWD_TAIL   => Some(HLWD_HANDLE),
            DI_BASE..=DI_TAIL       => Some(DI_HANDLE),
            AHB_BASE..=AHB_TAIL     => Some(AHB_HANDLE),
            MEM_BASE..=MEM_TAIL     => Some(MI_HANDLE),
            DDR_BASE..=DDR_TAIL     => Some(DDR_HANDLE),
            _ => None,
        }
    }

    /// Resolve a physical address associated with SRAM or the mask ROM.
    fn resolve_sram(&self, addr: u32) -> Option<DeviceHandle> {
        match (!self.rom_disabled, self.mirror_enabled) {
            (true,  false) => resolve_rom_nomir(addr),
            (true,  true)  => resolve_rom_mir(addr),
            (false, true)  => resolve_norom_mir(addr),
            (false, false) => resolve_norom_nomir(addr),
        }
    }
}

fn resolve_rom_nomir(addr: u32) -> Option<DeviceHandle> {
    use MemDevice::*;
    match addr {
        0x0d40_0000..=0x0d40_ffff | 0xfff0_0000..=0xfff0_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram0), mask: 0x0000_ffff }),

        // The top half of this is just garbage?
        0x0d41_0000..=0x0d41_ffff | 0xfff1_0000..=0xfff1_ffff =>
            Some(DeviceHandle { dev: Device::Mem(Sram1), mask: 0x0000_ffff }),

        0xfffe_0000..=0xfffe_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram0), mask: 0x0000_ffff }),
        0xffff_0000..=0xffff_1fff => 
            Some(DeviceHandle { dev: Device::Mem(MaskRom), mask: 0x0000_1fff }),
        _ => None,
    }
}

fn resolve_rom_mir(addr: u32) -> Option<DeviceHandle> {
    use MemDevice::*;
    match addr {
        0x0d40_0000..=0x0d41_7fff | 0xfff0_0000..=0xfff1_ffff =>
            Some(DeviceHandle { dev: Device::Mem(MaskRom), mask: 0x0000_1fff }),
        0xfffe_0000..=0xfffe_ffff => 
            Some(DeviceHandle { dev: Device::Mem(MaskRom), mask: 0x0000_1fff }),
        0xffff_0000..=0xffff_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram0), mask: 0x0000_ffff }),
        _ => None,
    }
}

fn resolve_norom_mir(addr: u32) -> Option<DeviceHandle> {
    use MemDevice::*;
    match addr {

        // Top half is garbage?
        0x0d40_0000..=0x0d40_ffff | 0xfff0_0000..=0xfff0_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram1), mask: 0x0000_ffff }),

        0x0d41_0000..=0x0d41_ffff | 0xfff1_0000..=0xfff1_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram1), mask: 0x0000_ffff }),

        // Top half is garbage?
        0xfffe_0000..=0xfffe_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram1), mask: 0x0000_ffff }),

        0xffff_0000..=0xffff_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram0), mask: 0x0000_ffff }),
        _ => None,
    }
}

fn resolve_norom_nomir(addr: u32) -> Option<DeviceHandle> {
    use MemDevice::*;
    match addr {

        0x0d40_0000..=0x0d40_ffff | 0xfff0_0000..=0xfff0_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram0), mask: 0x0000_ffff }),

        // Top half is garbage?
        0x0d41_0000..=0x0d41_ffff | 0xfff1_0000..=0xfff1_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram1), mask: 0x0000_ffff }),

        0xfffe_0000..=0xfffe_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram0), mask: 0x0000_ffff }),

        // Top half is garbage?
        0xffff_0000..=0xffff_ffff => 
            Some(DeviceHandle { dev: Device::Mem(Sram1), mask: 0x0000_ffff }),
        _ => None,
    }
}

