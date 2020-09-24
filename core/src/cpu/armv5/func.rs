
use crate::dbg::*;
use crate::cpu::*;
use crate::cpu::armv5::*;
use crate::cpu::armv5::decode::*;
use crate::cpu::armv5::bits::*;


/// Unimplemented instruction handler.
pub fn unimpl_instr(cpu: &mut Cpu, op: u32) -> DispatchRes {
    log(&cpu.dbg, LogLevel::Cpu, &format!(
        "Couldn't dispatch instruction {:08x} ({:?})", 
        op, ArmInst::decode(op)
    ));
    DispatchRes::FatalErr
}



pub fn ldr_imm_or_lit(cpu: &mut Cpu, op: u32) -> DispatchRes {
    use ArmInst::*;
    match ArmInst::decode(op) {
        LdrLit => return ldr_lit(cpu, LdrLitBits(op)),
        LdrImm => return ldr_imm(cpu, LdrImmBits(op)),
        _ => unreachable!(),
    }
}

pub fn ldr_lit(cpu: &mut Cpu, op: LdrLitBits) -> DispatchRes {
    DispatchRes::FatalErr
}

pub fn ldr_imm(cpu: &mut Cpu, op: LdrImmBits) -> DispatchRes {
    DispatchRes::FatalErr
}
