
pub mod bits;

pub mod dispatch;
pub mod decode;
pub mod interp;

pub mod coproc;
pub mod reg;
pub mod mmu;

use std::sync::{Arc,RwLock};

use crate::dbg::*;
use crate::topo::*;
use crate::bus::*;
use crate::cpu::*;

use decode::ArmInst;


/// Result of dispatching an instruction.
#[derive(Debug)]
pub enum DispatchRes {
    /// There was some fatal error dispatching the instruction.
    FatalErr,
    /// This instruction was not executed because the condition failed.
    CondFailed,
    /// This instruction retired and resulted in a branch.
    RetireBranch,
    /// This instruction retired and the PC should be incremented.
    RetireOk,
}

/// Result after exiting emulated CPU execution.
pub enum CpuRes {
    /// Some unrecoverable error occured and we need to stop emulation.
    HaltEmulation,
    /// We single-stepped and returned successfully.
    StepOk,
}

/// Container for an ARMv5-compatible CPU.
pub struct Cpu {
    pub reg: reg::RegisterFile,
    pub p15: coproc::SystemControl,
    pub lut: dispatch::Lut,
    pub mmu: mmu::Mmu,
    pub dbg: Arc<RwLock<Debugger>>,
}
impl Cpu {
    pub fn new(dbg: Arc<RwLock<Debugger>>, bus: Arc<RwLock<Topology>>) -> Self { 
        let cpu = Cpu {
            reg: reg::RegisterFile::new(),
            p15: coproc::SystemControl::new(),
            lut: dispatch::Lut::new(),
            mmu: mmu::Mmu::new(bus),
            dbg
        };
        log(&cpu.dbg, LogLevel::Cpu, "CPU instantiated");
        cpu
    }
}

/// Some helper functions, enshrining conventions for dealing with CPU state.
/// The ARM926-EJS has a five-stage pipeline which looks like this:
///
/// 1. Fetch an instruction from memory.
/// 2. Decode an instruction.
/// 3. Execute an instruction.
/// 4. Perform some data access/es to memory.
/// 5. Write back an instruction's results to the register file.
///
/// In hardware, one of more of these stages are occuring in a single cycle.
/// Regardless of where we are in the pipeline, the value of the program 
/// counter is always being read as "the address of the instruction currently
/// being fetched from memory."

impl Cpu {
    /// Read the program counter (from the context of the fetch stage).
    pub fn read_fetch_pc(&self) -> u32 {
        if self.reg.cpsr.thumb() {
            self.reg.pc.wrapping_sub(4)
        } else { 
            self.reg.pc.wrapping_sub(8)
        }
    }

    /// Read the program counter (from the context of the execute stage).
    pub fn read_exec_pc(&self) -> u32 { 
        self.reg.pc 
    }

    /// Write the program counter (from the context of the execute stage).
    pub fn write_exec_pc(&mut self, val: u32) {
        let new_pc = if self.reg.cpsr.thumb() {
            val.wrapping_add(4)
        } else {
            val.wrapping_add(8)
        };
        self.reg.cpsr.set_thumb((new_pc & 1) != 0);
        self.reg.pc = new_pc;
    }

    /// Increment the program counter.
    pub fn increment_pc(&mut self) {
        if self.reg.cpsr.thumb() {
            self.reg.pc = self.reg.pc.wrapping_add(2);
        } else {
            self.reg.pc = self.reg.pc.wrapping_add(4);
        }
    }
}


impl Cpu {
    pub fn step(&mut self) -> CpuRes {
        // Fetch an instruction from memory.
        let opcd = self.mmu.read32(self.read_fetch_pc());

        // Decode/dispatch an instruction.
        let disp_res = if self.reg.cond_pass(opcd) {
            let func = self.lut.arm.lookup(opcd);
            func.0(self, opcd)
        } else {
            DispatchRes::CondFailed
        };

        log(&self.dbg, LogLevel::Cpu, &format!(
            "{:08x}: Dispatched {:08x} ({:?})", 
            self.read_fetch_pc(), opcd, ArmInst::decode(opcd)
        ));

        let cpu_res = match disp_res {
            DispatchRes::RetireOk | DispatchRes::CondFailed => {
                self.increment_pc(); 
                CpuRes::StepOk
            },
            DispatchRes::RetireBranch => CpuRes::StepOk,
            DispatchRes::FatalErr => {
                log(&self.dbg, LogLevel::Cpu, &format!(
                    "Fatal error after dispatching {:?} at {:08x}",
                    ArmInst::decode(opcd), self.read_fetch_pc()
                ));
                CpuRes::HaltEmulation
            },
            _ => unreachable!(),
        };

        // Update the debugger's copy of the register file and return.
        self.dbg.write().unwrap().reg = self.reg;
        cpu_res
    }
}

