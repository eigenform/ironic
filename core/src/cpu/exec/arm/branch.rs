//! Implementation of branching instructions.

use crate::cpu::*;
use crate::cpu::reg::*;
use crate::cpu::exec::arm::bits::*;

pub fn sign_extend(x: u32, bits: i32) -> i32 {
    if ((x as i32 >> (bits - 1)) & 1) != 0 { 
        x as i32 | !0 << bits 
    } else { 
        x as i32 
    }
}
pub fn bl_imm(cpu: &mut Cpu, op: BranchBits) -> DispatchRes {
    let offset = sign_extend(op.imm24(), 24) * 4;
    let new_lr = cpu.read_fetch_pc().wrapping_add(4);
    let dest_pc = (cpu.read_exec_pc() as i32).wrapping_add(offset) as u32;

    println!("  lr={:08x}", new_lr);
    println!("  dest_pc={:08x}", dest_pc);

    cpu.reg[Reg::Lr] = new_lr;
    cpu.write_exec_pc(dest_pc);
    DispatchRes::RetireBranch
}
pub fn b(cpu: &mut Cpu, op: BranchBits) -> DispatchRes {
    let offset = sign_extend(op.imm24(), 24) * 4;
    let target = (cpu.read_exec_pc() as i32).wrapping_add(offset) as u32;
    cpu.write_exec_pc(target);
    DispatchRes::RetireBranch
}
pub fn bx(cpu: &mut Cpu, op: BxBits) -> DispatchRes {
    let dest_pc = cpu.reg[op.rm()];
    cpu.reg.cpsr.set_thumb(dest_pc & 1 != 0);
    cpu.write_exec_pc(dest_pc & 0xffff_fffe);
    DispatchRes::RetireBranch
}


