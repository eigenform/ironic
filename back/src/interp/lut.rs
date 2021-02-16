//! ARM/Thumb lookup tables for the interpreter backend.

use crate::interp::DispatchRes;
use crate::interp::dispatch;
use ironic_core::cpu::Cpu;
use crate::decode::arm::ArmInst;
use crate::decode::thumb::ThumbInst;

/// Lookup tables for the interpreter backend, evaluated at compile-time.
pub const INTERP_LUT: InterpLut = InterpLut::new();

/// A function pointer to an ARM instruction implementation.
#[derive(Clone, Copy)]
pub struct ArmFn(pub fn(&mut Cpu, u32) -> DispatchRes);

/// A function pointer to a Thumb instruction implementation.
#[derive(Clone, Copy)]
pub struct ThumbFn(pub fn(&mut Cpu, u16) -> DispatchRes);

/// The ARMv5 lookup table.
pub struct ArmLut { 
    pub data: [ArmFn; 0x1000] 
}
impl ArmLut {
    pub fn lookup(&self, opcd: u32) -> ArmFn { 
        self.data[Self::opcd_to_idx(opcd)] 
    }

    const fn idx_to_opcd(idx: usize) -> u32 {
        (((idx & 0x0ff0) << 16) | ((idx & 0x000f) << 4)) as u32
    }
    const fn opcd_to_idx(opcd: u32) -> usize {
        (((opcd >> 16) & 0x0ff0) | ((opcd >> 4) & 0x000f)) as usize
    }
    const LUT_SIZE: usize = 0x1000;
    const fn create_lut(default_entry: ArmFn) -> Self {
        let mut lut = ArmLut {
            data: [default_entry; 0x1000],
        };
        let mut i = 0;
        while i < Self::LUT_SIZE {
            let opcd = ArmLut::idx_to_opcd(i);
            lut.data[i as usize] = ArmFn::from_inst(ArmInst::decode(opcd));
            i += 1;
        }
        lut
    }
}

/// The ARMv5T lookup table.
pub struct ThumbLut { 
    pub data: [ThumbFn; 0x400] 
}
impl ThumbLut {
    pub fn lookup(&self, opcd: u16) -> ThumbFn { 
        self.data[Self::opcd_to_idx(opcd)] 
    }

    const fn idx_to_opcd(idx: usize) -> u16 {
        (idx << 6) as u16
    }
    const fn opcd_to_idx(opcd: u16) -> usize {
        ((opcd & 0xffc0) >> 6) as usize
    }
    const LUT_SIZE: usize = 0x400;
    const fn create_lut(default_entry: ThumbFn) -> Self {
        let mut lut = ThumbLut {
            data: [default_entry; 0x400],
        };
        let mut i = 0;
        while i < Self::LUT_SIZE {
            let opcd = ThumbLut::idx_to_opcd(i);
            lut.data[i as usize] = ThumbFn::from_inst(ThumbInst::decode(opcd));
            i += 1;
        }
        lut
    }
}

/// Container for lookup tables
pub struct InterpLut {
    /// Lookup table for ARM instructions.
    pub arm: ArmLut,
    /// Lookup table for Thumb instructions.
    pub thumb: ThumbLut,
}
impl InterpLut {
    pub const fn new() -> Self {
        let arm = ArmLut::create_lut(ArmFn(dispatch::arm_unimpl_instr));
        let thumb = ThumbLut::create_lut(ThumbFn(dispatch::thumb_unimpl_instr));
        InterpLut { arm, thumb }
    }
}


