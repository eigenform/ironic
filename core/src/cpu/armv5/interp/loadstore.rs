use crate::dbg::*;
use crate::cpu::armv5::*;
use crate::cpu::armv5::decode::*;
use crate::cpu::armv5::bits::*;
use crate::cpu::armv5::reg::*;

pub fn amode_imm(rn: u32, imm12: u32, u: bool, p: bool, w: bool) -> (u32, u32) {
    let res = if u { rn.wrapping_add(imm12) } else { rn.wrapping_sub(imm12) };
    match (p, w) {
        (false, false)  => (rn, res),
        (true, false)   => (res, rn),
        (true, true)    => (res, res),
        (false, true)   => panic!("Unsupported addressing mode?"),
    }
}

pub fn ldr_imm_or_lit(cpu: &mut Cpu, op: u32) -> DispatchRes {
    use ArmInst::*;
    match ArmInst::decode(op) {
        LdrLit => return ldr_lit(cpu, LdrLitBits(op)),
        LdrImm => return ldr_imm(cpu, LsImmBits(op)),
        _ => unreachable!(),
    }
}
pub fn ldr_lit(cpu: &mut Cpu, op: LdrLitBits) -> DispatchRes {
    assert_eq!(op.w(), false);

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


pub fn str_imm(cpu: &mut Cpu, op: LsImmBits) -> DispatchRes {
    let (addr, wb_addr) = amode_imm(cpu.reg[op.rn()], 
        op.imm12(), op.u(), op.p(), op.w()
    );
    cpu.reg[op.rn()] = wb_addr;
    cpu.mmu.write32(addr, cpu.reg[op.rt()]);
    DispatchRes::RetireOk
}



pub fn stmdb(cpu: &mut Cpu, op: LsMultiBits) -> DispatchRes {
    assert_ne!(op.rn(), 15);
    let reglist = op.register_list();

    let mut addr = cpu.reg[op.rn()] - (reglist.count_ones() * 4);
    let wb_addr = addr;

    for i in 0..16 {
        if (reglist & (1 << i)) != 0 {
            let val = if i == 15 {
                cpu.read_exec_pc()
            } else {
                cpu.reg[i]
            };
            cpu.mmu.write32(addr, val);
            addr += 4;
        }
    }
    if op.w() { 
        cpu.reg[op.rn()] = wb_addr;
    }
    DispatchRes::RetireOk
}
