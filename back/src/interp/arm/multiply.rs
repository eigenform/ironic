
use ironic_core::cpu::Cpu;
use crate::bits::arm::*;
use crate::interp::DispatchRes;

pub fn umull(cpu: &mut Cpu, op: SignedMlBits) -> DispatchRes {
    let rm_val = cpu.reg[op.rm()] as usize;
    let rn_val = cpu.reg[op.rn()] as usize;
    let res = rm_val * rn_val;

    let res_hi = ((res & 0xffffffff_00000000) >> 32) as u32;
    let res_lo =  (res & 0x00000000_ffffffff) as u32;
    cpu.reg[op.rdhi()] = res_hi;
    cpu.reg[op.rdlo()] = res_lo;
    if op.s() {
        cpu.reg.cpsr.set_n((res_hi & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z((res_hi == 0) && (res_lo == 0));
    }
    DispatchRes::RetireOk
}


pub fn mul(cpu: &mut Cpu, op: MulBits) -> DispatchRes {
    let rm_val = cpu.reg[op.rm()] as usize;
    let rn_val = cpu.reg[op.rn()] as usize;
    let res = ((rm_val * rn_val) & 0x00000000_ffffffff) as u32;
    cpu.reg[op.rd()] = res;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
    }
    DispatchRes::RetireOk
}
