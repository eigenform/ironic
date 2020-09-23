//! Types used for dispatching instructions.

use crate::cpu::*;
use crate::cpu::armv5::*;
use crate::cpu::armv5::decode::*;

/// A function pointer to an ARM instruction implementation.
#[derive(Clone, Copy)]
pub struct ArmFn(pub fn(&mut Cpu, u32));

/// Implementing [InstLutEntry] maps each instruction to a function.
impl InstLutEntry for ArmFn {
    type Inst = ArmInst;
    fn from_inst(inst: ArmInst) -> Self {
        match inst {
            _ => ArmFn(func::unimpl_instr),
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

/// Container for lookup tables
pub struct Lut {
    pub arm: ArmLut,
}
impl Lut {
    pub fn new() -> Self {
        Lut {
            arm: ArmLut::create_lut(ArmFn(func::unimpl_instr)),
        }
    }
}


