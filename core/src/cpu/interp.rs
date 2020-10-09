
use crate::dbg::*;
use crate::cpu::*;
use crate::cpu::decode::*;
use crate::cpu::dispatch::*;

pub mod alu;
pub mod loadstore;
pub mod dataproc;
pub mod branch;
pub mod coproc;

/// Unimplemented instruction handler.
pub fn unimpl_instr(cpu: &mut Cpu, op: u32) -> DispatchRes {
    log(&cpu.dbg, LogLevel::Cpu, &format!(
        "Couldn't dispatch instruction {:08x} ({:?})", 
        op, ArmInst::decode(op)
    ));
    println!("Couldn't dispatch instruction {:08x} ({:?})",
        op, ArmInst::decode(op));
    DispatchRes::FatalErr
}

