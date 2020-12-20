
/// Added by hand
#[repr(transparent)]
pub struct BlBits(pub u16);
impl BlBits {
    #[inline(always)]
    pub fn imm11(&self) -> u16 { (self.0 & 0x07ff) >> 0 }
}


/// ['RsbImm']
#[repr(transparent)]
pub struct RsbImmBits(pub u16);
impl RsbImmBits {
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rd(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['SbcReg', 'OrrReg', 'BicReg', 'EorReg', 'AdcReg', 'AndReg']
#[repr(transparent)]
pub struct BitwiseRegBits(pub u16);
impl BitwiseRegBits {
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rdn(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['CmpReg', 'TstReg', 'CmnReg']
#[repr(transparent)]
pub struct CmpRegBits(pub u16);
impl CmpRegBits {
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['MvnReg']
#[repr(transparent)]
pub struct MvnRegBits(pub u16);
impl MvnRegBits {
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rd(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['Mul']
#[repr(transparent)]
pub struct MulBits(pub u16);
impl MulBits {
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rdm(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['AddSpImmAlt', 'SubSpImm']
#[repr(transparent)]
pub struct AddSubSpImmAltBits(pub u16);
impl AddSubSpImmAltBits {
    #[inline(always)]
    pub fn imm7(&self) -> u16 { (self.0 & 0x007f) >> 0 }
}

/// ['Bx', 'BlxReg']
#[repr(transparent)]
pub struct BxBits(pub u16);
impl BxBits {
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x0078) >> 3 }
}

/// ['Svc', 'Bkpt']
#[repr(transparent)]
pub struct MiscBits(pub u16);
impl MiscBits {
    #[inline(always)]
    pub fn imm8(&self) -> u16 { (self.0 & 0x00ff) >> 0 }
}

/// ['CmpRegAlt']
#[repr(transparent)]
pub struct CmpRegAltBits(pub u16);
impl CmpRegAltBits {
    #[inline(always)]
    pub fn n(&self) -> bool { (self.0 & 0x0080) != 0 }
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x0078) >> 3 }
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['AddRegAlt']
#[repr(transparent)]
pub struct AddRegAltBits(pub u16);
impl AddRegAltBits {
    #[inline(always)]
    pub fn dn(&self) -> bool { (self.0 & 0x0080) != 0 }
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x0078) >> 3 }
    #[inline(always)]
    pub fn rdn(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['MovReg']
#[repr(transparent)]
pub struct MovRegBits(pub u16);
impl MovRegBits {
    #[inline(always)]
    pub fn d(&self) -> bool { (self.0 & 0x0080) != 0 }
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x0078) >> 3 }
    #[inline(always)]
    pub fn rd(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['AddImm', 'SubImm']
#[repr(transparent)]
pub struct AddSubImmBits(pub u16);
impl AddSubImmBits {
    #[inline(always)]
    pub fn imm3(&self) -> u16 { (self.0 & 0x01c0) >> 6 }
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rd(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['LdrReg', 'LdrsbReg', 'LdrshReg', 'StrbReg', 'LdrhReg', 'LdrbReg', 'StrReg', 'StrhReg']
#[repr(transparent)]
pub struct LoadStoreRegBits(pub u16);
impl LoadStoreRegBits {
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x01c0) >> 6 }
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rt(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['MovRegShiftReg']
#[repr(transparent)]
pub struct MovRsrBits(pub u16);
impl MovRsrBits {
    #[inline(always)]
    pub fn op(&self) -> u16 { (self.0 & 0x03c0) >> 6 }
    #[inline(always)]
    pub fn rs(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rdm(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['Pop']
#[repr(transparent)]
pub struct PopBits(pub u16);
impl PopBits {
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x0100) != 0 }
    #[inline(always)]
    pub fn register_list(&self) -> u16 { (self.0 & 0x00ff) >> 0 }
}

/// ['AddReg', 'SubReg']
#[repr(transparent)]
pub struct AddSubRegBits(pub u16);
impl AddSubRegBits {
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x01c0) >> 6 }
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rd(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['Push']
#[repr(transparent)]
pub struct PushBits(pub u16);
impl PushBits {
    #[inline(always)]
    pub fn m(&self) -> bool { (self.0 & 0x0100) != 0 }
    #[inline(always)]
    pub fn register_list(&self) -> u16 { (self.0 & 0x00ff) >> 0 }
}

/// ['MovImm', 'AddSpImm']
#[repr(transparent)]
pub struct MovImmBits(pub u16);
impl MovImmBits {
    #[inline(always)]
    pub fn rd(&self) -> u16 { (self.0 & 0x0700) >> 8 }
    #[inline(always)]
    pub fn imm8(&self) -> u16 { (self.0 & 0x00ff) >> 0 }
}

/// ['AddImmAlt', 'SubImmAlt']
#[repr(transparent)]
pub struct AddSubImmAltBits(pub u16);
impl AddSubImmAltBits {
    #[inline(always)]
    pub fn rdn(&self) -> u16 { (self.0 & 0x0700) >> 8 }
    #[inline(always)]
    pub fn imm8(&self) -> u16 { (self.0 & 0x00ff) >> 0 }
}

/// ['StrhImm', 'StrImm', 'StrbImm', 'LdrhImm', 'LdrbImm', 'LdrImm']
#[repr(transparent)]
pub struct LoadStoreImmBits(pub u16);
impl LoadStoreImmBits {
    #[inline(always)]
    pub fn imm5(&self) -> u16 { (self.0 & 0x07c0) >> 6 }
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rt(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

/// ['Stm', 'Ldm']
#[repr(transparent)]
pub struct LoadStoreMultiBits(pub u16);
impl LoadStoreMultiBits {
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0700) >> 8 }
    #[inline(always)]
    pub fn register_list(&self) -> u16 { (self.0 & 0x00ff) >> 0 }
}

/// ['CmpImm']
#[repr(transparent)]
pub struct CmpImmBits(pub u16);
impl CmpImmBits {
    #[inline(always)]
    pub fn rn(&self) -> u16 { (self.0 & 0x0700) >> 8 }
    #[inline(always)]
    pub fn imm8(&self) -> u16 { (self.0 & 0x00ff) >> 0 }
}

/// ['StrImmAlt', 'LdrImmAlt', 'LdrLit']
#[repr(transparent)]
pub struct LoadStoreAltBits(pub u16);
impl LoadStoreAltBits {
    #[inline(always)]
    pub fn rt(&self) -> u16 { (self.0 & 0x0700) >> 8 }
    #[inline(always)]
    pub fn imm8(&self) -> u16 { (self.0 & 0x00ff) >> 0 }
}

/// ['BAlt']
#[repr(transparent)]
pub struct BranchAltBits(pub u16);
impl BranchAltBits {
    #[inline(always)]
    pub fn imm11(&self) -> u16 { (self.0 & 0x07ff) >> 0 }
}

/// ['B']
#[repr(transparent)]
pub struct BranchBits(pub u16);
impl BranchBits {
    #[inline(always)]
    pub fn cond(&self) -> u16 { (self.0 & 0x0f00) >> 8 }
    #[inline(always)]
    pub fn imm8(&self) -> u16 { (self.0 & 0x00ff) >> 0 }
}

/// ['MovRegAlt']
#[repr(transparent)]
pub struct MovRegAltBits(pub u16);
impl MovRegAltBits {
    #[inline(always)]
    pub fn op(&self) -> u16 { (self.0 & 0x1800) >> 11 }
    #[inline(always)]
    pub fn imm5(&self) -> u16 { (self.0 & 0x07c0) >> 6 }
    #[inline(always)]
    pub fn rm(&self) -> u16 { (self.0 & 0x0038) >> 3 }
    #[inline(always)]
    pub fn rd(&self) -> u16 { (self.0 & 0x0007) >> 0 }
}

