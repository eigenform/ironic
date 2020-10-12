
use crate::cpu::*;
use crate::cpu::alu::*;
use crate::cpu::exec::thumb::bits::*;

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

pub fn mov_imm(cpu: &mut Cpu, op: MovImmBits) -> DispatchRes {
    assert_ne!(op.rd(), 15);
    let val = op.imm8() as u32;

    cpu.reg[op.rd()] = val;
    cpu.reg.cpsr.set_n(val & 0x8000_0000 != 0);
    cpu.reg.cpsr.set_z(val == 0);
    DispatchRes::RetireOk
}

pub fn add_reg_alt(cpu: &mut Cpu, op: AddRegAltBits) -> DispatchRes {
    assert_ne!(op.rm(), 13);
    let rd = if op.dn() { op.rdn() | 0x8 } else { op.rdn() };
    let rn = rd;
    let (alu_out, n, z, c, v) = add_generic(cpu.reg[rn], cpu.reg[op.rm()]);
    cpu.reg[rd] = alu_out;
    cpu.reg.cpsr.set_n(n);
    cpu.reg.cpsr.set_z(z);
    cpu.reg.cpsr.set_c(c);
    cpu.reg.cpsr.set_v(v);
    DispatchRes::RetireOk
}
