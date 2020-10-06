//! Types used for dispatching instructions.

use crate::cpu::lut::*;
use crate::cpu::decode::*;

use crate::cpu::*;
use crate::cpu::interp;
use crate::cpu::interp::branch;
use crate::cpu::interp::loadstore;
use crate::cpu::interp::dataproc;

/// Result of dispatching an instruction.
#[derive(Debug)]
pub enum DispatchRes {
    /// There was some fatal error dispatching the instruction.
    FatalErr,
    /// This instruction was not executed because the condition failed.
    CondFailed,
    /// This instruction retired and resulted in a branch.
    RetireBranch,
    /// This instruction retired and the PC should be incremented.
    RetireOk,
}


/// A function pointer to an ARM instruction implementation.
#[derive(Clone, Copy)]
pub struct ArmFn(pub fn(&mut Cpu, u32) -> DispatchRes);

/// Implementing [InstLutEntry] maps each instruction to a function.
impl InstLutEntry for ArmFn {
    type Inst = ArmInst;
    fn from_inst(inst: ArmInst) -> Self {
        use ArmInst::*;
        use std::mem::transmute;

        // We use this to coerce the borrow checker into taking pointers to
        // functions which take a newtype wrapping a u32 (for bitfields).
        macro_rules! cfn { ($func:expr) => { unsafe {
            transmute::<*const fn(), fn(&mut Cpu, u32) -> DispatchRes>
                ($func as *const fn())
        }}}

        match inst {
            LdrLit | LdrImm => ArmFn(loadstore::ldr_imm_or_lit),
            SubSpImm | SubImm => ArmFn(cfn!(dataproc::sub_imm)),

            LdrReg      => ArmFn(cfn!(loadstore::ldr_reg)),

            StrImm      => ArmFn(cfn!(loadstore::str_imm)),
            StrbImm     => ArmFn(cfn!(loadstore::strb_imm)),
            Stmdb       => ArmFn(cfn!(loadstore::stmdb)),

            B           => ArmFn(cfn!(branch::b)),
            Bx          => ArmFn(cfn!(branch::bx)),
            BlImm       => ArmFn(cfn!(branch::bl_imm)),

            MovImm      => ArmFn(cfn!(dataproc::mov_imm)),
            MovReg      => ArmFn(cfn!(dataproc::mov_reg)),
            AddImm      => ArmFn(cfn!(dataproc::add_imm)),
            AddReg      => ArmFn(cfn!(dataproc::add_reg)),
            OrrImm      => ArmFn(cfn!(dataproc::orr_imm)),
            OrrReg      => ArmFn(cfn!(dataproc::orr_reg)),
            EorReg      => ArmFn(cfn!(dataproc::eor_reg)),
            AndImm      => ArmFn(cfn!(dataproc::and_imm)),
            CmnImm      => ArmFn(cfn!(dataproc::cmn_imm)),
            CmpImm      => ArmFn(cfn!(dataproc::cmp_imm)),
            CmpReg      => ArmFn(cfn!(dataproc::cmp_reg)),
            TstReg      => ArmFn(cfn!(dataproc::tst_reg)),
            BicImm      => ArmFn(cfn!(dataproc::bic_imm)),
            _ => ArmFn(interp::unimpl_instr),
        }
    }
}

/// An ARMv5 lookup table.
pub struct ArmLut { 
    pub data: [ArmFn; 0x1000] 
}
impl InstLut for ArmLut {
    const LUT_SIZE: usize = 0x1000;
    type Entry = ArmFn;
    type Instr = ArmInst;
    type Index = usize;

    fn lookup(&self, opcd: u32) -> ArmFn { 
        self.data[Self::opcd_to_idx(opcd)] 
    }

    fn idx_to_opcd(idx: usize) -> u32 {
        (((idx & 0x0ff0) << 16) | ((idx & 0x000f) << 4)) as u32
    }

    fn opcd_to_idx(opcd: u32) -> usize {
        (((opcd >> 16) & 0x0ff0) | ((opcd >> 4) & 0x000f)) as usize
    }

    fn create_lut(default_entry: ArmFn) -> Self {
        let mut lut = ArmLut {
            data: [default_entry; 0x1000],
        };
        for i in 0..Self::LUT_SIZE {
            let opcd = ArmLut::idx_to_opcd(i);
            lut.data[i as usize] = ArmFn::from_inst(ArmInst::decode(opcd));
        }
        lut
    }
}

/// Container for lookup tables
pub struct Lut {
    pub arm: ArmLut,
}
impl Lut {
    pub fn new() -> Self {
        Lut {
            arm: ArmLut::create_lut(ArmFn(interp::unimpl_instr)),
        }
    }
}


