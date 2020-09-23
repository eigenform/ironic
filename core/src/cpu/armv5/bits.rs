
#[repr(transparent)]
pub struct LdcLitBits(pub u32);
impl LdcLitBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn imm8(&self) -> u32 { (self.0 & 0x000000ff) >> 0 }
}

#[repr(transparent)]
pub struct PldLitBits(pub u32);
impl PldLitBits {
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

#[repr(transparent)]
pub struct LdrdLitBits(pub u32);
impl LdrdLitBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm4h(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn imm4l(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

#[repr(transparent)]
pub struct LdrhLitBits(pub u32);
impl LdrhLitBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm4h(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn imm4l(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

#[repr(transparent)]
pub struct LdrshLitBits(pub u32);
impl LdrshLitBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm4h(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn imm4l(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

#[repr(transparent)]
pub struct LdrsbLitBits(pub u32);
impl LdrsbLitBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm4h(&self) -> u32 { (self.0 & 0x00000f00) >> 8 }
    #[inline(always)]
    pub fn imm4l(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

#[repr(transparent)]
pub struct StcBits(pub u32);
impl StcBits {
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

#[repr(transparent)]
pub struct LdcImmBits(pub u32);
impl LdcImmBits {
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

#[repr(transparent)]
pub struct SubSpRegBits(pub u32);
impl SubSpRegBits {
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

#[repr(transparent)]
pub struct AddSpRegBits(pub u32);
impl AddSpRegBits {
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

#[repr(transparent)]
pub struct QdaddBits(pub u32);
impl QdaddBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

#[repr(transparent)]
pub struct QsubBits(pub u32);
impl QsubBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

#[repr(transparent)]
pub struct QaddBits(pub u32);
impl QaddBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

#[repr(transparent)]
pub struct QdsubBits(pub u32);
impl QdsubBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

#[repr(transparent)]
pub struct BxBits(pub u32);
impl BxBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

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

#[repr(transparent)]
pub struct BxjBits(pub u32);
impl BxjBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

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

#[repr(transparent)]
pub struct BlxRegBits(pub u32);
impl BlxRegBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rm(&self) -> u32 { (self.0 & 0x0000000f) >> 0 }
}

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

#[repr(transparent)]
pub struct SmullBits(pub u32);
impl SmullBits {
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

#[repr(transparent)]
pub struct UmlalBits(pub u32);
impl UmlalBits {
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

#[repr(transparent)]
pub struct SmlalBits(pub u32);
impl SmlalBits {
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

#[repr(transparent)]
pub struct UmullBits(pub u32);
impl UmullBits {
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

#[repr(transparent)]
pub struct AddSpImmBits(pub u32);
impl AddSpImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

#[repr(transparent)]
pub struct SubSpImmBits(pub u32);
impl SubSpImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

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

#[repr(transparent)]
pub struct McrrBits(pub u32);
impl McrrBits {
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

#[repr(transparent)]
pub struct MrrcBits(pub u32);
impl MrrcBits {
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

#[repr(transparent)]
pub struct SmlalbbBits(pub u32);
impl SmlalbbBits {
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

#[repr(transparent)]
pub struct TeqRegShiftRegBits(pub u32);
impl TeqRegShiftRegBits {
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

#[repr(transparent)]
pub struct CmnRegShiftRegBits(pub u32);
impl CmnRegShiftRegBits {
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

#[repr(transparent)]
pub struct TstRegShiftRegBits(pub u32);
impl TstRegShiftRegBits {
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

#[repr(transparent)]
pub struct CmpRegShiftRegBits(pub u32);
impl CmpRegShiftRegBits {
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

#[repr(transparent)]
pub struct LdrsbImmBits(pub u32);
impl LdrsbImmBits {
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

#[repr(transparent)]
pub struct StrdRegBits(pub u32);
impl StrdRegBits {
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

#[repr(transparent)]
pub struct StrhImmBits(pub u32);
impl StrhImmBits {
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

#[repr(transparent)]
pub struct LdrsbRegBits(pub u32);
impl LdrsbRegBits {
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

#[repr(transparent)]
pub struct LdrshImmBits(pub u32);
impl LdrshImmBits {
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

#[repr(transparent)]
pub struct LdrshRegBits(pub u32);
impl LdrshRegBits {
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

#[repr(transparent)]
pub struct StrdImmBits(pub u32);
impl StrdImmBits {
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

#[repr(transparent)]
pub struct LdrhImmBits(pub u32);
impl LdrhImmBits {
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

#[repr(transparent)]
pub struct LdrdRegBits(pub u32);
impl LdrdRegBits {
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

#[repr(transparent)]
pub struct LdrhRegBits(pub u32);
impl LdrhRegBits {
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

#[repr(transparent)]
pub struct LdrdImmBits(pub u32);
impl LdrdImmBits {
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

#[repr(transparent)]
pub struct StrhRegBits(pub u32);
impl StrhRegBits {
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

#[repr(transparent)]
pub struct AndRegShiftRegBits(pub u32);
impl AndRegShiftRegBits {
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

#[repr(transparent)]
pub struct AdcRegShiftRegBits(pub u32);
impl AdcRegShiftRegBits {
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

#[repr(transparent)]
pub struct MovRegShiftRegBits(pub u32);
impl MovRegShiftRegBits {
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

#[repr(transparent)]
pub struct OrrRegShiftRegBits(pub u32);
impl OrrRegShiftRegBits {
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

#[repr(transparent)]
pub struct EorRegShiftRegBits(pub u32);
impl EorRegShiftRegBits {
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

#[repr(transparent)]
pub struct RscRegShiftRegBits(pub u32);
impl RscRegShiftRegBits {
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

#[repr(transparent)]
pub struct MvnRegShiftRegBits(pub u32);
impl MvnRegShiftRegBits {
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

#[repr(transparent)]
pub struct SbcRegShiftRegBits(pub u32);
impl SbcRegShiftRegBits {
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

#[repr(transparent)]
pub struct AddRegShiftRegBits(pub u32);
impl AddRegShiftRegBits {
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

#[repr(transparent)]
pub struct BicRegShiftRegBits(pub u32);
impl BicRegShiftRegBits {
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

#[repr(transparent)]
pub struct RsbRegShiftRegBits(pub u32);
impl RsbRegShiftRegBits {
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

#[repr(transparent)]
pub struct SubRegShiftRegBits(pub u32);
impl SubRegShiftRegBits {
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

#[repr(transparent)]
pub struct CmpRegBits(pub u32);
impl CmpRegBits {
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

#[repr(transparent)]
pub struct TstRegBits(pub u32);
impl TstRegBits {
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

#[repr(transparent)]
pub struct CmnRegBits(pub u32);
impl CmnRegBits {
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

#[repr(transparent)]
pub struct TeqRegBits(pub u32);
impl TeqRegBits {
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

#[repr(transparent)]
pub struct LdrbLitBits(pub u32);
impl LdrbLitBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

#[repr(transparent)]
pub struct LdrLitBits(pub u32);
impl LdrLitBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn p(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn u(&self) -> bool { (self.0 & 0x00800000) != 0 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rt(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

#[repr(transparent)]
pub struct MrcBits(pub u32);
impl MrcBits {
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

#[repr(transparent)]
pub struct McrBits(pub u32);
impl McrBits {
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

#[repr(transparent)]
pub struct CmnImmBits(pub u32);
impl CmnImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

#[repr(transparent)]
pub struct CmpImmBits(pub u32);
impl CmpImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

#[repr(transparent)]
pub struct TstImmBits(pub u32);
impl TstImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

#[repr(transparent)]
pub struct TeqImmBits(pub u32);
impl TeqImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

#[repr(transparent)]
pub struct LdrbtAltBits(pub u32);
impl LdrbtAltBits {
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

#[repr(transparent)]
pub struct StrbtAltBits(pub u32);
impl StrbtAltBits {
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

#[repr(transparent)]
pub struct LdrtAltBits(pub u32);
impl LdrtAltBits {
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

#[repr(transparent)]
pub struct StrtAltBits(pub u32);
impl StrtAltBits {
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

#[repr(transparent)]
pub struct SbcRegBits(pub u32);
impl SbcRegBits {
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

#[repr(transparent)]
pub struct OrrRegBits(pub u32);
impl OrrRegBits {
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

#[repr(transparent)]
pub struct BicRegBits(pub u32);
impl BicRegBits {
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

#[repr(transparent)]
pub struct AddRegBits(pub u32);
impl AddRegBits {
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

#[repr(transparent)]
pub struct RscRegBits(pub u32);
impl RscRegBits {
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

#[repr(transparent)]
pub struct EorRegBits(pub u32);
impl EorRegBits {
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

#[repr(transparent)]
pub struct MvnRegBits(pub u32);
impl MvnRegBits {
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

#[repr(transparent)]
pub struct AdcRegBits(pub u32);
impl AdcRegBits {
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

#[repr(transparent)]
pub struct SubRegBits(pub u32);
impl SubRegBits {
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

#[repr(transparent)]
pub struct AndRegBits(pub u32);
impl AndRegBits {
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

#[repr(transparent)]
pub struct RsbRegBits(pub u32);
impl RsbRegBits {
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

#[repr(transparent)]
pub struct AddImmBits(pub u32);
impl AddImmBits {
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

#[repr(transparent)]
pub struct AdcImmBits(pub u32);
impl AdcImmBits {
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

#[repr(transparent)]
pub struct RsbImmBits(pub u32);
impl RsbImmBits {
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

#[repr(transparent)]
pub struct OrrImmBits(pub u32);
impl OrrImmBits {
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

#[repr(transparent)]
pub struct BicImmBits(pub u32);
impl BicImmBits {
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

#[repr(transparent)]
pub struct SubImmBits(pub u32);
impl SubImmBits {
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

#[repr(transparent)]
pub struct MvnImmBits(pub u32);
impl MvnImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn s(&self) -> bool { (self.0 & 0x00100000) != 0 }
    #[inline(always)]
    pub fn rd(&self) -> u32 { (self.0 & 0x0000f000) >> 12 }
    #[inline(always)]
    pub fn imm12(&self) -> u32 { (self.0 & 0x00000fff) >> 0 }
}

#[repr(transparent)]
pub struct AndImmBits(pub u32);
impl AndImmBits {
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

#[repr(transparent)]
pub struct RscImmBits(pub u32);
impl RscImmBits {
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

#[repr(transparent)]
pub struct EorImmBits(pub u32);
impl EorImmBits {
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

#[repr(transparent)]
pub struct SbcImmBits(pub u32);
impl SbcImmBits {
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

#[repr(transparent)]
pub struct LdrbtBits(pub u32);
impl LdrbtBits {
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

#[repr(transparent)]
pub struct StrbtBits(pub u32);
impl StrbtBits {
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

#[repr(transparent)]
pub struct LdrtBits(pub u32);
impl LdrtBits {
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

#[repr(transparent)]
pub struct StrtBits(pub u32);
impl StrtBits {
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

#[repr(transparent)]
pub struct StmBits(pub u32);
impl StmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

#[repr(transparent)]
pub struct StmdaBits(pub u32);
impl StmdaBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

#[repr(transparent)]
pub struct LdmdaBits(pub u32);
impl LdmdaBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

#[repr(transparent)]
pub struct LdmibBits(pub u32);
impl LdmibBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

#[repr(transparent)]
pub struct LdmdbBits(pub u32);
impl LdmdbBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

#[repr(transparent)]
pub struct LdmBits(pub u32);
impl LdmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

#[repr(transparent)]
pub struct StmdbBits(pub u32);
impl StmdbBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

#[repr(transparent)]
pub struct StmibBits(pub u32);
impl StmibBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn w(&self) -> bool { (self.0 & 0x00200000) != 0 }
    #[inline(always)]
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x0000ffff) >> 0 }
}

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

#[repr(transparent)]
pub struct BlImmAltBits(pub u32);
impl BlImmAltBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn h(&self) -> bool { (self.0 & 0x01000000) != 0 }
    #[inline(always)]
    pub fn imm24(&self) -> u32 { (self.0 & 0x00ffffff) >> 0 }
}

#[repr(transparent)]
pub struct LdrRegBits(pub u32);
impl LdrRegBits {
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

#[repr(transparent)]
pub struct StrbRegBits(pub u32);
impl StrbRegBits {
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

#[repr(transparent)]
pub struct LdrbRegBits(pub u32);
impl LdrbRegBits {
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

#[repr(transparent)]
pub struct StrRegBits(pub u32);
impl StrRegBits {
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
    pub fn rn(&self) -> u32 { (self.0 & 0x000f0000) >> 16 }
    #[inline(always)]
    pub fn register_list(&self) -> u32 { (self.0 & 0x00007fff) >> 0 }
}

#[repr(transparent)]
pub struct LdmRegExcepBits(pub u32);
impl LdmRegExcepBits {
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

#[repr(transparent)]
pub struct StrImmBits(pub u32);
impl StrImmBits {
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

#[repr(transparent)]
pub struct StrbImmBits(pub u32);
impl StrbImmBits {
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

#[repr(transparent)]
pub struct LdrbImmBits(pub u32);
impl LdrbImmBits {
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

#[repr(transparent)]
pub struct LdrImmBits(pub u32);
impl LdrImmBits {
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

#[repr(transparent)]
pub struct SvcBits(pub u32);
impl SvcBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn imm24(&self) -> u32 { (self.0 & 0x00ffffff) >> 0 }
}

#[repr(transparent)]
pub struct BBits(pub u32);
impl BBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn imm24(&self) -> u32 { (self.0 & 0x00ffffff) >> 0 }
}

#[repr(transparent)]
pub struct BlImmBits(pub u32);
impl BlImmBits {
    #[inline(always)]
    pub fn cond(&self) -> u32 { (self.0 & 0xf0000000) >> 28 }
    #[inline(always)]
    pub fn imm24(&self) -> u32 { (self.0 & 0x00ffffff) >> 0 }
}

