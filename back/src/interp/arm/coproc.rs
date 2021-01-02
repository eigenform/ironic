//! Implementation of co-processor instructions.

use ironic_core::cpu::Cpu;
use crate::bits::arm::*;
use crate::interp::DispatchRes;

pub fn mcr(cpu: &mut Cpu, op: MoveCoprocBits) -> DispatchRes {
    assert_eq!(op.coproc(), 15);
    cpu.p15.write(cpu.reg[op.rt()], op.crn(), op.crm(), op.opc2());
    DispatchRes::RetireOk
}

pub fn mrc(cpu: &mut Cpu, op: MoveCoprocBits) -> DispatchRes {
    assert_eq!(op.coproc(), 15);
    if op.rt() != 15 {
        let val = cpu.p15.read(op.crn(), op.crm(), op.opc2());
        cpu.reg[op.rt()] = val;
    } else {
        let val = cpu.p15.read_alt(op.crn(), op.crm(), op.opc2());
        if val.n.is_some() { cpu.reg.cpsr.set_n(val.n.unwrap()); }
        if val.z.is_some() { cpu.reg.cpsr.set_z(val.z.unwrap()); }
        if val.c.is_some() { cpu.reg.cpsr.set_c(val.c.unwrap()); }
        if val.v.is_some() { cpu.reg.cpsr.set_v(val.v.unwrap()); }
    }
    DispatchRes::RetireOk
}
