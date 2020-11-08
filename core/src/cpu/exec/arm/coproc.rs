//! Implementation of co-processor instructions.

use crate::cpu::*;
use crate::cpu::exec::arm::bits::*;

pub fn mcr(cpu: &mut Cpu, op: MoveCoprocBits) -> DispatchRes {
    assert_eq!(op.coproc(), 15);
    cpu.write_p15(cpu.reg[op.rt()], op.crn(), op.crm(), op.opc2());
    DispatchRes::RetireOk
}

pub fn mrc(cpu: &mut Cpu, op: MoveCoprocBits) -> DispatchRes {
    assert_eq!(op.coproc(), 15);
    assert_ne!(op.rt(), 15);
    let val = cpu.read_p15(op.crn(), op.crm(), op.opc2());
    cpu.reg[op.rt()] = val;
    DispatchRes::RetireOk
}
