
pub mod reg;
pub mod coproc;
pub mod dispatch;
pub mod decode;
pub mod func;
pub mod bits;

use crate::dbg::*;
use std::sync::{Arc,RwLock};

pub type CpuResult = Result<CpuExit, &'static str>;

/// 
pub enum CpuExit {
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
}


