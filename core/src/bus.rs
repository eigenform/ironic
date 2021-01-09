pub mod prim;
pub mod decode;
pub mod dispatch;
pub mod mmio;
pub mod task;
use crate::bus::task::*;

use crate::mem::*;
use crate::dev::hlwd::*;
use crate::dev::aes::*;
use crate::dev::sha::*;
use crate::dev::nand::*;
use crate::dev::ehci::*;
use crate::dev::ohci::*;
use crate::dev::sdhc::*;


/// Implementation of an emulated bus.
///
/// In this model, the bus itself owns all memories and system devices.
pub struct Bus {
    // System memories
    pub mrom: BigEndianMemory,
    pub sram0: BigEndianMemory,
    pub sram1: BigEndianMemory,
    pub mem1: BigEndianMemory,
    pub mem2: BigEndianMemory,

    // System devices
    pub hlwd: Hollywood,
    pub nand: NandInterface,
    pub aes: AesInterface,
    pub sha: ShaInterface,
    pub ehci: EhcInterface,
    pub ohci0: OhcInterface,
    pub ohci1: OhcInterface,
    pub sd0: SDInterface,
    pub sd1: WLANInterface,

    /// True when the ROM mapping is disabled.
    pub rom_disabled: bool,
    /// True when the SRAM mirror is enabled.
    pub mirror_enabled: bool,

    /// Queue for pending work on I/O devices.
    pub tasks: Vec<Task>,
    pub cycle: usize,
}
impl Bus {
    pub fn new()-> Self {
        Bus { 
            mrom: BigEndianMemory::new(0x0000_2000, Some("./boot0.bin")),
            sram0: BigEndianMemory::new(0x0001_0000, None),
            sram1: BigEndianMemory::new(0x0001_0000, None),
            mem1: BigEndianMemory::new(0x0180_0000, None),
            mem2: BigEndianMemory::new(0x0400_0000, None),

            hlwd: Hollywood::new(),
            nand: NandInterface::new("./nand.bin"),
            aes: AesInterface::new(),
            sha: ShaInterface::new(),
            ehci: EhcInterface::new(),
            ohci0: OhcInterface { idx: 0, ..Default::default() },
            ohci1: OhcInterface { idx: 1, ..Default::default() },
            sd0: SDInterface::default(),
            sd1: WLANInterface::default(),

            rom_disabled: false,
            mirror_enabled: false,
            tasks: Vec::new(),
            cycle: 0,
        }
    }
}

