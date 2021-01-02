
use ironic_core::cpu::Cpu;
use ironic_core::cpu::reg::CpuMode;
use ironic_core::cpu::alu::*;
use ironic_core::cpu::psr::*;
use crate::bits::arm::*;
use crate::interp::DispatchRes;

pub fn mrs(cpu: &mut Cpu, op: MrsBits) -> DispatchRes {
    let res = match op.r() {
        true =>  cpu.reg.spsr.read(cpu.reg.cpsr.mode()).0,
        false => cpu.reg.cpsr.0,
    };
    cpu.reg[op.rd()] = res;
    DispatchRes::RetireOk
}

pub fn do_msr(cpu: &mut Cpu, val: u32, r: bool, m: u32) -> DispatchRes {
    let mut mask = 0u32;

    // Only privileged modes can write these fields
    if cpu.reg.cpsr.mode().is_privileged() {
        mask |= if (m & 0b0001) != 0 { 0x0000_00ff } else { 0 };
        mask |= if (m & 0b0010) != 0 { 0x0000_ff00 } else { 0 };
        mask |= if (m & 0b0100) != 0 { 0x00ff_0000 } else { 0 };
    }
    // User mode is free to alter the condition bits
    mask |= if (m & 0b1000) != 0 { 0xff00_0000 } else { 0 };

    if r {
        // Write the SPSR for the current mode
        let current_mode = cpu.reg.cpsr.mode();
        assert_ne!(current_mode, CpuMode::Usr);
        assert_ne!(current_mode, CpuMode::Sys);
        let old_spsr = cpu.reg.spsr.read(current_mode);
        let new_spsr = Psr((old_spsr.0 & !mask) | (val & mask));
        cpu.reg.spsr.write(current_mode, new_spsr);
    } else {
        // Write the CPSR
        let old_cpsr = cpu.reg.cpsr;
        let new_cpsr = Psr((old_cpsr.0 & !mask) | (val & mask));
        cpu.reg.write_cpsr(new_cpsr);
    }
    DispatchRes::RetireOk
}

pub fn msr_imm(cpu: &mut Cpu, op: MsrImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });
    do_msr(cpu, val, op.r(), op.mask())
}
pub fn msr_reg(cpu: &mut Cpu, op: MsrRegBits) -> DispatchRes {
    do_msr(cpu, cpu.reg[op.rn()], op.r(), op.mask())
}
