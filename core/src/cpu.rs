
pub mod coproc;
pub mod excep;
pub mod reg;
pub mod mmu;
pub mod exec;
pub mod lut;
pub mod alu;

use std::sync::{Arc,RwLock};

use crate::dbg::*;
use crate::bus::*;
use crate::cpu::lut::*;
use crate::cpu::reg::*;
use crate::cpu::excep::*;
use crate::cpu::coproc::CoprocTask;

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
    /// We single stepped and took some exception.
    StepException(ExceptionType),
}


#[derive(PartialEq)]
pub enum CpuStatus { Boot0, Boot1, Boot2Stub, Boot2, Kernel }

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
    pub status: CpuStatus,
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
            status: CpuStatus::Boot0,
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
}

/// These are functions that define the behavior for dealing with the current
/// mode and behavior for implementing exceptions.
impl Cpu {

    pub fn set_mode(&mut self, target_mode: CpuMode) {
        if target_mode == self.reg.mode { return; }
        self.reg.swap_bank(target_mode);
        self.reg.cpsr.set_mode(target_mode);
        self.reg.mode = target_mode;
        self.mmu.cpu_mode = target_mode;
    }

    /// Cause the CPU to take some exception.
    pub fn generate_exception(&mut self, e: ExceptionType) {
        let current_mode = self.reg.cpsr.mode();
        assert_eq!(current_mode, self.reg.mode);

        let target_mode = CpuMode::from(e);
        let target_pc = ExceptionType::get_vector(e);
        let return_pc = self.read_fetch_pc()
            .wrapping_add(ExceptionType::get_pc_off(e, self.reg.cpsr.thumb()));

        // Just log syscalls here, for now
        match e {
            ExceptionType::Undef(opcd) => {
                ios::log_syscall(opcd, self.read_fetch_pc(), self.reg[Reg::Lr]); 
            },
            _ => {},
        }

        // Save the CPSR to the target SPSR, then change mode
        let cpsr = self.reg.read_cpsr();
        self.reg.spsr.write(target_mode, cpsr);
        self.set_mode(target_mode);

        self.reg.cpsr.set_thumb(false);
        if e == ExceptionType::Fiq {
            self.reg.cpsr.set_fiq_disable(true);
        }
        self.reg.cpsr.set_irq_disable(true);
        self.reg[Reg::Lr] = return_pc;
        self.write_exec_pc(target_pc);
    }

    /// Return from an exception.
    pub fn exception_return(&mut self, dest_pc: u32) {
        assert_ne!(self.reg.mode, CpuMode::Usr);
        assert_ne!(self.reg.mode, CpuMode::Sys);

        let current_mode = self.reg.mode;
        let current_spsr = self.reg.spsr.read(self.reg.mode);

        self.reg.write_cpsr(current_spsr);
        self.write_exec_pc(dest_pc & 0xffff_fffe);
    }
}


/// These functions implement the accesses and side-effects associated with
/// the system control coproessor (p15).

impl Cpu {
    /// Read from the system control coprocessor.
    pub fn read_p15(&mut self, crn: u32, crm: u32, opcd2: u32) -> u32 {
        self.p15.read(crn, crm, opcd2)
    }

    /// Write to the system control coprocessor, then potentially handle some 
    /// side-effects (specifically, on the MMU) associated with a particular 
    /// change in some register.
    pub fn write_p15(&mut self, val: u32, crn: u32, crm: u32, opcd2: u32) {
        let res = self.p15.write(val, crn, crm, opcd2);
        //println!("P15 write returned {:?}", res);
        match res {
            CoprocTask::ControlChange => self.mmu.ctrl = self.p15.c1_ctrl,
            CoprocTask::TtbrChange => self.mmu.ttbr = self.p15.c2_ttbr0,
            CoprocTask::DacrChange => self.mmu.dacr = self.p15.c3_dacr,
            CoprocTask::None => {},
        }
    }
}


/// These are functions for decoding and dispatching an instruction from
/// either the ARM or Thumb lookup table.

impl Cpu {
    /// Decode and dispatch an ARM instruction.
    pub fn exec_arm(&mut self) -> DispatchRes {
        let opcd = self.mmu.read32(self.read_fetch_pc());

        //if self.status == CpuStatus::Boot2 {
        //    let pc = self.read_fetch_pc();
        //    let opname = format!("{:?}", ArmInst::decode(opcd));
        //    println!("({:08x}) {:12} {:x?}", opcd, opname, self.reg);
        //}

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

        //if self.status == CpuStatus::Boot2 {
        //    let pc = self.read_fetch_pc();
        //    let opname = format!("{:?}", ThumbInst::decode(opcd));
        //    println!("({:08x}) {:12} {:x?}", opcd, opname, self.reg);
        //}

        let func = self.lut.thumb.lookup(opcd);
        func.0(self, opcd)
    }
}


impl Cpu {
    pub fn step(&mut self) -> CpuRes {
        assert!((self.read_fetch_pc() & 1) == 0);

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
            DispatchRes::RetireBranch => {
                CpuRes::StepOk
            },
            DispatchRes::Exception(e) => {
                self.generate_exception(e);
                CpuRes::StepException(e)
            },
            DispatchRes::FatalErr => {
                println!("CPU halted at pc={:08x}", self.read_fetch_pc());
                CpuRes::HaltEmulation
            },
            _ => unreachable!(),
        };

        match self.status {
            CpuStatus::Boot0 => {
                if self.read_fetch_pc() == 0xfff0_0000 { 
                    println!("Entered boot1");
                    self.status = CpuStatus::Boot1;
                }
            }
            CpuStatus::Boot1 => {
                if self.read_fetch_pc() == 0xfff0_0058 { 
                    println!("Entered boot2 stub");
                    self.status = CpuStatus::Boot2Stub;
                }
            }
            CpuStatus::Boot2Stub => {
                if self.read_fetch_pc() == 0xffff_0000 { 
                    println!("Entered boot2");
                    self.status = CpuStatus::Boot2;
                }
            }

            _ => {},
        }
        cpu_res
    }
}

