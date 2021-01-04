
/// Hollywood platform interface.
pub mod hlwd;
/// The NAND flash interface.
pub mod nand;
/// The AES engine interface.
pub mod aes;
/// The SHA engine interface.
pub mod sha;

/// USB Enhanced Host Controller interface.
pub mod ehci;
/// USB Open Host Controller interface.
pub mod ohci;
/// SD Host Controller interface.
pub mod sdhc;

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
pub const MEM1_MASK:    u32 = 0x017f_ffff;

pub const MEM2_BASE:    u32 = 0x1000_0000;
pub const MEM2_MASK:    u32 = 0x03ff_ffff;

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
pub const MROM_MASK:    u32 = 0x0000_1fff;

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

