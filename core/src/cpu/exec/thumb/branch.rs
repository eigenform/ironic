
use crate::cpu::*;
use crate::cpu::reg::{Cond, Reg};
use crate::cpu::exec::thumb::bits::*;

pub fn sign_extend(x: u32, bits: i32) -> i32 {
    if ((x as i32 >> (bits - 1)) & 1) != 0 { 
        x as i32 | !0 << bits 
    } else { 
        x as i32 
    }
}

pub fn bl_prefix(cpu: &mut Cpu, op: BlBits) -> DispatchRes {
    let offset = sign_extend((op.imm11() as u32) << 12, 23);
    let res = (cpu.read_exec_pc() as i32).wrapping_add(offset);
    cpu.scratch = res as u32;
    DispatchRes::RetireOk
}
pub fn bl_imm_suffix(cpu: &mut Cpu, op: BlBits) -> DispatchRes {
    let offset = (op.imm11() as u32) << 1;
    let dest_pc = cpu.scratch.wrapping_add(offset);
    let new_lr = cpu.read_fetch_pc().wrapping_add(2) | 1;
    cpu.reg[Reg::Lr] = new_lr;
    cpu.write_exec_pc(dest_pc);
    DispatchRes::RetireBranch
}
pub fn bx(cpu: &mut Cpu, op: BxBits) -> DispatchRes {
    let dest_pc = cpu.reg[op.rm()];
    cpu.reg.cpsr.set_thumb(dest_pc & 1 != 0);
    cpu.write_exec_pc(dest_pc & 0xffff_fffe);
    DispatchRes::RetireBranch
}

pub fn blx_reg(cpu: &mut Cpu, op: BxBits) -> DispatchRes {
    assert_ne!(op.rm(), 15);
    let new_lr = cpu.read_fetch_pc().wrapping_add(2) | 1;
    let dest_pc = cpu.reg[op.rm()];
    cpu.reg.cpsr.set_thumb(dest_pc & 1 != 0);
    cpu.reg[Reg::Lr] = new_lr;
    cpu.write_exec_pc(dest_pc & 0xffff_fffe);
    DispatchRes::RetireBranch
}

pub fn b_unconditional(cpu: &mut Cpu, op: BranchAltBits) -> DispatchRes {
    let offset = sign_extend(op.imm11() as u32, 11) << 1;
    let dest_pc = (cpu.read_exec_pc() as i32).wrapping_add(offset) as u32;
    cpu.write_exec_pc(dest_pc);
    DispatchRes::RetireBranch
}

pub fn b(cpu: &mut Cpu, op: BranchBits) -> DispatchRes {
    if cpu.reg.is_cond_satisfied(Cond::from(op.cond() as u32)) {
        let offset = sign_extend(op.imm8() as u32, 8) << 1;
        //let offset = ((op.imm8() as u32) << 1) as i32;
        let dest_pc = (cpu.read_exec_pc() as i32).wrapping_add(offset) as u32;
        cpu.write_exec_pc(dest_pc);
        DispatchRes::RetireBranch
    } else {
        DispatchRes::RetireOk
    }
}
