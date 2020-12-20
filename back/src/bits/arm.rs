//! Wrapper types for representing ARM instructions as bitfields.

/// ['Stc', 'LdcImm']
#[repr(transparent)]
pub struct LsCoprocBits(pub u32);
impl LsCoprocBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm8(&self) -> u32 { (self.0 & 0x000000ff) >> 0 }
}

/// ['MvnReg', 'MovReg']
#[repr(transparent)]
pub struct MovRegBits(pub u32);
impl MovRegBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm5(&self) -> u32 { (self.0 & 0x00000f80) >> 7 }
    #[inline(always)]
    pub fn stype(&self) -> u32 { (self.0 & 0x00000060) >> 5 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Qdadd', 'Qsub', 'Qadd', 'Qdsub']
#[repr(transparent)]
pub struct QBits(pub u32);
impl QBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Bx', 'Bxj', 'BlxReg']
#[repr(transparent)]
pub struct BxBits(pub u32);
impl BxBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Clz']
#[repr(transparent)]
pub struct ClzBits(pub u32);
impl ClzBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Bkpt']
#[repr(transparent)]
pub struct BkptBits(pub u32);
impl BkptBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x000fff00) >> 8 }
    #[inline(always)]
    pub fn imm4(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['MsrReg']
#[repr(transparent)]
pub struct MsrRegBits(pub u32);
impl MsrRegBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn r(&self) -> bool { (self.0 & 0x00400000) != 0 }
    #[inline(always)]
    pub fn mask(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['MrsRegBanked']
#[repr(transparent)]
pub struct MrsRegBankedBits(pub u32);
impl MrsRegBankedBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn r(&self) -> bool { (self.0 & 0x00400000) != 0 }
    #[inline(always)]
    pub fn m1(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn m(&self) -> bool { (self.0 & 0x00000100) != 0 }
}

/// ['MsrRegBanked']
#[repr(transparent)]
pub struct MsrRegBankedBits(pub u32);
impl MsrRegBankedBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn r(&self) -> bool { (self.0 & 0x00400000) != 0 }
    #[inline(always)]
    pub fn m1(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn m(&self) -> bool { (self.0 & 0x00000100) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Mrs']
#[repr(transparent)]
pub struct MrsBits(pub u32);
impl MrsBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn r(&self) -> bool { (self.0 & 0x00400000) != 0 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
}

/// ['Smull', 'Umlal', 'Smlal', 'Umull']
#[repr(transparent)]
pub struct SignedMlBits(pub u32);
impl SignedMlBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rdhi(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rdlo(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Mul']
#[repr(transparent)]
pub struct MulBits(pub u32);
impl MulBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Mla']
#[repr(transparent)]
pub struct MlaBits(pub u32);
impl MlaBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn ra(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['MovImm', 'MvnImm']
#[repr(transparent)]
pub struct MovImmBits(pub u32);
impl MovImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

/// ['PldReg']
#[repr(transparent)]
pub struct PldRegBits(pub u32);
impl PldRegBits {
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn r(&self) -> bool { (self.0 & 0x00400000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm5(&self) -> u32 { (self.0 & 0x00000f80) >> 7 }
    #[inline(always)]
    pub fn stype(&self) -> u32 { (self.0 & 0x00000060) >> 5 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Mcrr', 'Mrrc']
#[repr(transparent)]
pub struct MoveCoprocDoubleBits(pub u32);
impl MoveCoprocDoubleBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rt2(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn coproc(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn opc1(&self) -> u32 { (self.0 & 0x000000f0) >> 4 }
    #[inline(always)]
    pub fn crm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Smulwb']
#[repr(transparent)]
pub struct SmulwbBits(pub u32);
impl SmulwbBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn m(&self) -> bool { (self.0 & 0x00000040) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Smlawb']
#[repr(transparent)]
pub struct SmlawbBits(pub u32);
impl SmlawbBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn ra(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn m(&self) -> bool { (self.0 & 0x00000040) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Smlalbb']
#[repr(transparent)]
pub struct SmalbbBits(pub u32);
impl SmalbbBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rdhi(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rdlo(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn m(&self) -> bool { (self.0 & 0x00000040) != 0 }
    #[inline(always)]
    pub fn n(&self) -> bool { (self.0 & 0x00000020) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['TeqRegShiftReg', 'CmnRegShiftReg', 'TstRegShiftReg', 'CmpRegShiftReg']
#[repr(transparent)]
pub struct DpTestRsrBits(pub u32);
impl DpTestRsrBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rs(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn stype(&self) -> u32 { (self.0 & 0x00000060) >> 5 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Smlabb']
#[repr(transparent)]
pub struct SmlabbBits(pub u32);
impl SmlabbBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn ra(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn m(&self) -> bool { (self.0 & 0x00000040) != 0 }
    #[inline(always)]
    pub fn n(&self) -> bool { (self.0 & 0x00000020) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Smulbb']
#[repr(transparent)]
pub struct SmulbbBits(pub u32);
impl SmulbbBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn m(&self) -> bool { (self.0 & 0x00000040) != 0 }
    #[inline(always)]
    pub fn n(&self) -> bool { (self.0 & 0x00000020) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['PldImm']
#[repr(transparent)]
pub struct PldImmBits(pub u32);
impl PldImmBits {
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn r(&self) -> bool { (self.0 & 0x00400000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

/// ['LdrsbImm', 'StrhImm', 'LdrshImm', 'StrdImm', 'LdrhImm', 'LdrdImm']
#[repr(transparent)]
pub struct LsSignedImmBits(pub u32);
impl LsSignedImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm4h(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn imm4l(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['StrdReg', 'LdrsbReg', 'LdrshReg', 'LdrdReg', 'LdrhReg', 'StrhReg']
#[repr(transparent)]
pub struct LsSignedRegBits(pub u32);
impl LsSignedRegBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['AndRegShiftReg', 'AdcRegShiftReg', 'OrrRegShiftReg', 'EorRegShiftReg', 'RscRegShiftReg', 'SbcRegShiftReg', 'AddRegShiftReg', 'BicRegShiftReg', 'RsbRegShiftReg', 'SubRegShiftReg']
#[repr(transparent)]
pub struct DpRsrBits(pub u32);
impl DpRsrBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rs(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn stype(&self) -> u32 { (self.0 & 0x00000060) >> 5 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['MovRegShiftReg', 'MvnRegShiftReg']
#[repr(transparent)]
pub struct MovRsrBits(pub u32);
impl MovRsrBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rs(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn stype(&self) -> u32 { (self.0 & 0x00000060) >> 5 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['CmpReg', 'TstReg', 'CmnReg', 'TeqReg']
#[repr(transparent)]
pub struct DpTestRegBits(pub u32);
impl DpTestRegBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm5(&self) -> u32 { (self.0 & 0x00000f80) >> 7 }
    #[inline(always)]
    pub fn stype(&self) -> u32 { (self.0 & 0x00000060) >> 5 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['Mrc', 'Mcr']
#[repr(transparent)]
pub struct MoveCoprocBits(pub u32);
impl MoveCoprocBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn opc1(&self) -> u32 { (self.0 & 0x00e00000) >> 21 }
    #[inline(always)]
    pub fn crn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn coproc(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn opc2(&self) -> u32 { (self.0 & 0x000000e0) >> 5 }
    #[inline(always)]
    pub fn crm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['MovImmAlt']
#[repr(transparent)]
pub struct MovImmAltBits(pub u32);
impl MovImmAltBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn imm4(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

/// ['CmnImm', 'CmpImm', 'TstImm', 'TeqImm']
#[repr(transparent)]
pub struct DpTestImmBits(pub u32);
impl DpTestImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

/// ['LdrbtAlt', 'StrbtAlt', 'LdrtAlt', 'StrtAlt']
#[repr(transparent)]
pub struct LsTransAltBits(pub u32);
impl LsTransAltBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm5(&self) -> u32 { (self.0 & 0x00000f80) >> 7 }
    #[inline(always)]
    pub fn stype(&self) -> u32 { (self.0 & 0x00000060) >> 5 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['SbcReg', 'OrrReg', 'BicReg', 'AddReg', 'RscReg', 'EorReg', 'AdcReg', 'SubReg', 'AndReg', 'RsbReg']
#[repr(transparent)]
pub struct DpRegBits(pub u32);
impl DpRegBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm5(&self) -> u32 { (self.0 & 0x00000f80) >> 7 }
    #[inline(always)]
    pub fn stype(&self) -> u32 { (self.0 & 0x00000060) >> 5 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['AddImm', 'AdcImm', 'RsbImm', 'OrrImm', 'BicImm', 'SubImm', 'AndImm', 'RscImm', 'EorImm', 'SbcImm']
#[repr(transparent)]
pub struct DpImmBits(pub u32);
impl DpImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

/// ['Ldrbt', 'Strbt', 'Ldrt', 'Strt']
#[repr(transparent)]
pub struct LsTransBits(pub u32);
impl LsTransBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

/// ['Stm', 'Stmda', 'Ldmda', 'Ldmib', 'Ldmdb', 'Ldm', 'Stmdb', 'Stmib']
#[repr(transparent)]
pub struct LsMultiBits(pub u32);
impl LsMultiBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

/// ['MsrImm']
#[repr(transparent)]
pub struct MsrImmBits(pub u32);
impl MsrImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn r(&self) -> bool { (self.0 & 0x00400000) != 0 }
    #[inline(always)]
    pub fn mask(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

/// ['LdrReg', 'StrbReg', 'LdrbReg', 'StrReg']
#[repr(transparent)]
pub struct LsRegBits(pub u32);
impl LsRegBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm5(&self) -> u32 { (self.0 & 0x00000f80) >> 7 }
    #[inline(always)]
    pub fn stype(&self) -> u32 { (self.0 & 0x00000060) >> 5 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

/// ['LdmRegUser']
#[repr(transparent)]
pub struct LdmRegUserBits(pub u32);
impl LdmRegUserBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x00007fff) >> 0 }
}

/// ['StrImm', 'StrbImm', 'LdrbImm', 'LdrImm']
#[repr(transparent)]
pub struct LsImmBits(pub u32);
impl LsImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

/// ['StmRegUser']
#[repr(transparent)]
pub struct StmRegUserBits(pub u32);
impl StmRegUserBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

/// ['Svc', 'B', 'BlImm']
#[repr(transparent)]
pub struct BranchBits(pub u32);
impl BranchBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn h(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn imm24(&self) -> u32 { (self.0 & 0x00ffffff) >> 0 }
}

