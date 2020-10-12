
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
