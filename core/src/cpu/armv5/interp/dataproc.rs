
use crate::cpu::armv5::*;
use crate::cpu::armv5::bits::*;
use crate::cpu::armv5::interp::alu::*;


pub fn mov_imm(cpu: &mut Cpu, op: MovSpImmBits) -> DispatchRes {
    if op.rd() == 15 {
        return DispatchRes::FatalErr;
    }

    let (res, carry) = barrel_shift(ShiftArgs::Imm { 
        imm12: op.imm12(), c_in: cpu.reg.cpsr.c() 
    });

    cpu.reg[op.rd()] = res;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    DispatchRes::RetireOk
}


pub fn mov_reg(cpu: &mut Cpu, op: MovSpRegBits) -> DispatchRes {
    let (res, carry) = barrel_shift(ShiftArgs::Reg {
        rm: cpu.reg[op.rm()], 
        stype: op.stype(), 
        imm5: op.imm5(), 
        c_in: cpu.reg.cpsr.c()
    });

    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }

    if op.rd() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rd()] = res;
        DispatchRes::RetireOk
    }
}





