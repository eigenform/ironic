
use crate::cpu::*;
use crate::cpu::bits::*;
use crate::cpu::reg::*;
use crate::cpu::dispatch::*;


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
pub fn bx(cpu: &mut Cpu, op: BxBits) -> DispatchRes {
    let rm = cpu.reg[op.rm()];
    cpu.reg.cpsr.set_thumb(rm & 1 != 0);
    cpu.write_exec_pc(rm & 0xffff_fffe);
    DispatchRes::RetireBranch
}


