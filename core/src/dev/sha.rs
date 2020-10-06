
pub mod util;

use crate::bus::*;
use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;
use std::sync::{Arc, RwLock};

pub enum ShaCommand {
    ComputeHash,
}

/// Representing the SHA interface.
pub struct ShaInterface {
    ctrl: u32,
    src: u32,

    /// The internal state of the SHA-1 engine.
    state: util::Sha1State,
}
impl ShaInterface {
    pub fn new() -> Self {
        ShaInterface {
            state: util::Sha1State::new(),
            ctrl: 0,
            src: 0,
        }
    }
    /// Reset the state of the SHA interface.
    fn reset(&mut self) {
        self.ctrl = 0;
        self.src = 0;
        self.state.digest[0] = 0;
        self.state.digest[1] = 0;
        self.state.digest[2] = 0;
        self.state.digest[3] = 0;
        self.state.digest[4] = 0;
    }
}

impl MmioDevice for ShaInterface {
    type Width = u32;

    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x00 => self.ctrl,
            0x08 => self.state.digest[0],
            0x0c => self.state.digest[1],
            0x10 => self.state.digest[2],
            0x14 => self.state.digest[3],
            0x18 => self.state.digest[4],
            _ => panic!("Unimplemented SHA read at offset {:04x}", off),
        };
        BusPacket::Word(val)
    }

    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            0x00 => {
                self.ctrl = val;
                if (val & 0x8000_0000) != 0 {
                    return Some(BusTask::Sha(val));
                }
            },
            0x04 => self.src = val,
            0x08 => self.state.digest[0] = val,
            0x0c => self.state.digest[1] = val,
            0x10 => self.state.digest[2] = val,
            0x14 => self.state.digest[3] = val,
            0x18 => self.state.digest[4] = val,
            _ => panic!("Unhandled write32 to {:08x}", off),
        }
        None
    }
}

impl Bus {
    pub fn handle_task_sha(&mut self, val: u32) {
        panic!("SHA task handler unimplemented");
    }
}


