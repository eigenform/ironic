
use crate::dbg::*;
use crate::cpu::*;

use crate::cpu::armv5::*;
use crate::cpu::armv5::decode::*;
use crate::cpu::armv5::bits::*;
use crate::cpu::armv5::reg::*;
use crate::cpu::armv5::alu::*;


/// Unimplemented instruction handler.
pub fn unimpl_instr(cpu: &mut Cpu, op: u32) -> DispatchRes {
    log(&cpu.dbg, LogLevel::Cpu, &format!(
        "Couldn't dispatch instruction {:08x} ({:?})", 
        op, ArmInst::decode(op)
    ));
    DispatchRes::FatalErr
}


pub fn mov_imm(cpu: &mut Cpu, op: MovSpImmBits) -> DispatchRes {
    if op.rd() == 15 {
        return DispatchRes::FatalErr;
    }

    let (res, carry) = barrel_shift(ShiftArgs::Imm { 
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c() 
    });

    cpu.reg[op.rd()] = res;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    DispatchRes::RetireOk
}


pub fn mov_reg(cpu: &mut Cpu, op: MovSpRegBits) -> DispatchRes {
    let (res, carry) = barrel_shift(ShiftArgs::Reg {
        rm: cpu.reg[op.rm()], 
        stype: op.stype(), 
        imm5: op.imm5(), 
        c_in: cpu.reg.cpsr.c()
    });

    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }

    if op.rd() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        DispatchRes::RetireOk
    }
}



pub fn ldr_imm_or_lit(cpu: &mut Cpu, op: u32) -> DispatchRes {
    use ArmInst::*;
    match ArmInst::decode(op) {
        LdrLit => return ldr_lit(cpu, LdrLitBits(op)),
        LdrImm => return ldr_imm(cpu, LsImmBits(op)),
        _ => unreachable!(),
    }
}
pub fn ldr_lit(cpu: &mut Cpu, op: LdrLitBits) -> DispatchRes {
    if op.w() {
        return DispatchRes::FatalErr;
    }

    let addr = cpu.read_exec_pc();
    let target_addr = if op.p() { 
        if op.u() { 
            addr.wrapping_add(op.imm12()) 
        } else { 
            addr.wrapping_sub(op.imm12()) 
        }
    } else { 
        addr 
    };
    
    let res = cpu.mmu.read32(target_addr);
    if op.rt() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rt()] = res;
        DispatchRes::RetireOk
    }
}
pub fn ldr_imm(cpu: &mut Cpu, op: LsImmBits) -> DispatchRes {
    DispatchRes::FatalErr
}




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


