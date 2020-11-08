//! For handling the ARM instruction set.

pub mod bits;
pub mod decode;
pub mod dispatch;

pub mod loadstore;
pub mod dataproc;
pub mod branch;
pub mod coproc;
pub mod multiply;
pub mod status;

use crate::cpu::*;
use crate::cpu::arm::dispatch::ArmFn;

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


