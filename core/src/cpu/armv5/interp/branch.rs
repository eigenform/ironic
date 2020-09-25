
use crate::cpu::*;
use crate::dbg::*;
use crate::cpu::armv5::*;
use crate::cpu::armv5::bits::*;
use crate::cpu::armv5::reg::*;



pub fn sign_extend(x: u32, bits: i32) -> i32 {
    if ((x as i32 >> (bits - 1)) & 1) != 0 { 
        x as i32 | !0 << bits 
    } else { 
        x as i32 
    }
}
pub fn bl_imm(cpu: &mut Cpu, op: BranchBits) -> DispatchRes {
    cpu.reg[Reg::Lr] = cpu.read_exec_pc().wrapping_sub(4);
    let offset = sign_extend(op.imm24(), 24) * 4;
    let target = (cpu.read_exec_pc() as i32).wrapping_add(offset) as u32;
    cpu.write_exec_pc(target);
    DispatchRes::RetireBranch
}
pub fn b(cpu: &mut Cpu, op: BranchBits) -> DispatchRes {
    let offset = sign_extend(op.imm24(), 24) * 4;
    let target = (cpu.read_exec_pc() as i32).wrapping_add(offset) as u32;
    cpu.write_exec_pc(target);
    DispatchRes::RetireBranch
}


