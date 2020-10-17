
use crate::cpu::*;
use crate::cpu::reg::*;
use crate::cpu::exec::thumb::bits::*;

pub fn push(cpu: &mut Cpu, op: PushBits) -> DispatchRes {
    let num_regs = if op.m() {
        op.register_list().count_ones() + 1
    } else {
        op.register_list().count_ones()
    };

    let start_addr = cpu.reg[Reg::Sp] - (4 * num_regs);
    let end_addr = cpu.reg[Reg::Sp] - 4;
    let mut addr = start_addr;
    for i in 0..8 {
        if (op.register_list() & (1 << i)) != 0 {
            cpu.mmu.write32(addr, cpu.reg[i as u32]);
            addr += 4;
        }
    }
    if op.m() {
        cpu.mmu.write32(addr, cpu.reg[Reg::Lr]);
        addr += 4;
    }
    assert!(end_addr == addr - 4);
    cpu.reg[Reg::Sp] = start_addr;

    DispatchRes::RetireOk
}


pub fn ldr_lit(cpu: &mut Cpu, op: LoadStoreAltBits) -> DispatchRes {
    let imm = (op.imm8() * 4) as u32;
    let addr = (cpu.read_exec_pc() & 0xffff_fffc).wrapping_add(imm);

    let res = cpu.mmu.read32(addr);
    if op.rt() == 15 {
        cpu.write_exec_pc(res);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rt()] = res;
        DispatchRes::RetireOk
    }
}


pub fn ldr_imm(cpu: &mut Cpu, op: LoadStoreImmBits) -> DispatchRes {
    let imm = (op.imm5() as u32) << 2;
    let addr = cpu.reg[op.rn()].wrapping_add(imm);
    let res = cpu.mmu.read32(addr);
    cpu.reg[op.rt()] = res;
    DispatchRes::RetireOk
}

pub fn ldr_imm_sp(cpu: &mut Cpu, op: LoadStoreAltBits) -> DispatchRes {
    let imm = (op.imm8() as u32) << 2;
    let addr = cpu.reg[Reg::Sp].wrapping_add(imm);
    let res = cpu.mmu.read32(addr);
    cpu.reg[op.rt()] = res;
    DispatchRes::RetireOk
}


pub fn str_imm(cpu: &mut Cpu, op: LoadStoreImmBits) -> DispatchRes {
    let imm = (op.imm5() as u32) << 2;
    let addr = cpu.reg[op.rn()].wrapping_add(imm);
    let val = cpu.reg[op.rt()];
    cpu.mmu.write32(addr, val);
    DispatchRes::RetireOk
}
