

use crate::jit::*;
use crate::lut::*;
use crate::jit::lut::*;
use crate::decode::arm::ArmInst;
use crate::decode::thumb::ThumbInst;

/// Handler for unimplemented ARM instructions.
pub fn arm_unimpl_instr(back: &mut JitBackend, op: u32) {
    if (op & 0xe600_0000) != 0xe600_0000 {
        println!("Couldn't dispatch instruction {:08x} ({:?})",
            op, ArmInst::decode(op));
    }
}

/// Handler for unimplemented Thumb instructions.
pub fn thumb_unimpl_instr(back: &mut JitBackend, op: u16) {
    println!("Couldn't dispatch Thumb instruction {:04x} ({:?})",
        op, ThumbInst::decode(op));
}

// We use these macros to coerce the borrow checker into taking pointers to
// functions which take a newtype wrapping a u32/u16 (for bitfields).
macro_rules! afn { ($func:expr) => { unsafe {
    std::mem::transmute::<*const fn(), fn(&mut JitBackend, u32)>
        ($func as *const fn())
}}}

macro_rules! tfn { ($func:expr) => { unsafe {
    std::mem::transmute::<*const fn(), fn(&mut JitBackend, u16)>
        ($func as *const fn())
}}}


impl InstLutEntry for ArmFn {
    type Inst = ArmInst;
    fn from_inst(inst: ArmInst) -> Self {
        use ArmInst::*;
        match inst {
            _ => ArmFn(arm_unimpl_instr),
        }
    }
}

impl InstLutEntry for ThumbFn {
    type Inst = ThumbInst;
    fn from_inst(inst: ThumbInst) -> Self {
        use ThumbInst::*;
        match inst {
            _ => ThumbFn(thumb_unimpl_instr),
        }
    }
}


