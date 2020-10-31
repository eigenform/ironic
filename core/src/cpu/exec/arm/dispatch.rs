//! Map from instructions to functions in the ARM lookup table.

use crate::cpu::lut::*;
use crate::cpu::exec::arm::*;
use crate::cpu::exec::arm::decode::ArmInst;

/// A function pointer to an ARM instruction implementation.
#[derive(Clone, Copy)]
pub struct ArmFn(pub fn(&mut Cpu, u32) -> DispatchRes);

/// Unimplemented instruction handler.
pub fn unimpl_instr(cpu: &mut Cpu, op: u32) -> DispatchRes {
    log(&cpu.dbg, LogLevel::Cpu, &format!(
        "Couldn't dispatch instruction {:08x} ({:?})", 
        op, ArmInst::decode(op)
    ));
    println!("Couldn't dispatch instruction {:08x} ({:?})",
        op, ArmInst::decode(op));
    DispatchRes::FatalErr
}

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
            MsrImm      => ArmFn(cfn!(status::msr_imm)),
            MsrReg      => ArmFn(cfn!(status::msr_reg)),
            Mrs         => ArmFn(cfn!(status::mrs)),
            Umull       => ArmFn(cfn!(multiply::umull)),
            Mul         => ArmFn(cfn!(multiply::mul)),

            LdrImm      => ArmFn(cfn!(loadstore::ldr_imm)),
            LdrbImm     => ArmFn(cfn!(loadstore::ldrb_imm)),
            SubImm      => ArmFn(cfn!(dataproc::sub_imm)),
            SubReg      => ArmFn(cfn!(dataproc::sub_reg)),

            LdrReg      => ArmFn(cfn!(loadstore::ldr_reg)),
            StrReg      => ArmFn(cfn!(loadstore::str_reg)),

            Ldmib       => ArmFn(cfn!(loadstore::ldmib)),
            Ldm         => ArmFn(cfn!(loadstore::ldmia)),

            StrImm      => ArmFn(cfn!(loadstore::str_imm)),
            StrbImm     => ArmFn(cfn!(loadstore::strb_imm)),
            Stmdb       => ArmFn(cfn!(loadstore::stmdb)),
            Stm         => ArmFn(cfn!(loadstore::stm)),

            Mcr         => ArmFn(cfn!(coproc::mcr)),
            Mrc         => ArmFn(cfn!(coproc::mrc)),

            B           => ArmFn(cfn!(branch::b)),
            Bx          => ArmFn(cfn!(branch::bx)),
            BlImm       => ArmFn(cfn!(branch::bl_imm)),

            RsbImm      => ArmFn(cfn!(dataproc::rsb_imm)),
            RsbReg      => ArmFn(cfn!(dataproc::rsb_reg)),
            MovImm      => ArmFn(cfn!(dataproc::mov_imm)),
            MvnImm      => ArmFn(cfn!(dataproc::mvn_imm)),
            MvnReg      => ArmFn(cfn!(dataproc::mvn_reg)),
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
            TstImm      => ArmFn(cfn!(dataproc::tst_imm)),
            BicImm      => ArmFn(cfn!(dataproc::bic_imm)),

            OrrRegShiftReg => ArmFn(cfn!(dataproc::orr_rsr)),
            _           => ArmFn(unimpl_instr),
        }
    }
}

