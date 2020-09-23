use crate::cpu::armv5::*;



/// Unimplemented instruction handler.
pub fn unimpl_instr(cpu: &mut Cpu, op: u32) {
    panic!("Unimplemented instruction {:08x}", op)
}


