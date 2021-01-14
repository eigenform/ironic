
pub mod coproc;
pub mod excep;
pub mod reg;
pub mod psr;
pub mod mmu;
pub mod alu;

use std::sync::{Arc,RwLock};

use crate::bus::*;
use crate::cpu::excep::*;

/// Result after exiting the emulated CPU.
pub enum CpuRes {
    /// Some unrecoverable error occured and we need to stop emulation.
    HaltEmulation,
    /// We single-stepped and returned successfully.
    StepOk,
    /// We single stepped and took some exception.
    StepException(ExceptionType),
    /// We caught a Realview Semihosting command.
    Semihosting,
}

/// Container for ARMv5-compatible CPU state.
pub struct Cpu {
    pub bus: Arc<RwLock<Bus>>,
    /// The CPU's register file.
    pub reg: reg::RegisterFile,
    /// The system control co-processor.
    pub p15: coproc::SystemControl,

    /// Current stage in the boot process.
    pub boot_status: BootStatus,

    pub current_exception: Option<ExceptionType>,

    pub scratch: u32,
    pub dbg_on: bool,
    pub dbg_steps: u32,

    /// Whether or not an interrupt request is currently asserted.
    pub irq_input: bool,
}
impl Cpu {
    pub fn new(bus: Arc<RwLock<Bus>>) -> Self { 
        let cpu = Cpu {
            bus,
            reg: reg::RegisterFile::new(),
            p15: coproc::SystemControl::new(),
            scratch: 0,
            irq_input: false,
            boot_status: BootStatus::Boot0,
            current_exception: None,
            dbg_on: false,
            dbg_steps: 1_000_000,
        };
        cpu
    }
}

/// Helper functions/conventions for transforming CPU state.
impl Cpu {
    /// Read the program counter (from the context of the fetch stage).
    pub fn read_fetch_pc(&self) -> u32 {
        let pc_adj = if self.reg.cpsr.thumb() { 4 } else { 8 };
        self.reg.pc.wrapping_sub(pc_adj)
    }

    /// Read the program counter (from the context of the execute stage).
    pub fn read_exec_pc(&self) -> u32 { self.reg.pc }

    /// Write the program counter (from the context of the execute stage).
    pub fn write_exec_pc(&mut self, val: u32) {
        let pc_adj = if self.reg.cpsr.thumb() { 4 } else { 8 };
        self.reg.pc = val.wrapping_add(pc_adj);
    }

    /// Increment the program counter, depending on the Thumb bit state.
    pub fn increment_pc(&mut self) {
        let pc_inc = if self.reg.cpsr.thumb() { 2 } else { 4 };
        self.reg.pc = self.reg.pc.wrapping_add(pc_inc);
    }
}


/// Current status of the platform's boot process.
#[derive(PartialEq)]
pub enum BootStatus { Boot0, Boot1, Boot2Stub, Boot2, Kernel }
impl Cpu {
    pub fn update_boot_status(&mut self) {
        match self.boot_status {
            BootStatus::Boot0 => {
                if self.read_fetch_pc() == 0xfff0_0000 { 
                    println!("Entered boot1");
                    self.boot_status = BootStatus::Boot1;
                }
            }
            BootStatus::Boot1 => {
                if self.read_fetch_pc() == 0xfff0_0058 { 
                    println!("Entered boot2 stub");
                    self.boot_status = BootStatus::Boot2Stub;
                }
            }
            BootStatus::Boot2Stub => {
                if self.read_fetch_pc() == 0xffff_0000 { 
                    println!("Entered boot2");
                    self.boot_status = BootStatus::Boot2;
                }
            }
            BootStatus::Boot2 => {
                if self.read_fetch_pc() == 0xffff_2224 { 
                    println!("Entered kernel");
                    self.boot_status = BootStatus::Kernel;
                }

            }
            _ => {},
        }
    }


}

