
use std::sync::{Arc,RwLock};
//use std::thread;
//use std::sync::mpsc::{channel, Sender, Receiver};
use crate::dbg::*;

use crate::mem::*;
use crate::bus::*;
use crate::cpu::*;
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

/// Context/state associated with some thread responsible for CPU emulation.
pub struct EmuThreadContext {
    pub bus: Arc<RwLock<Bus>>,
    pub dbg: Arc<RwLock<Debugger>>,
    pub cpu: Cpu,
}
impl EmuThreadContext {
    pub fn new(dbg: Arc<RwLock<Debugger>>, bus: Arc<RwLock<Bus>>) -> Self {
        let cpu = Cpu::new(dbg.clone(), bus.clone());
        EmuThreadContext { bus: bus.clone(), dbg: dbg.clone(), cpu }
    }
}

impl EmuThreadContext {

    /// Run the emulator thread for some number of steps (currently, Bus steps
    /// are interleaved with CPU steps).
    pub fn run_slice(&mut self, num_steps: usize) {
        for _i in 0..num_steps {
            let res = self.cpu.step();
            match res {
                CpuRes::StepOk => { self.bus.write().unwrap().step(); },
                CpuRes::HaltEmulation => break,
            }
        }
        println!("CPU slice finished pc={:08x}", self.cpu.read_fetch_pc());
    }
}

