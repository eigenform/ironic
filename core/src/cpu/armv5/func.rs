
use crate::dbg::*;
use crate::cpu::*;
use crate::cpu::armv5::*;
use crate::cpu::armv5::decode::*;
use crate::cpu::armv5::bits::*;


/// Unimplemented instruction handler.
pub fn unimpl_instr(cpu: &mut Cpu, op: u32) -> DispatchRes {
    log(&cpu.dbg, LogLevel::Cpu, &format!(
        "Couldn't dispatch instruction {:08x} ({:?})", 
        op, ArmInst::decode(op)
    ));
    DispatchRes::FatalErr
}


enum ShiftType { 
    Lsl = 0b00, 
    Lsr = 0b01, 
    Asr = 0b10, 
    Ror = 0b11, 
}

fn shift_and_carry(val: u32, stype: ShiftType, shift_imm: u32, c_in: bool) 
    -> (u32, bool) {
    use ShiftType::*;
    if shift_imm == 0 {
        return (val, c_in);
    }
    let (res, c_out) = match stype {
        Lsl => {
            let res = val << shift_imm;
            let c = (1 << (31 - shift_imm) & res) != 0;
            (res, c)
        },
        Lsr => {
            let res = val >> shift_imm;
            let c = (1 << (shift_imm - 1) & res) != 0;
            (res, c)
        },
        Asr => {
            let res = ((val as i32) >> shift_imm) as u32;
            let c = (1 << (shift_imm - 1) & res) != 0;
            (res, c)
        },
        Ror => {
            let res = val.rotate_right(shift_imm);
            let c = (1 << (shift_imm - 1) & res) != 0;
            (res, c)
        },
    };
    (res, c_out)
}
fn compute_imm_carry(imm12: u32, c_in: bool) -> (u32, bool) {
    let (shift_imm, val) = ((imm12 & 0xf00) >> 12, imm12 & 0xff);
    shift_and_carry(val, ShiftType::Ror, shift_imm * 2, c_in)
}
fn compute_imm(imm12: u32, c_in: bool) -> u32 {
    let (shift_imm, val) = ((imm12 & 0xf00) >> 12, imm12 & 0xff);
    let (res, _) = shift_and_carry(val, ShiftType::Ror, shift_imm * 2, c_in);
    res
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


pub fn mov_imm(cpu: &mut Cpu, op: MovSpImmBits) -> DispatchRes {
    if op.rd() == 15 {
        return DispatchRes::FatalErr;
    }

    let (res, carry) = compute_imm_carry(op.imm12(), cpu.reg.cpsr.c());
    cpu.reg[op.rd()] = res;
    if op.s() {
        cpu.reg.cpsr.set_n((res & 0x8000_0000) != 0);
        cpu.reg.cpsr.set_z(res == 0);
        cpu.reg.cpsr.set_c(carry);
    }
    DispatchRes::RetireOk
}
