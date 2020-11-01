//! Load/store instructions.

use crate::cpu::*;
use crate::cpu::alu::*;
use crate::cpu::exec::arm::bits::*;

pub fn do_amode(rn: u32, imm: u32, u: bool, p: bool, w: bool) -> (u32, u32) {
    let res = if u { rn.wrapping_add(imm) } else { rn.wrapping_sub(imm) };
    match (p, w) {
        (false, false)  => (rn, res),
        (true, false)   => (res, rn),
        (true, true)    => (res, res),
        (false, true)   => panic!("Unsupported addressing mode?"),
    }
}

pub fn do_amode_lit(pc: u32, imm: u32, p: bool, u: bool) -> u32 {
    match (p, u) {
        (true, true) => pc.wrapping_add(imm),
        (true, false) => pc.wrapping_sub(imm),
        _ => pc
    }
}

pub fn ldrb_imm(cpu: &mut Cpu, op: LsImmBits) -> DispatchRes {
    assert_ne!(op.rt(), 15);
    let res = if op.rn() == 15 {
        assert_eq!(op.w(), false);
        let addr = do_amode_lit(cpu.read_exec_pc(), op.imm12(), op.p(), op.u());
        cpu.mmu.read8(addr)
    } else {
        let (addr, wb_addr) = do_amode(cpu.reg[op.rn()], 
            op.imm12(), op.u(), op.p(), op.w());
        cpu.reg[op.rn()] = wb_addr;
        cpu.mmu.read8(addr)
    };
    cpu.reg[op.rt()] = res as u32;
    DispatchRes::RetireOk
}

pub fn ldr_imm(cpu: &mut Cpu, op: LsImmBits) -> DispatchRes {
    let res = if op.rn() == 15 {
        assert_eq!(op.w(), false);
        let addr = do_amode_lit(cpu.read_exec_pc(), op.imm12(), op.p(), op.u());
        let val = cpu.mmu.read32(addr);
        val
    } else {
        let (addr, wb_addr) = do_amode(cpu.reg[op.rn()], 
            op.imm12(), op.u(), op.p(), op.w());
        cpu.reg[op.rn()] = wb_addr;
        cpu.mmu.read32(addr)
    };
    if op.rt() == 15 {
        cpu.reg.cpsr.set_thumb(res & 1 != 0);
        cpu.write_exec_pc(res & 0xffff_fffe);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rt()] = res;
        DispatchRes::RetireOk
    }
}

pub fn ldr_reg(cpu: &mut Cpu, op: LsRegBits) -> DispatchRes {
    let (offset, _) = barrel_shift(ShiftArgs::Reg { rm: cpu.reg[op.rm()],
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });

    let (addr, wb_addr) = do_amode(cpu.reg[op.rn()], 
        offset, op.u(), op.p(), op.w()
    );
    let val = cpu.mmu.read32(addr);

    cpu.reg[op.rn()] = wb_addr;
    if op.rt() == 15 {
        cpu.write_exec_pc(val);
        DispatchRes::RetireBranch
    } else {
        cpu.reg[op.rt()] = val;
        DispatchRes::RetireOk
    }
}

pub fn str_reg(cpu: &mut Cpu, op: LsRegBits) -> DispatchRes {
    let (offset, _) = barrel_shift(ShiftArgs::Reg { rm: cpu.reg[op.rm()],
        stype: op.stype(), imm5: op.imm5(), c_in: cpu.reg.cpsr.c()
    });

    let (addr, wb_addr) = do_amode(cpu.reg[op.rn()], 
        offset, op.u(), op.p(), op.w()
    );

    let val = cpu.reg[op.rt()];
    cpu.mmu.write32(addr, val);
    cpu.reg[op.rn()] = wb_addr;
    DispatchRes::RetireOk
}



pub fn str_imm(cpu: &mut Cpu, op: LsImmBits) -> DispatchRes {
    let (addr, wb_addr) = do_amode(cpu.reg[op.rn()], 
        op.imm12(), op.u(), op.p(), op.w()
    );
    cpu.reg[op.rn()] = wb_addr;
    cpu.mmu.write32(addr, cpu.reg[op.rt()]);
    DispatchRes::RetireOk
}
pub fn strb_imm(cpu: &mut Cpu, op: LsImmBits) -> DispatchRes {
    let (addr, wb_addr) = do_amode(cpu.reg[op.rn()], 
        op.imm12(), op.u(), op.p(), op.w()
    );
    cpu.reg[op.rn()] = wb_addr;
    cpu.mmu.write8(addr, cpu.reg[op.rt()]);
    DispatchRes::RetireOk
}

pub fn ldm_user(cpu: &mut Cpu, op: LdmRegUserBits) -> DispatchRes {
    assert_ne!(op.rn(), 15);
    let reglist = op.register_list();

    let len = reglist.count_ones() * 4;
    let mut addr = if op.u() { 
        cpu.reg[op.rn()]
    } else {
        cpu.reg[op.rn()] - len
    };
    if op.p() == op.u() {
        addr += 4;
    }

    for i in 0..16 {
        if (reglist & (1 << i)) != 0 {
            let val = cpu.mmu.read32(addr);

            match i {
                13 => cpu.reg.bank.sys[0] = val,
                14 => cpu.reg.bank.sys[1] = val,
                _ => cpu.reg[i as u32] = val,
            }
            addr += 4;
        }
    }
    DispatchRes::RetireOk
}

pub fn ldmib(cpu: &mut Cpu, op: LsMultiBits) -> DispatchRes {
    assert_ne!(op.rn(), 15);
    let reglist = op.register_list();
    let mut addr = cpu.reg[op.rn()] + 4;
    let wb_addr = addr + (reglist.count_ones() * 4);

    for i in 0..16 {
        if (reglist & (1 << i)) != 0 {
            cpu.reg[i as u32] = cpu.mmu.read32(addr);
            addr += 4;
        }
    }
    assert!(addr == wb_addr);
    if op.w() { 
        cpu.reg[op.rn()] = wb_addr;
    }
    DispatchRes::RetireOk
}


pub fn ldmia(cpu: &mut Cpu, op: LsMultiBits) -> DispatchRes {
    assert_ne!(op.rn(), 15);
    let reglist = op.register_list();
    let mut addr = cpu.reg[op.rn()];
    let wb_addr = addr + (reglist.count_ones() * 4);

    for i in 0..16 {
        if (reglist & (1 << i)) != 0 {
            cpu.reg[i as u32] = cpu.mmu.read32(addr);
            addr += 4;
        }
    }
    assert!(addr == wb_addr);
    if op.w() { 
        cpu.reg[op.rn()] = wb_addr;
    }
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
                cpu.reg[i as u32]
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

pub fn stm(cpu: &mut Cpu, op: LsMultiBits) -> DispatchRes {
    assert_ne!(op.rn(), 15);

    let reglist = op.register_list();
    let mut addr = cpu.reg[op.rn()];
    let mut wb_addr = addr + (reglist.count_ones() * 4);

    for i in 0..16 {
        if (reglist & (1 << i)) != 0 {
            let val = if i == 15 {
                cpu.read_exec_pc()
            } else {
                cpu.reg[i as u32]
            };
            cpu.mmu.write32(addr, val);
            addr += 4;
        }
    }

    assert!(addr == wb_addr);
    if op.w() { 
        cpu.reg[op.rn()] = wb_addr;
    }
    DispatchRes::RetireOk
}

