pub mod prim;
pub mod decode;
pub mod dispatch;
pub mod mmio;
pub mod task;

use crate::topo;
use crate::bus::task::*;

use std::sync::{Arc,RwLock};
pub type MemRef = Arc<RwLock<topo::SystemMemory>>;
pub type DevRef = Arc<RwLock<topo::SystemDevice>>;

/// Implementation of an emulated bus.
pub struct Bus {
    /// Reference to [SystemMemory].
    pub mem: MemRef,
    /// Reference to [SystemDevice].
    pub dev: DevRef,

    /// True when the ROM mapping is disabled.
    pub rom_disabled: bool,
    /// True when the SRAM mirror is enabled.
    pub mirror_enabled: bool,

    /// Queue for pending work on I/O devices.
    pub tasks: Vec<Task>,
    pub cycle: usize,
}
impl Bus {
    pub fn new(mem: MemRef, dev: DevRef)-> Self {
        Bus { 
            mem, dev,
            rom_disabled: false,
            mirror_enabled: false,
            tasks: Vec::new(),
            cycle: 0,
        }
    }
}

