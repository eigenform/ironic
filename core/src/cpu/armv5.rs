
pub mod reg;
pub mod coproc;
pub mod dispatch;
pub mod decode;
pub mod func;
pub mod bits;

use std::sync::{Arc,RwLock};

use crate::dbg::*;
use crate::topo::*;
use crate::bus::*;
use crate::cpu::*;


/// Result of dispatching an instruction.
#[derive(Debug)]
pub enum DispatchRes {
    FatalErr,
    Continue,
}

/// Result after exiting emulated CPU execution.
pub enum CpuRes {
    EmuThreadHalt,
    StepCompleted,
}

/// Container for an ARMv5-compatible CPU.
pub struct Cpu {
    pub pc: u32,
    pub reg: reg::RegisterFile,
    pub p15: coproc::SystemControl,
    pub lut: dispatch::Lut,
    pub dbg: Arc<RwLock<Debugger>>,
}
impl Cpu {
    pub fn new(dbg: Arc<RwLock<Debugger>>) -> Self { 
        let cpu = Cpu {
            pc: 0xffff_0000,
            reg: reg::RegisterFile::new(),
            p15: coproc::SystemControl::new(),
            lut: dispatch::Lut::new(),
            dbg,
        };
        log(&cpu.dbg, LogLevel::Cpu, "CPU instantiated");
        cpu
    }

    pub fn step(&mut self, top: &mut Topology) -> CpuRes {
        let opcd = top.read32(self.reg.pc);
        let func = self.lut.arm.lookup(opcd);
        let res = func.0(self, opcd);

        // Update debugger's view of the register file.
        // This is fine when we're single-stepping.
        self.dbg.write().unwrap().reg = self.reg;

        match res {
            DispatchRes::FatalErr => return CpuRes::EmuThreadHalt,
            DispatchRes::Continue => {
                self.reg.pc += 4;
                return CpuRes::StepCompleted;
            },
        }
    }
}


