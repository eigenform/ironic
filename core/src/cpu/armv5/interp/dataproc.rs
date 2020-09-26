
use crate::cpu::armv5::*;
use crate::cpu::armv5::bits::*;
use crate::cpu::armv5::interp::alu::*;

pub fn sub_generic(rn: u32, val: u32) -> (u32, bool, bool) {
    let res = rn.wrapping_sub(val);
    let unsigned_overflow = rn.checked_sub(val).is_some();
    let signed_overflow = (rn as i32).checked_sub(val as i32).is_none();
    (res, unsigned_overflow, signed_overflow)
}
pub fn add_generic(rn: u32, val: u32) -> (u32, bool, bool) {
    let res = rn.wrapping_add(val);
    let unsigned_overflow = rn.checked_add(val).is_none();
    let signed_overflow = (rn as i32).checked_add(val as i32).is_none();
    (res, unsigned_overflow, signed_overflow)
}

pub fn add_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (res, v, c) = add_generic(cpu.reg[op.rn()], val);
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(c);
        cpu.reg.cpsr.set_v(v);
    }
    if op.rd() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        DispatchRes::RetireOk
    }
}


pub fn sub_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (res, v, c) = sub_generic(cpu.reg[op.rn()], val);
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(c);
        cpu.reg.cpsr.set_v(v);
    }
    if op.rd() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        DispatchRes::RetireOk
    }
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





