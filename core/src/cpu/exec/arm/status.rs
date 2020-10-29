
use crate::cpu::*;
use crate::cpu::reg::*;
use crate::cpu::alu::*;
use crate::cpu::exec::arm::bits::*;

pub fn msr_imm(cpu: &mut Cpu, op: MsrImmBits) -> DispatchRes {
    let (val, _) = barrel_shift(ShiftArgs::Imm {
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c()
    });

    let mut mask = 0u32;
    mask |= if (op.mask() & 0b0001) != 0 { 0x0000_00ff } else { 0 };
    mask |= if (op.mask() & 0b0010) != 0 { 0x0000_ff00 } else { 0 };
    mask |= if (op.mask() & 0b0100) != 0 { 0x00ff_0000 } else { 0 };
    mask |= if (op.mask() & 0b1000) != 0 { 0xff00_0000 } else { 0 };

    let current_mode = cpu.reg.mode;
    if op.r() {
        assert_ne!(current_mode, CpuMode::Usr);
        assert_ne!(current_mode, CpuMode::Sys);
        let old_spsr = cpu.reg.read_spsr(current_mode);
        let new_spsr = Psr((old_spsr.0 & !mask) | (val & mask));
        cpu.reg.write_spsr(current_mode, new_spsr);
    } else {
        let old_cpsr = cpu.reg.read_cpsr();
        let new_cpsr = Psr((old_cpsr.0 & !mask) | (val & mask));
        cpu.reg.write_cpsr(new_cpsr);
    }
    DispatchRes::RetireOk
}
