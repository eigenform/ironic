use crate::cpu::*;

pub fn unimpl_instr() {
    panic!("Unimplemented instruction!")
}

/// Function pointer to an ARM instruction implementation.
#[derive(Clone, Copy)]
pub struct ArmFn(pub fn());
impl InstLutEntry for ArmFn {
    type Inst = ArmInst;
    fn from_inst(inst: ArmInst) -> Self {
        match inst {
            _ => ArmFn(unimpl_instr),
        }
    }
}

/// Enumerated type describing different kinds of ARM instruction encodings.
pub enum ArmInst {
    SubSpReg, AddSpReg, 

    SbcReg, OrrReg, BicReg, AddReg, RscReg, EorReg, MvnReg, AdcReg,
    SubReg, MovReg, AndReg, RsbReg, CmpReg, TstReg, CmnReg, TeqReg,

    MovImm, AdcImm, RsbImm, OrrImm, BicImm, MvnImm, AndImm, RscImm, 
    EorImm, SbcImm, MovImmAlt, CmnImm, CmpImm, TstImm, TeqImm,

    AndRegShiftReg, AdcRegShiftReg, MovRegShiftReg, OrrRegShiftReg,
    EorRegShiftReg, RscRegShiftReg, MvnRegShiftReg, SbcRegShiftReg,
    AddRegShiftReg, BicRegShiftReg, RsbRegShiftReg, SubRegShiftReg,
    TeqRegShiftReg, CmnRegShiftReg, TstRegShiftReg, CmpRegShiftReg,

    StrdReg, StrhReg, StrReg, StrbReg, 
    StrImm, StrbImm, StrdImm, StrhImm, 

    LdrReg, LdrbReg, LdrsbReg, LdrshReg, LdrdReg, LdrhReg, 

    Stm, StmRegUser, Stmda, Ldmda, Ldmib, Ldmdb, Ldm, Stmdb, Stmib,
    LdrbtAlt, StrbtAlt, LdrtAlt, StrtAlt, Ldrbt, Strbt, Ldrt, Strt, Stc, 

    Smull, Umlal, Smlal, Umull, Smlalbb, Smlabb, Smulbb,
    Mul, Mla, Smulwb, Smlawb, Qdadd, Qsub, Qadd, Qdsub,

    // All of these pairs share the same LUT index
    LdcLit, LdcImm,
    MsrReg, MsrRegBanked, 
    Mrs, MrsRegBanked, 
    PldReg, PldImm,
    LdrsbLit, LdrsbImm, 
    LdrshLit, LdrshImm, 
    LdrhLit, LdrhImm,
    LdrdLit, LdrdImm, 
    AddSpImm, AddImm, 
    SubSpImm, SubImm, 
    LdmRegUser, LdmRegException,
    LdrLit, LdrImm, 
    LdrbLit, LdrbImm, 
    B, BlImmAlt, 

    Svc, Clz, PldLit, MsrImm, Mrc, Mcr, Mcrr, Mrrc,
    BlImm, Bx, Bxj, Bkpt, BlxReg,
    Undefined,
}
impl Instruction for ArmInst {
    type Opcd = u32;
    fn decode(opcd: u32) -> Self {
        match opcd & 0x0e5fff00 {
            0x0c1f5e00 => return ArmInst::LdcLit,
            _ => {}
        }
        match opcd & 0xff3f0000 {
            0xf51f0000 => return ArmInst::PldLit,
            _ => {}
        }
        match opcd & 0x0e5f00f0 {
            0x004f00d0 => return ArmInst::LdrdLit,
            0x005f00b0 => return ArmInst::LdrhLit,
            0x005f00f0 => return ArmInst::LdrshLit,
            0x005f00d0 => return ArmInst::LdrsbLit,
            _ => {}
        }
        match opcd & 0x0e50ff00 {
            0x0c005e00 => return ArmInst::Stc,
            0x0c105e00 => return ArmInst::LdcImm,
            _ => {}
        }
        match opcd & 0x0fef0010 {
            0x004d0000 => return ArmInst::SubSpReg,
            0x008d0000 => return ArmInst::AddSpReg,
            _ => {}
        }
        match opcd & 0x0ff000f0 {
            0x01400050 => return ArmInst::Qdadd,
            0x01200050 => return ArmInst::Qsub,
            0x01000050 => return ArmInst::Qadd,
            0x01600050 => return ArmInst::Qdsub,
            0x01200010 => return ArmInst::Bx,
            0x01600010 => return ArmInst::Clz,
            0x01200020 => return ArmInst::Bxj,
            0x01200070 => return ArmInst::Bkpt,
            0x01200030 => return ArmInst::BlxReg,
            _ => {}
        }
        match opcd & 0x0fb002f0 {
            0x01200000 => return ArmInst::MsrReg,
            0x01000200 => return ArmInst::MrsRegBanked,
            0x01200200 => return ArmInst::MsrRegBanked,
            0x01000000 => return ArmInst::Mrs,
            _ => {}
        }
        match opcd & 0x0fe000f0 {
            0x00c00090 => return ArmInst::Smull,
            0x00a00090 => return ArmInst::Umlal,
            0x00e00090 => return ArmInst::Smlal,
            0x00800090 => return ArmInst::Umull,
            0x00000090 => return ArmInst::Mul,
            0x00200090 => return ArmInst::Mla,
            _ => {}
        }
        match opcd & 0x0fef0000 {
            0x028d0000 => return ArmInst::AddSpImm,
            0x024d0000 => return ArmInst::SubSpImm,
            _ => {}
        }
        match opcd & 0xff300010 {
            0xf7100000 => return ArmInst::PldReg,
            _ => {}
        }
        match opcd & 0x0ff00e00 {
            0x0c400e00 => return ArmInst::Mcrr,
            0x0c500e00 => return ArmInst::Mrrc,
            _ => {}
        }
        match opcd & 0x0ff000b0 {
            0x012000a0 => return ArmInst::Smulwb,
            0x01200080 => return ArmInst::Smlawb,
            _ => {}
        }
        match opcd & 0x0ff00090 {
            0x01400080 => return ArmInst::Smlalbb,
            0x01300010 => return ArmInst::TeqRegShiftReg,
            0x01700010 => return ArmInst::CmnRegShiftReg,
            0x01100010 => return ArmInst::TstRegShiftReg,
            0x01500010 => return ArmInst::CmpRegShiftReg,
            0x01000080 => return ArmInst::Smlabb,
            0x01600080 => return ArmInst::Smulbb,
            _ => {}
        }
        match opcd & 0xff300000 {
            0xf5100000 => return ArmInst::PldImm,
            _ => {}
        }
        match opcd & 0x0e5000f0 {
            0x005000d0 => return ArmInst::LdrsbImm,
            0x000000f0 => return ArmInst::StrdReg,
            0x004000b0 => return ArmInst::StrhImm,
            0x001000d0 => return ArmInst::LdrsbReg,
            0x005000f0 => return ArmInst::LdrshImm,
            0x001000f0 => return ArmInst::LdrshReg,
            0x004000f0 => return ArmInst::StrdImm,
            0x005000b0 => return ArmInst::LdrhImm,
            0x000000d0 => return ArmInst::LdrdReg,
            0x001000b0 => return ArmInst::LdrhReg,
            0x004000d0 => return ArmInst::LdrdImm,
            0x000000b0 => return ArmInst::StrhReg,
            _ => {}
        }
        match opcd & 0x0fe00090 {
            0x00000010 => return ArmInst::AndRegShiftReg,
            0x00a00010 => return ArmInst::AdcRegShiftReg,
            0x01a00010 => return ArmInst::MovRegShiftReg,
            0x01800010 => return ArmInst::OrrRegShiftReg,
            0x00200010 => return ArmInst::EorRegShiftReg,
            0x00e00010 => return ArmInst::RscRegShiftReg,
            0x01e00010 => return ArmInst::MvnRegShiftReg,
            0x00c00010 => return ArmInst::SbcRegShiftReg,
            0x00800010 => return ArmInst::AddRegShiftReg,
            0x01c00010 => return ArmInst::BicRegShiftReg,
            0x00600010 => return ArmInst::RsbRegShiftReg,
            0x00400010 => return ArmInst::SubRegShiftReg,
            _ => {}
        }
        match opcd & 0x0ff00010 {
            0x01500000 => return ArmInst::CmpReg,
            0x01100000 => return ArmInst::TstReg,
            0x01700000 => return ArmInst::CmnReg,
            0x01300000 => return ArmInst::TeqReg,
            _ => {}
        }
        match opcd & 0x0e5f0000 {
            0x045f0000 => return ArmInst::LdrbLit,
            0x041f0000 => return ArmInst::LdrLit,
            _ => {}
        }
        match opcd & 0x0f100e10 {
            0x0e100e10 => return ArmInst::Mrc,
            0x0e000e10 => return ArmInst::Mcr,
            _ => {}
        }
        match opcd & 0x0ff00000 {
            0x03000000 => return ArmInst::MovImmAlt,
            0x03700000 => return ArmInst::CmnImm,
            0x03500000 => return ArmInst::CmpImm,
            0x03100000 => return ArmInst::TstImm,
            0x03300000 => return ArmInst::TeqImm,
            _ => {}
        }
        match opcd & 0x0f700010 {
            0x06700000 => return ArmInst::LdrbtAlt,
            0x06600000 => return ArmInst::StrbtAlt,
            0x06300000 => return ArmInst::LdrtAlt,
            0x06200000 => return ArmInst::StrtAlt,
            _ => {}
        }
        match opcd & 0x0fe00010 {
            0x00c00000 => return ArmInst::SbcReg,
            0x01800000 => return ArmInst::OrrReg,
            0x01c00000 => return ArmInst::BicReg,
            0x00800000 => return ArmInst::AddReg,
            0x00e00000 => return ArmInst::RscReg,
            0x00200000 => return ArmInst::EorReg,
            0x01e00000 => return ArmInst::MvnReg,
            0x00a00000 => return ArmInst::AdcReg,
            0x00400000 => return ArmInst::SubReg,
            0x01a00000 => return ArmInst::MovReg,
            0x00000000 => return ArmInst::AndReg,
            0x00600000 => return ArmInst::RsbReg,
            _ => {}
        }
        match opcd & 0x0fe00000 {
            0x03a00000 => return ArmInst::MovImm,
            0x02800000 => return ArmInst::AddImm,
            0x02a00000 => return ArmInst::AdcImm,
            0x02600000 => return ArmInst::RsbImm,
            0x03800000 => return ArmInst::OrrImm,
            0x03c00000 => return ArmInst::BicImm,
            0x02400000 => return ArmInst::SubImm,
            0x03e00000 => return ArmInst::MvnImm,
            0x02000000 => return ArmInst::AndImm,
            0x02e00000 => return ArmInst::RscImm,
            0x02200000 => return ArmInst::EorImm,
            0x02c00000 => return ArmInst::SbcImm,
            _ => {}
        }
        match opcd & 0x0f700000 {
            0x04700000 => return ArmInst::Ldrbt,
            0x04600000 => return ArmInst::Strbt,
            0x04300000 => return ArmInst::Ldrt,
            0x04200000 => return ArmInst::Strt,
            _ => {}
        }
        match opcd & 0x0fd00000 {
            0x08800000 => return ArmInst::Stm,
            0x08000000 => return ArmInst::Stmda,
            0x08100000 => return ArmInst::Ldmda,
            0x09900000 => return ArmInst::Ldmib,
            0x09100000 => return ArmInst::Ldmdb,
            0x08900000 => return ArmInst::Ldm,
            0x09000000 => return ArmInst::Stmdb,
            0x09800000 => return ArmInst::Stmib,
            _ => {}
        }
        match opcd & 0x0fb00000 {
            0x03200000 => return ArmInst::MsrImm,
            _ => {}
        }
        match opcd & 0xfe000000 {
            0xfa000000 => return ArmInst::BlImmAlt,
            _ => {}
        }
        match opcd & 0x0e500010 {
            0x06100000 => return ArmInst::LdrReg,
            0x06400000 => return ArmInst::StrbReg,
            0x06500000 => return ArmInst::LdrbReg,
            0x06000000 => return ArmInst::StrReg,
            _ => {}
        }
        match opcd & 0x0e508000 {
            0x08500000 => return ArmInst::LdmRegUser,
            0x08508000 => return ArmInst::LdmRegException,
            _ => {}
        }
        match opcd & 0x0e500000 {
            0x04000000 => return ArmInst::StrImm,
            0x04400000 => return ArmInst::StrbImm,
            0x04500000 => return ArmInst::LdrbImm,
            0x08400000 => return ArmInst::StmRegUser,
            0x04100000 => return ArmInst::LdrImm,
            _ => {}
        }
        match opcd & 0x0f000000 {
            0x0f000000 => return ArmInst::Svc,
            0x0a000000 => return ArmInst::B,
            0x0b000000 => return ArmInst::BlImm,
            _ => {}
        }
        ArmInst::Undefined
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

    fn lookup(&self, idx: usize) -> ArmFn { 
        self.data[idx] 
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


