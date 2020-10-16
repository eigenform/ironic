
use crate::cpu::*;
use crate::cpu::reg::Reg;
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

    cpu.reg[Reg::Lr] = cpu.read_exec_pc().wrapping_sub(2) | 1;
    cpu.write_exec_pc(dest_pc);
    DispatchRes::RetireBranch
}
