
use crate::cpu::lut::*;
use crate::cpu::exec::thumb::*;
use crate::cpu::exec::thumb::decode::ThumbInst;

#[derive(Clone, Copy)]
pub struct ThumbFn(pub fn(&mut Cpu, u16) -> DispatchRes);

pub fn unimpl_instr(cpu: &mut Cpu, op: u16) -> DispatchRes {
    log(&cpu.dbg, LogLevel::Cpu, &format!(
        "Couldn't dispatch Thumb instruction {:04x} ({:?})",
        op, ThumbInst::decode(op)));
    println!("Couldn't dispatch Thumb instruction {:04x} ({:?})",
        op, ThumbInst::decode(op));
    DispatchRes::FatalErr
}

impl InstLutEntry for ThumbFn {
    type Inst = ThumbInst;
    fn from_inst(inst: ThumbInst) -> Self {
        use ThumbInst::*;
        use std::mem::transmute;

        macro_rules! cfn { ($func:expr) => { unsafe {
            transmute::<*const fn(), fn(&mut Cpu, u16) -> DispatchRes>
                ($func as *const fn())
        }}}

        match inst {
            _       => ThumbFn(unimpl_instr),
        }
    }
}
