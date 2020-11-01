//! For handling the Thumb instruction set.

pub mod bits;
pub mod decode;
pub mod dispatch;

pub mod loadstore;
pub mod dataproc;
pub mod branch;
pub mod misc;

use crate::cpu::*;
use crate::cpu::exec::thumb::dispatch::*;
use crate::cpu::exec::thumb::decode::*;

/// An ARMv5T lookup table.
pub struct ThumbLut { 
    pub data: [dispatch::ThumbFn; 0x400] 
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


