
use crate::cpu::*;
use crate::cpu::reg::*;
use crate::cpu::psr::*;
use crate::cpu::alu::*;
use crate::cpu::exec::arm::bits::*;

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

pub fn msr_imm(cpu: &mut Cpu, op: MsrImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });

    let mut mask = 0u32;
    mask |= if (op.mask() & 0b0001) != 0 { 0x0000_00ff } else { 0 };
    mask |= if (op.mask() & 0b0010) != 0 { 0x0000_ff00 } else { 0 };
    mask |= if (op.mask() & 0b0100) != 0 { 0x00ff_0000 } else { 0 };
    mask |= if (op.mask() & 0b1000) != 0 { 0xff00_0000 } else { 0 };

    let current_mode = cpu.reg.cpsr.mode();
    if op.r() {
        assert_ne!(current_mode, CpuMode::Usr);
        assert_ne!(current_mode, CpuMode::Sys);
        let old_spsr = cpu.reg.spsr.read(current_mode);
        let new_spsr = Psr((old_spsr.0 & !mask) | (val & mask));
        cpu.reg.spsr.write(current_mode, new_spsr);
    } else {
        let old_cpsr = cpu.reg.cpsr;
        let new_cpsr = Psr((old_cpsr.0 & !mask) | (val & mask));
        cpu.reg.write_cpsr(new_cpsr);
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

    let current_mode = cpu.reg.cpsr.mode();
    if op.r() {
        assert_ne!(current_mode, CpuMode::Usr);
        assert_ne!(current_mode, CpuMode::Sys);
        let old_spsr = cpu.reg.spsr.read(current_mode);
        let new_spsr = Psr((old_spsr.0 & !mask) | (val & mask));
        cpu.reg.spsr.write(current_mode, new_spsr);
    } else {
        let old_cpsr = cpu.reg.cpsr;
        let new_cpsr = Psr((old_cpsr.0 & !mask) | (val & mask));
        cpu.reg.write_cpsr(new_cpsr);
    }
    DispatchRes::RetireOk
}
