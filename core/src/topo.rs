

use std::sync::{Arc,RwLock};
use crate::dbg::*;

use crate::mem::*;
use crate::dev::hlwd::*;
use crate::dev::aes::*;
use crate::dev::sha::*;
use crate::dev::nand::*;


/// Top-level container for system memories.
///
/// This structure owns all of the references to memory devices.
pub struct SystemMemory {
    pub mrom: BigEndianMemory,
    pub sram0: BigEndianMemory,
    pub sram1: BigEndianMemory,
    pub mem1: BigEndianMemory,
    pub mem2: BigEndianMemory,
}
impl SystemMemory {
    pub fn new() -> Self {
        SystemMemory {
            mrom: BigEndianMemory::new(0x0000_2000, Some("./boot0.bin")),
            sram0: BigEndianMemory::new(0x0001_0000, None),
            sram1: BigEndianMemory::new(0x0000_8000, None),
            mem1: BigEndianMemory::new(0x0180_0000, None),
            mem2: BigEndianMemory::new(0x0400_0000, None),
        }
    }
}

/// Top-level container for system I/O devices.
///
/// This structure owns all of the references to I/O devices.
pub struct SystemDevice {
    pub nand: NandInterface,
    pub aes: AesInterface,
    pub sha: ShaInterface,
    pub hlwd: Hollywood,
}
impl SystemDevice {
    pub fn new(dbg: Arc<RwLock<Debugger>>) -> Self {
        SystemDevice {
            aes: AesInterface::new(),
            sha: ShaInterface::new(),
            nand: NandInterface::new(dbg.clone(), "./nand.bin"),
            hlwd: Hollywood::new(dbg.clone()),
        }
    }
}



