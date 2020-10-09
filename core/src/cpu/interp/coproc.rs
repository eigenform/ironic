
use crate::cpu::*;
use crate::cpu::decode::*;
use crate::cpu::dispatch::*;
use crate::cpu::bits::*;

pub fn mcr(cpu: &mut Cpu, op: MoveCoprocBits) -> DispatchRes {
    assert_eq!(op.coproc(), 15);
    cpu.p15.write(cpu.reg[op.rt()], op.crn(), op.crm(), op.opc2());
    DispatchRes::RetireOk
}

pub fn mrc(cpu: &mut Cpu, op: MoveCoprocBits) -> DispatchRes {
    assert_eq!(op.coproc(), 15);
    assert_ne!(op.rt(), 15);
    let val = cpu.p15.read(op.crn(), op.crm(), op.opc2());
    cpu.reg[op.rt()] = val;
    DispatchRes::RetireOk
}
