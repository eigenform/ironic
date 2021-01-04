
use crate::bits::thumb::*;
use crate::interp::DispatchRes;
use ironic_core::cpu::Cpu;
use ironic_core::cpu::alu::*;
use ironic_core::cpu::reg::Reg;


/// Set all of the condition flags.
macro_rules! set_all_flags { 
    ($cpu:ident, $n:ident, $z:ident, $c:ident, $v:ident) => {
        $cpu.reg.cpsr.set_n($n);
        $cpu.reg.cpsr.set_z($z);
        $cpu.reg.cpsr.set_c($c);
        $cpu.reg.cpsr.set_v($v);
    }
}


pub fn mov_rsr(cpu: &mut Cpu, op: MovRsrBits) -> DispatchRes {
    let rm_val = cpu.reg[op.rdm()];
    let rs_val = cpu.reg[op.rs()];
    let stype = ((op.op() & 0b0100) >> 1 | (op.op() & 0b0001)) as u32;

    let (res, carry) = barrel_shift(ShiftArgs::RegShiftReg { 
        rm: rm_val, stype, rs: rs_val, c_in: cpu.reg.cpsr.c()
    });

    cpu.reg[op.rdm()] = res;
    cpu.reg.cpsr.set_n(res & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(res == 0);
    cpu.reg.cpsr.set_c(carry);
    DispatchRes::RetireOk
}

pub fn mov_reg(cpu: &mut Cpu, op: MovRegBits) -> DispatchRes {
    assert_ne!(op.rm(), 15);

    let rd = if op.d() { op.rd() | 0x8 } else { op.rd() };
    let rm_val = cpu.reg[op.rm()];

    if rd == 15 {
        cpu.write_exec_pc(rm_val);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[rd] = rm_val;
        DispatchRes::RetireOk
    }
}

pub fn mvn_reg(cpu: &mut Cpu, op: MvnRegBits) -> DispatchRes {
    let rm = cpu.reg[op.rm()];
    let (val, carry) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: ShiftType::Lsl as u32, imm5: 0, c_in: cpu.reg.cpsr.c()
    });
    let res = !val;
    cpu.reg[op.rd()] = res;
    cpu.reg.cpsr.set_n(res & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(res == 0);
    cpu.reg.cpsr.set_c(carry);
    DispatchRes::RetireOk
}

pub fn mov_reg_alt(cpu: &mut Cpu, op: MovRegAltBits) -> DispatchRes {
    assert!(op.imm5() != 0);
    assert_ne!(op.rd(), 15);
    let rm = cpu.reg[op.rm()];
    let (res, carry) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: op.op() as u32, imm5: op.imm5() as u32, c_in: cpu.reg.cpsr.c()
    });
    cpu.reg[op.rd()] = res;
    cpu.reg.cpsr.set_n(res & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(res == 0);
    cpu.reg.cpsr.set_c(carry);
    DispatchRes::RetireOk
}

pub fn mov_imm(cpu: &mut Cpu, op: MovImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let val = op.imm8() as u32;

    cpu.reg[op.rd()] = val;
    cpu.reg.cpsr.set_n(val & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(val == 0);
    DispatchRes::RetireOk
}


pub fn add_reg_alt(cpu: &mut Cpu, op: AddRegAltBits) -> DispatchRes {
    // ???
    //assert_ne!(op.rm(), 13);

    let rd = if op.dn() { op.rdn() | 0x8 } else { op.rdn() };
    let rn = rd;
    let (alu_out, _n, _z, _c, _v) = add_generic(cpu.reg[rn], cpu.reg[op.rm()]);
    cpu.reg[rd] = alu_out;

    // ???
    //set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn tst_reg(cpu: &mut Cpu, op: CmpRegBits) -> DispatchRes {
    let (val, carry) = barrel_shift(ShiftArgs::Reg {
        rm: cpu.reg[op.rm()], 
        stype: ShiftType::Lsl as u32,
        imm5: 0,
        c_in: cpu.reg.cpsr.c()
    });

    let res = cpu.reg[op.rn()] & val;
    cpu.reg.cpsr.set_n(res & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(res == 0);
    cpu.reg.cpsr.set_c(carry);
    DispatchRes::RetireOk
}

pub fn add_reg(cpu: &mut Cpu, op: AddSubRegBits) -> DispatchRes {
    let rm = cpu.reg[op.rm()];
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: ShiftType::Lsl as u32, imm5: 0, c_in: cpu.reg.cpsr.c()
    });
    let (alu_out, n, z, c, v) = add_generic(cpu.reg[op.rn()], val);
    cpu.reg[op.rd()] = alu_out;
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn sub_reg(cpu: &mut Cpu, op: AddSubRegBits) -> DispatchRes {
    let rm = cpu.reg[op.rm()];
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: ShiftType::Lsl as u32, imm5: 0, c_in: cpu.reg.cpsr.c()
    });
    let (alu_out, n, z, c, v) = sub_generic(cpu.reg[op.rn()], val);
    cpu.reg[op.rd()] = alu_out;
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn sbc_reg(cpu: &mut Cpu, op: BitwiseRegBits) -> DispatchRes {
    let rm = cpu.reg[op.rm()];
    let (shifted, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: ShiftType::Lsl as u32, imm5: 0, c_in: cpu.reg.cpsr.c()
    });
    let val = if cpu.reg.cpsr.c() { shifted } else { shifted + 1 };

    let (alu_out, n, z, c, v) = sub_generic(cpu.reg[op.rdn()], val);
    cpu.reg[op.rdn()] = alu_out;
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn adc_reg(cpu: &mut Cpu, op: BitwiseRegBits) -> DispatchRes {
    let rm = cpu.reg[op.rm()];
    let (shifted, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: ShiftType::Lsl as u32, imm5: 0, c_in: cpu.reg.cpsr.c()
    });
    let val = if cpu.reg.cpsr.c() { shifted + 1 } else { shifted };
    let (alu_out, n, z, c, v) = add_generic(cpu.reg[op.rdn()], val);
    cpu.reg[op.rdn()] = alu_out;
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}



pub fn mul_reg(cpu: &mut Cpu, op: MulBits) -> DispatchRes {
    let rn_val = cpu.reg[op.rn()];
    let rm_val = cpu.reg[op.rdm()];
    let alu_out = rn_val.wrapping_mul(rm_val);
    cpu.reg[op.rdm()] = alu_out;
    cpu.reg.cpsr.set_n(alu_out & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(alu_out == 0);
    DispatchRes::RetireOk
}


pub fn do_bitwise_reg(cpu: &mut Cpu, rm: u16, rdn: u16, op: BitwiseOp) {
    let (val, carry) = barrel_shift(ShiftArgs::Reg { rm: cpu.reg[rm], 
        stype: ShiftType::Lsl as u32, imm5: 0, c_in: cpu.reg.cpsr.c()
    });
    let base = cpu.reg[rdn];
    let res = match op {
        BitwiseOp::And => base & val,
        BitwiseOp::Orr => base | val,
        BitwiseOp::Eor => base ^ val,
        BitwiseOp::Bic => base & !val,
    };
    cpu.reg[rdn] = res;
    cpu.reg.cpsr.set_n(res & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(res == 0);
    cpu.reg.cpsr.set_c(carry);
}
pub fn and_reg(cpu: &mut Cpu, op: BitwiseRegBits) -> DispatchRes {
    do_bitwise_reg(cpu, op.rm(), op.rdn(), BitwiseOp::And);
    DispatchRes::RetireOk
}
pub fn orr_reg(cpu: &mut Cpu, op: BitwiseRegBits) -> DispatchRes {
    do_bitwise_reg(cpu, op.rm(), op.rdn(), BitwiseOp::Orr);
    DispatchRes::RetireOk
}
pub fn eor_reg(cpu: &mut Cpu, op: BitwiseRegBits) -> DispatchRes {
    do_bitwise_reg(cpu, op.rm(), op.rdn(), BitwiseOp::Eor);
    DispatchRes::RetireOk
}
pub fn bic_reg(cpu: &mut Cpu, op: BitwiseRegBits) -> DispatchRes {
    do_bitwise_reg(cpu, op.rm(), op.rdn(), BitwiseOp::Bic);
    DispatchRes::RetireOk
}

pub fn cmp_imm(cpu: &mut Cpu, op: CmpImmBits) -> DispatchRes {
    let (_, n, z, c, v) = sub_generic(cpu.reg[op.rn()], op.imm8() as u32);
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn cmp_reg(cpu: &mut Cpu, op: CmpRegBits) -> DispatchRes {
    let rm = cpu.reg[op.rm()];
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: ShiftType::Lsl as u32, imm5: 0, c_in: cpu.reg.cpsr.c()
    });

    let (_, n, z, c, v) = sub_generic(cpu.reg[op.rn()], val);
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn cmp_reg_alt(cpu: &mut Cpu, op: CmpRegAltBits) -> DispatchRes {
    let rn = if op.n() { op.rn() | 0x8 } else { op.rn() };
    assert!(!(rn < 8 && op.rm() < 8));
    assert!(!(rn == 15 || op.rm() == 15));

    let rm = cpu.reg[op.rm()];
    let (val, _) = barrel_shift(ShiftArgs::Reg { rm, 
        stype: ShiftType::Lsl as u32, imm5: 0, c_in: cpu.reg.cpsr.c()
    });
    let (_, n, z, c, v) = sub_generic(cpu.reg[rn], val);
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

/// NOTE: Not sure this is correct?
pub fn rsb_imm(cpu: &mut Cpu, op: RsbImmBits) -> DispatchRes {
    let rn_val = cpu.reg[op.rn()];
    let (alu_out, n, z, c, v) = sub_generic(0, rn_val);
    cpu.reg[op.rd()] = alu_out;
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn add_imm(cpu: &mut Cpu, op: AddSubImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let rn_val = cpu.reg[op.rn()];
    let imm3 = op.imm3() as u32;
    let (alu_out, n, z, c, v) = add_generic(rn_val, imm3);
    cpu.reg[op.rd()] = alu_out;
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn sub_imm(cpu: &mut Cpu, op: AddSubImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let rn_val = cpu.reg[op.rn()];
    let imm3 = op.imm3() as u32;
    let (alu_out, n, z, c, v) = sub_generic(rn_val, imm3);
    cpu.reg[op.rd()] = alu_out;
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn sub_imm_alt(cpu: &mut Cpu, op: AddSubImmAltBits) -> DispatchRes {
    assert_ne!(op.rdn(), 15);
    let rn_val = cpu.reg[op.rdn()];
    let imm8 = op.imm8() as u32;
    let (alu_out, n, z, c, v) = sub_generic(rn_val, imm8);
    cpu.reg[op.rdn()] = alu_out;
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn add_imm_alt(cpu: &mut Cpu, op: AddSubImmAltBits) -> DispatchRes {
    assert_ne!(op.rdn(), 15);
    let rn_val = cpu.reg[op.rdn()];
    let imm8 = op.imm8() as u32;
    let (alu_out, n, z, c, v) = add_generic(rn_val, imm8);
    cpu.reg[op.rdn()] = alu_out;
    set_all_flags!(cpu, n, z, c, v);
    DispatchRes::RetireOk
}

pub fn sub_sp_imm(cpu: &mut Cpu, op: AddSubSpImmAltBits) -> DispatchRes {
    let imm = (op.imm7() as u32) << 2;
    let res = cpu.reg[Reg::Sp].wrapping_sub(imm);
    cpu.reg[Reg::Sp] = res;
    DispatchRes::RetireOk
}

pub fn add_sp_imm(cpu: &mut Cpu, op: MovImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let imm = (op.imm8() as u32) << 2;
    let res = cpu.reg[Reg::Sp].wrapping_add(imm);
    cpu.reg[op.rd()] = res;
    DispatchRes::RetireOk
}

pub fn add_sp_imm_alt(cpu: &mut Cpu, op: AddSubSpImmAltBits) -> DispatchRes {
    let imm7 = (op.imm7() as u32) << 2;
    let res = cpu.reg[Reg::Sp].wrapping_add(imm7);
    cpu.reg[Reg::Sp] = res;
    DispatchRes::RetireOk
}



