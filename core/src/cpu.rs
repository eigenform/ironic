
pub mod coproc;
pub mod reg;
pub mod mmu;
pub mod exec;
pub mod lut;
pub mod alu;

use std::sync::{Arc,RwLock};

use crate::dbg::*;
use crate::bus::*;
use crate::cpu::lut::*;

use crate::cpu::exec::DispatchRes;

use crate::cpu::exec::arm;
use crate::cpu::exec::arm::decode::ArmInst;
use crate::cpu::exec::arm::dispatch::ArmFn;

use crate::cpu::exec::thumb;
use crate::cpu::exec::thumb::decode::ThumbInst;
use crate::cpu::exec::thumb::dispatch::ThumbFn;

/// Container for lookup tables
pub struct CpuLut {
    /// Lookup table for ARM instructions.
    pub arm: arm::ArmLut,
    /// Lookup table for Thumb instructions.
    pub thumb: thumb::ThumbLut,
}
impl CpuLut {
    pub fn new() -> Self {
        CpuLut {
            arm: arm::ArmLut::create_lut(ArmFn(arm::dispatch::unimpl_instr)),
            thumb: thumb::ThumbLut::create_lut(ThumbFn(thumb::dispatch::unimpl_instr)),
        }
    }
}

/// Result after exiting the emulated CPU.
pub enum CpuRes {
    /// Some unrecoverable error occured and we need to stop emulation.
    HaltEmulation,
    /// We single-stepped and returned successfully.
    StepOk,
}

/// Container for an ARMv5-compatible CPU.
pub struct Cpu {

    /// NOTE: Hacky scratch register, for dealing with the fact that Thumb BL 
    /// instructions are interpreted as a pair of 16-bit instructions
    pub scratch: u32,

    /// The CPU's register file
    pub reg: reg::RegisterFile,

    /// The system control co-processor
    pub p15: coproc::SystemControl,

    /// ARM/Thumb lookup tables (instruction decoding)
    pub lut: CpuLut,

    /// The CPU's memory management unit
    pub mmu: mmu::Mmu,

    /// Some shared state with the UI thread.
    pub dbg: Arc<RwLock<Debugger>>,
    pub log_buf: Vec<String>,
}
impl Cpu {
    pub fn new(dbg: Arc<RwLock<Debugger>>, bus: Arc<RwLock<Bus>>) -> Self { 
        let cpu = Cpu {
            reg: reg::RegisterFile::new(),
            p15: coproc::SystemControl::new(),
            lut: CpuLut::new(),
            mmu: mmu::Mmu::new(bus),
            scratch: 0,
            log_buf: Vec::new(),
            dbg
        };
        log(&cpu.dbg, LogLevel::Cpu, "CPU instantiated");
        cpu
    }
}

/// Some helper functions, enshrining conventions for dealing with CPU state.
/// The ARM926EJS has a five-stage pipeline which looks like this:
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

    /// Decode and dispatch an ARM instruction.
    pub fn exec_arm(&mut self) -> DispatchRes {
        let opcd = self.mmu.read32(self.read_fetch_pc());

        self.log(format!("{:08x}: {:12} {:x?} ", self.read_fetch_pc(),
            format!("{:?}",ArmInst::decode(opcd)), self.reg));

        if self.reg.cond_pass(opcd) {
            let func = self.lut.arm.lookup(opcd);
            func.0(self, opcd)
        } else {
            DispatchRes::CondFailed
        }
    }

    /// Decode and dispatch a Thumb instruction.
    pub fn exec_thumb(&mut self) -> DispatchRes {
        let opcd = self.mmu.read16(self.read_fetch_pc());

        self.log(format!("{:08x}: {:12} {:x?} ", self.read_fetch_pc(), 
            format!("{:?}", ThumbInst::decode(opcd)), self.reg));

        let func = self.lut.thumb.lookup(opcd);
        func.0(self, opcd)
    }
}

/// Logging/debugging interfaces.
impl Cpu {
    fn log(&mut self, s: String) { self.log_buf.push(s); }

    /// Drain the CPU log buffer into the attached debugger.
    fn drain_log_buffer(&mut self) {
        let mut entries = std::mem::replace(&mut self.log_buf, Vec::new());
        let mut dbg = self.dbg.write().unwrap();
        for entry in entries.drain(..) {
            dbg.console_buf.push(LogEntry{ lvl: LogLevel::Cpu, data: entry });
        }
    }
}


impl Cpu {
    pub fn step(&mut self) -> CpuRes {
        let disp_res = if self.reg.cpsr.thumb() {
            self.exec_thumb()
        } else {
            self.exec_arm()
        };

        let cpu_res = match disp_res {
            DispatchRes::RetireOk | DispatchRes::CondFailed => {
                self.increment_pc(); 
                CpuRes::StepOk
            },
            DispatchRes::RetireBranch => CpuRes::StepOk,
            DispatchRes::FatalErr => CpuRes::HaltEmulation,
            _ => unreachable!(),
        };

        self.drain_log_buffer();
        // Update the debugger's copy of the register file and return.
        self.dbg.write().unwrap().reg = self.reg;
        cpu_res
    }
}

