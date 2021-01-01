
use crate::lut::*;
use crate::jit::*;
use crate::decode::arm::ArmInst;
use crate::decode::thumb::ThumbInst;

#[derive(Clone, Copy)]
pub struct ArmFn(pub fn(&mut JitBackend, u32));
#[derive(Clone, Copy)]
pub struct ThumbFn(pub fn(&mut JitBackend, u16));

/// The ARMv5 lookup table.
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

/// The ARMv5T lookup table.
pub struct ThumbLut { 
    pub data: [ThumbFn; 0x400] 
}
impl InstLut for ThumbLut {
    const LUT_SIZE: usize = 0x400;
    type Entry = ThumbFn;
    type Instr = ThumbInst;
    type Index = usize;

    fn lookup(&self, opcd: u16) -> ThumbFn { 
        self.data[Self::opcd_to_idx(opcd)] 
    }

    fn idx_to_opcd(idx: usize) -> u16 {
        (idx << 6) as u16
    }

    fn opcd_to_idx(opcd: u16) -> usize {
        ((opcd & 0xffc0) >> 6) as usize
    }

    fn create_lut(default_entry: ThumbFn) -> Self {
        let mut lut = ThumbLut {
            data: [default_entry; 0x400],
        };
        for i in 0..Self::LUT_SIZE {
            let opcd = ThumbLut::idx_to_opcd(i);
            lut.data[i as usize] = ThumbFn::from_inst(ThumbInst::decode(opcd));
        }
        lut
    }
}

/// Container for lookup tables
pub struct JitLut {
    /// Lookup table for ARM instructions.
    pub arm: ArmLut,
    /// Lookup table for Thumb instructions.
    pub thumb: ThumbLut,
}
impl JitLut {
    pub fn new() -> Self {
        let arm = ArmLut::create_lut(ArmFn(dispatch::arm_unimpl_instr));
        let thumb = ThumbLut::create_lut(ThumbFn(dispatch::thumb_unimpl_instr));
        JitLut { arm, thumb }
    }
}

