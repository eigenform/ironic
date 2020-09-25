use crate::dbg::*;
use crate::cpu::armv5::*;
use crate::cpu::armv5::decode::*;
use crate::cpu::armv5::bits::*;
use crate::cpu::armv5::reg::*;


pub fn ldr_imm_or_lit(cpu: &mut Cpu, op: u32) -> DispatchRes {
    use ArmInst::*;
    match ArmInst::decode(op) {
        LdrLit => return ldr_lit(cpu, LdrLitBits(op)),
        LdrImm => return ldr_imm(cpu, LsImmBits(op)),
        _ => unreachable!(),
    }
}
pub fn ldr_lit(cpu: &mut Cpu, op: LdrLitBits) -> DispatchRes {
    if op.w() {
        return DispatchRes::FatalErr;
    }

    let addr = cpu.read_exec_pc();
    let target_addr = if op.p() { 
        if op.u() { 
            addr.wrapping_add(op.imm12()) 
        } else { 
            addr.wrapping_sub(op.imm12()) 
        }
    } else { 
        addr 
    };
    
    let res = cpu.mmu.read32(target_addr);
    if op.rt() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rt()] = res;
        DispatchRes::RetireOk
    }
}
pub fn ldr_imm(cpu: &mut Cpu, op: LsImmBits) -> DispatchRes {
    DispatchRes::FatalErr
}


