//! Data-processing instructions.

use ironic_core::cpu::Cpu;
use ironic_core::cpu::alu::*;
use crate::bits::arm::*;
use crate::interp::DispatchRes;

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
    if op.rd() == 15 {
        if op.s() {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            set_all_flags!(cpu, n, z, c, v);
        }
        DispatchRes::RetireOk
    }
}

pub fn rsb_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = sub_generic(val, cpu.reg[op.rn()]);
    if op.rd() == 15 {
        if op.s() {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            set_all_flags!(cpu, n, z, c, v);
        }
        DispatchRes::RetireOk
    }
}

pub fn sub_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = sub_generic(cpu.reg[op.rn()], val);
    if op.rd() == 15 {
        if op.s() {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            set_all_flags!(cpu, n, z, c, v);
        }
        DispatchRes::RetireOk
    }
}

pub fn mvn_imm(cpu: &mut Cpu, op: MovImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);

    let (val, carry) = barrel_shift(ShiftArgs::Imm { 
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c() 
    });
    let res = !val;
    if op.rd() == 15 {
        if op.s() {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
            cpu.reg.cpsr.set_z(res == 0);
            cpu.reg.cpsr.set_c(carry);
        }
        DispatchRes::RetireOk
    }
}

pub fn mov_imm(cpu: &mut Cpu, op: MovImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let (res, carry) = barrel_shift(ShiftArgs::Imm { 
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c() 
    });
    if op.rd() == 15 {
        if op.s() {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
            cpu.reg.cpsr.set_z(res == 0);
            cpu.reg.cpsr.set_c(carry);
        }
        DispatchRes::RetireOk
    }
}


pub fn add_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    let rm = if op.rm() == 15 { cpu.read_exec_pc() } else { cpu.reg[op.rm()] };
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = add_generic(cpu.reg[op.rn()], val);
    if op.rd() == 15 {
        if op.s() {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            set_all_flags!(cpu, n, z, c, v);
        }
        DispatchRes::RetireOk
    }
}

pub fn rsb_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm: cpu.reg[op.rm()],
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = sub_generic(val, cpu.reg[op.rn()]);
    if op.rd() == 15 {
        if op.s() {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            set_all_flags!(cpu, n, z, c, v);
        }
        DispatchRes::RetireOk
    }
}

pub fn sub_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    let rm = if op.rm() == 15 { cpu.read_exec_pc() } else { cpu.reg[op.rm()] };
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });
    let (res, n, z, c, v) = sub_generic(cpu.reg[op.rn()], val);
    if op.rd() == 15 {
        if op.s() {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            set_all_flags!(cpu, n, z, c, v);
        }
        DispatchRes::RetireOk
    }

}

pub fn mvn_reg(cpu: &mut Cpu, op: MovRegBits) -> DispatchRes {
    let (val, carry) = barrel_shift(ShiftArgs::Reg { rm: cpu.reg[op.rm()],
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c(),
    });
    let res = !val;
    if op.rd() == 15  {
        if op.s() {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
            cpu.reg.cpsr.set_z(res == 0);
            cpu.reg.cpsr.set_c(carry);
        }
        DispatchRes::RetireOk
    }
}

pub fn mov_reg(cpu: &mut Cpu, op: MovRegBits) -> DispatchRes {
    let rm = if op.rm() == 15 { cpu.read_exec_pc() } else { cpu.reg[op.rm()] };
    let (res, carry) = barrel_shift(ShiftArgs::Reg { rm,
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });
    if op.rd() == 15 {
        if op.s() { 
            //println!("movs r{}, r{} res={:08x}", op.rd(), op.rm(), res);
            cpu.exception_return(res); 
        } else { 
            cpu.write_exec_pc(res); 
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        if op.s() {
            cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
            cpu.reg.cpsr.set_z(res == 0);
            cpu.reg.cpsr.set_c(carry);
        }
        DispatchRes::RetireOk
    }
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


fn do_bitwise_reg(cpu: &mut Cpu, rn: u32, rm: u32, rd: u32, imm5: u32, 
    s: bool, stype: u32, op: BitwiseOp) -> DispatchRes {
    assert_ne!(rd, 15);
    let (val, carry) = barrel_shift(ShiftArgs::Reg {
        rm: cpu.reg[rm], stype, imm5, c_in: cpu.reg.cpsr.c()
    });
    let base = cpu.reg[rn];
    let res = match op {
        BitwiseOp::And => base & val,
        BitwiseOp::Orr => base | val,
        BitwiseOp::Eor => base ^ val,
        _ => panic!("ARM reg bitwise {:?} unimpl", op),
    };
    if rd == 15 {
        if s {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[rd] = res;
        if s {
            cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
            cpu.reg.cpsr.set_z(res == 0);
            cpu.reg.cpsr.set_c(carry);
        }
        DispatchRes::RetireOk
    }
}
pub fn orr_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    do_bitwise_reg(cpu, op.rn(), op.rm(), op.rd(), op.imm5(), 
        op.s(), op.stype(), BitwiseOp::Orr)
}
pub fn eor_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    do_bitwise_reg(cpu, op.rn(), op.rm(), op.rd(), op.imm5(), 
        op.s(), op.stype(), BitwiseOp::Eor)
}
pub fn and_reg(cpu: &mut Cpu, op: DpRegBits) -> DispatchRes {
    do_bitwise_reg(cpu, op.rn(), op.rm(), op.rd(), op.imm5(), 
        op.s(), op.stype(), BitwiseOp::And)
}


fn do_bitwise_imm(cpu: &mut Cpu, rn: u32, rd: u32, imm: u32, 
    s: bool, op: BitwiseOp) -> DispatchRes {
    assert_ne!(rd, 15);
    let (val, carry) = barrel_shift(ShiftArgs::Imm { 
        imm12: imm, c_in: cpu.reg.cpsr.c() 
    });
    let base = cpu.reg[rn];
    let res = match op {
        BitwiseOp::And => base & val,
        BitwiseOp::Bic => base & !val,
        BitwiseOp::Orr => base | val,
        _ => panic!("ARM imm bitwise {:?} unimplemented", op),
    };
    if rd == 15 {
        if s {
            cpu.exception_return(res);
        } else {
            cpu.write_exec_pc(res);
        }
        DispatchRes::RetireBranch
    } else {
        cpu.reg[rd] = res;
        if s {
            cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
            cpu.reg.cpsr.set_z(res == 0);
            cpu.reg.cpsr.set_c(carry);
        }
        DispatchRes::RetireOk
    }
}
pub fn and_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    do_bitwise_imm(cpu, op.rn(), op.rd(), op.imm12(), op.s(), BitwiseOp::And)
}
pub fn bic_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    do_bitwise_imm(cpu, op.rn(), op.rd(), op.imm12(), op.s(), BitwiseOp::Bic)
}
pub fn orr_imm(cpu: &mut Cpu, op: DpImmBits) -> DispatchRes {
    do_bitwise_imm(cpu, op.rn(), op.rd(), op.imm12(), op.s(), BitwiseOp::Orr)
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


pub fn tst_imm(cpu: &mut Cpu, op: DpTestImmBits) -> DispatchRes {
    let (val, carry) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    let res = cpu.reg[op.rn()] & val;
    cpu.reg.cpsr.set_n(res & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(res == 0);
    cpu.reg.cpsr.set_c(carry);
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



