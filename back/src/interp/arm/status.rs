
use ironic_core::cpu::Cpu;
use ironic_core::cpu::reg::CpuMode;
use ironic_core::cpu::alu::*;
use ironic_core::cpu::psr::*;
use crate::bits::arm::*;
use crate::interp::DispatchRes;

pub fn mrs(cpu: &mut Cpu, op: MrsBits) -> DispatchRes {
    let res = match op.r() {
        // Read the SPSR of the current mode
        true =>  cpu.reg.spsr.read(cpu.reg.cpsr.mode()).0,
        // Read the CPSR
        false => cpu.reg.cpsr.0,
    };
    cpu.reg[op.rd()] = res;
    DispatchRes::RetireOk
}

fn write_spsr(cpu: &mut Cpu, mask: u32, val: u32) {
    let current_mode = cpu.reg.cpsr.mode();
    // These modes do not have SPSRs; the behavior is unpredictable 
    assert_ne!(current_mode, CpuMode::Usr);
    assert_ne!(current_mode, CpuMode::Sys);
    let old_spsr = cpu.reg.spsr.read(current_mode);
    let new_spsr = Psr((old_spsr.0 & !mask) | (val & mask));
    cpu.reg.spsr.write(current_mode, new_spsr);
}

fn write_cpsr(cpu: &mut Cpu, mask: u32, val: u32) {
    let old_cpsr = cpu.reg.cpsr;
    let new_cpsr = Psr((old_cpsr.0 & !mask) | (val & mask));
    cpu.reg.write_cpsr(new_cpsr);
}

pub fn msr_imm(cpu: &mut Cpu, op: MsrImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });

    let mut mask = 0u32;
    mask |= if (op.mask() & 0b0001) != 0 { 0x0000_00ff } else { 0 };
    mask |= if (op.mask() & 0b0010) != 0 { 0x0000_ff00 } else { 0 };
    mask |= if (op.mask() & 0b0100) != 0 { 0x00ff_0000 } else { 0 };
    mask |= if (op.mask() & 0b1000) != 0 { 0xff00_0000 } else { 0 };

    if op.r() {
        write_spsr(cpu, mask, val);
    } else {
        write_cpsr(cpu, mask, val);
    }
    DispatchRes::RetireOk
}


pub fn msr_reg(cpu: &mut Cpu, op: MsrRegBits) -> DispatchRes {
    let val = cpu.reg[op.rn()];
    let mut mask = 0u32;
    mask |= if (op.mask() & 0b0001) != 0 { 0x0000_00ff } else { 0 };
    mask |= if (op.mask() & 0b0010) != 0 { 0x0000_ff00 } else { 0 };
    mask |= if (op.mask() & 0b0100) != 0 { 0x00ff_0000 } else { 0 };
    mask |= if (op.mask() & 0b1000) != 0 { 0xff00_0000 } else { 0 };
    if op.r() {
        write_spsr(cpu, mask, val);
    } else {
        write_cpsr(cpu, mask, val);
    }
    DispatchRes::RetireOk
}
