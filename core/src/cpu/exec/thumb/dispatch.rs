
use crate::cpu::exec::thumb::*;
use crate::cpu::exec::thumb::decode::ThumbInst;

#[derive(Clone, Copy)]
pub struct ThumbFn(pub fn(&mut Cpu, u16) -> DispatchRes);

pub fn unimpl_instr(cpu: &mut Cpu, op: u16) -> DispatchRes {
    log(&cpu.dbg, LogLevel::Cpu, &format!(
        "Couldn't dispatch Thumb instruction {:04x} ({:?})",
        op, ThumbInst::decode(op)));
    println!("Couldn't dispatch Thumb instruction {:04x} ({:?})",
        op, ThumbInst::decode(op));
    DispatchRes::FatalErr
}

impl InstLutEntry for ThumbFn {
    type Inst = ThumbInst;
    fn from_inst(inst: ThumbInst) -> Self {
        use ThumbInst::*;
        use std::mem::transmute;

        macro_rules! cfn { ($func:expr) => { unsafe {
            transmute::<*const fn(), fn(&mut Cpu, u16) -> DispatchRes>
                ($func as *const fn())
        }}}

        match inst {
            Push        => ThumbFn(cfn!(loadstore::push)),
            Pop         => ThumbFn(cfn!(loadstore::pop)),
            LdrLit      => ThumbFn(cfn!(loadstore::ldr_lit)),
            LdrImm      => ThumbFn(cfn!(loadstore::ldr_imm)),
            LdrImmAlt   => ThumbFn(cfn!(loadstore::ldr_imm_sp)),
            StrImmAlt   => ThumbFn(cfn!(loadstore::str_imm_sp)),
            StrImm      => ThumbFn(cfn!(loadstore::str_imm)),

            CmpImm      => ThumbFn(cfn!(dataproc::cmp_imm)),
            CmpReg      => ThumbFn(cfn!(dataproc::cmp_reg)),
            MovReg      => ThumbFn(cfn!(dataproc::mov_reg)),
            BicReg      => ThumbFn(cfn!(dataproc::bic_reg)),
            TstReg      => ThumbFn(cfn!(dataproc::tst_reg)),
            MovRegAlt   => ThumbFn(cfn!(dataproc::mov_reg_alt)),
            MovImm      => ThumbFn(cfn!(dataproc::mov_imm)),
            AddRegAlt   => ThumbFn(cfn!(dataproc::add_reg_alt)),
            AddSpImm    => ThumbFn(cfn!(dataproc::add_sp_imm)),
            AddReg      => ThumbFn(cfn!(dataproc::add_reg)),
            SubReg      => ThumbFn(cfn!(dataproc::sub_reg)),
            AddImm      => ThumbFn(cfn!(dataproc::add_imm)),
            SubSpImm    => ThumbFn(cfn!(dataproc::sub_sp_imm)),
            AndReg      => ThumbFn(cfn!(dataproc::and_reg)),
            OrrReg      => ThumbFn(cfn!(dataproc::orr_reg)),
            Mul         => ThumbFn(cfn!(dataproc::mul_reg)),

            BlPrefix    => ThumbFn(cfn!(branch::bl_prefix)),
            BlImmSuffix => ThumbFn(cfn!(branch::bl_imm_suffix)),
            Bx          => ThumbFn(cfn!(branch::bx)),
            B           => ThumbFn(cfn!(branch::b)),
            BAlt        => ThumbFn(cfn!(branch::b_unconditional)),
            _           => ThumbFn(unimpl_instr),
        }
    }
}
