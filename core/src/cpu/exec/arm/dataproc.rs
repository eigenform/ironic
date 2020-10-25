//! Data-processing instructions.

use crate::cpu::*;
use crate::cpu::alu::*;
use crate::cpu::exec::arm::bits::*;

/// Set all of the condition flags.
macro_rules! set_all_flags { 
    ($cpu:ident, $n:ident, $z:ident, $c:ident, $v:ident) => {
        $cpu.reg.cpsr.set_n($n);
        $cpu.reg.cpsr.set_z($z);
        $cpu.reg.cpsr.set_c($c);
        $cpu.reg.cpsr.set_v($v);
    }
}

pub fn add_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = add_generic(cpu.reg[op.rn()], val);
    if op.s() {
        set_all_flags!(cpu, n, z, c, v);
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
    let rm = if op.rm() == 15 { cpu.read_exec_pc() } else { cpu.reg[op.rm()] };
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = add_generic(cpu.reg[op.rn()], val);
    if op.s() {
        set_all_flags!(cpu, n, z, c, v);
    }
    if op.rd() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        DispatchRes::RetireOk
    }
}

pub fn rsb_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm: cpu.reg[op.rm()],
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = sub_generic(val, cpu.reg[op.rn()]);
    if op.s() {
        set_all_flags!(cpu, n, z, c, v);
    }
    if op.rd() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        DispatchRes::RetireOk
    }
}



pub fn rsb_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = sub_generic(val, cpu.reg[op.rn()]);
    if op.s() {
        set_all_flags!(cpu, n, z, c, v);
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
        set_all_flags!(cpu, n, z, c, v);
    }
    if op.rd() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        DispatchRes::RetireOk
    }
}

pub fn sub_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    let rm = if op.rm() == 15 { cpu.read_exec_pc() } else { cpu.reg[op.rm()] };
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = sub_generic(cpu.reg[op.rn()], val);
    if op.s() {
        set_all_flags!(cpu, n, z, c, v);
    }
    if op.rd() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        DispatchRes::RetireOk
    }

}

pub fn mvn_reg(cpu: &mut Cpu, op: MovRegBits) -> DispatchRes {
    let (val, carry) = barrel_shift(ShiftArgs::Reg { rm: cpu.reg[op.rm()],
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c(),
    });
    let res = !val;

    cpu.reg[op.rd()] = res;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    DispatchRes::RetireOk
}



pub fn mvn_imm(cpu: &mut Cpu, op: MovImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);

    let (val, carry) = barrel_shift(ShiftArgs::Imm { 
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c() 
    });
    let res = !val;

    cpu.reg[op.rd()] = res;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    DispatchRes::RetireOk
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
    let rm = if op.rm() == 15 { cpu.read_exec_pc() } else { cpu.reg[op.rm()] };
    let (res, carry) = barrel_shift(ShiftArgs::Reg { rm,
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

pub fn orr_rsr(cpu: &mut Cpu, op: DpRsrBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);

    let (val, carry) = barrel_shift(ShiftArgs::RegShiftReg {
        rm: cpu.reg[op.rm()], 
        stype: op.stype(), 
        rs: cpu.reg[op.rs()],
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


fn do_bitwise_imm(cpu: &mut Cpu, rn: u32, rd: u32, imm: u32, s: bool, op: BitwiseOp) {
    assert_ne!(rd, 15);
    let (val, carry) = barrel_shift(ShiftArgs::Imm { imm12: imm, c_in: cpu.reg.cpsr.c() });
    let base = cpu.reg[rn];
    let res = match op {
        BitwiseOp::And => base & val,
        BitwiseOp::Bic => base & !val,
        BitwiseOp::Orr => base | val,
        _ => panic!("ARM imm bitwise {:?} unimplemented", op),
    };
    if s {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    cpu.reg[rd] = res;
}
pub fn and_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    do_bitwise_imm(cpu, op.rn(), op.rd(), op.imm12(), op.s(), BitwiseOp::And);
    DispatchRes::RetireOk
}
pub fn bic_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    do_bitwise_imm(cpu, op.rn(), op.rd(), op.imm12(), op.s(), BitwiseOp::Bic);
    DispatchRes::RetireOk
}
pub fn orr_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    do_bitwise_imm(cpu, op.rn(), op.rd(), op.imm12(), op.s(), BitwiseOp::Orr);
    DispatchRes::RetireOk
}



pub fn cmn_imm(cpu: &mut Cpu, op: DpTestImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (_, n, z, c, v) = add_generic(cpu.reg[op.rn()], val);
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}


pub fn cmp_imm(cpu: &mut Cpu, op: DpTestImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (_, n, z, c, v) = sub_generic(cpu.reg[op.rn()], val);
    set_all_flags!(cpu, n, z, c, v);
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
    set_all_flags!(cpu, n, z, c, v);
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



