
use crate::cpu::*;
use crate::cpu::bits::*;
use crate::cpu::dispatch::*;
use crate::cpu::interp::alu::*;

pub fn sub_generic(rn: u32, val: u32) -> (u32, bool, bool, bool, bool) {
    let res = rn.wrapping_sub(val);
    let n = (res & 0x8000_0000) != 0;
    let z = res == 0;
    let c = rn.checked_sub(val).is_some();
    let v = (rn as i32).checked_sub(val as i32).is_none();
    (res, n, z, c, v)
}
pub fn add_generic(rn: u32, val: u32) -> (u32, bool, bool, bool, bool) {
    let res = rn.wrapping_add(val);
    let n = (res & 0x8000_0000) != 0;
    let z = res == 0;
    let c = rn.checked_add(val).is_none();
    let v = (rn as i32).checked_add(val as i32).is_none();
    (res, n, z, c, v)
}

pub fn add_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = add_generic(cpu.reg[op.rn()], val);
    if op.s() {
        cpu.reg.cpsr.set_n(n);
        cpu.reg.cpsr.set_z(z);
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

pub fn add_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    let (val, carry) = barrel_shift(ShiftArgs::Reg { rm: cpu.reg[op.rm()], 
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = add_generic(cpu.reg[op.rn()], val);
    if op.s() {
        cpu.reg.cpsr.set_n(n);
        cpu.reg.cpsr.set_z(z);
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
    let (res, n, z, c, v) = sub_generic(cpu.reg[op.rn()], val);
    if op.s() {
        cpu.reg.cpsr.set_n(n);
        cpu.reg.cpsr.set_z(z);
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


pub fn mov_imm(cpu: &mut Cpu, op: MovImmBits) -> DispatchRes {
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
pub fn mov_reg(cpu: &mut Cpu, op: MovRegBits) -> DispatchRes {
    let (res, carry) = barrel_shift(ShiftArgs::Reg { rm: cpu.reg[op.rm()], 
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
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


pub fn orr_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let (val, carry) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let res = cpu.reg[op.rn()] | val;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    cpu.reg[op.rd()] = res;
    DispatchRes::RetireOk
}
pub fn orr_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let (val, carry) = barrel_shift(ShiftArgs::Reg {
        rm: cpu.reg[op.rm()], 
        stype: op.stype(), 
        imm5: op.imm5(), 
        c_in: cpu.reg.cpsr.c()
    });

    let res = cpu.reg[op.rn()] | val;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    cpu.reg[op.rd()] = res;
    DispatchRes::RetireOk
}

pub fn eor_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let (val, carry) = barrel_shift(ShiftArgs::Reg {
        rm: cpu.reg[op.rm()], 
        stype: op.stype(), 
        imm5: op.imm5(), 
        c_in: cpu.reg.cpsr.c()
    });

    let res = cpu.reg[op.rn()] ^ val;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    cpu.reg[op.rd()] = res;
    DispatchRes::RetireOk
}



pub fn and_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let (val, carry) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let res = cpu.reg[op.rn()] & val;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    cpu.reg[op.rd()] = res;
    DispatchRes::RetireOk
}

pub fn bic_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let (val, carry) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let res = cpu.reg[op.rn()] & !val;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    cpu.reg[op.rd()] = res;
    DispatchRes::RetireOk
}



pub fn cmn_imm(cpu: &mut Cpu, op: DpTestImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (_, n, z, c, v) = add_generic(cpu.reg[op.rn()], val);
    cpu.reg.cpsr.set_n(n);
    cpu.reg.cpsr.set_z(z);
    cpu.reg.cpsr.set_c(c);
    cpu.reg.cpsr.set_v(v);
    DispatchRes::RetireOk
}


pub fn cmp_imm(cpu: &mut Cpu, op: DpTestImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (_, n, z, c, v) = sub_generic(cpu.reg[op.rn()], val);
    cpu.reg.cpsr.set_n(n);
    cpu.reg.cpsr.set_z(z);
    cpu.reg.cpsr.set_c(c);
    cpu.reg.cpsr.set_v(v);
    DispatchRes::RetireOk
}

pub fn cmp_reg(cpu: &mut Cpu, op: DpTestRegBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Reg {
        rm: cpu.reg[op.rm()], 
        stype: op.stype(), 
        imm5: op.imm5(), 
        c_in: cpu.reg.cpsr.c()
    });

    let (_, n, z, c, v) = sub_generic(cpu.reg[op.rn()], val);
    cpu.reg.cpsr.set_n(n);
    cpu.reg.cpsr.set_z(z);
    cpu.reg.cpsr.set_c(c);
    cpu.reg.cpsr.set_v(v);
    DispatchRes::RetireOk
}


pub fn tst_reg(cpu: &mut Cpu, op: DpTestRegBits) -> DispatchRes {
    let (val, carry) = barrel_shift(ShiftArgs::Reg {
        rm: cpu.reg[op.rm()], 
        stype: op.stype(), 
        imm5: op.imm5(), 
        c_in: cpu.reg.cpsr.c()
    });

    let res = cpu.reg[op.rn()] & val;
    cpu.reg.cpsr.set_n(res & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(res == 0);
    cpu.reg.cpsr.set_c(carry);
    DispatchRes::RetireOk
}



