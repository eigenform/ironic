
use crate::cpu::*;
use crate::cpu::reg::*;
use crate::cpu::excep::*;
use crate::cpu::exec::thumb::bits::*;

pub fn svc(cpu: &mut Cpu, op: MiscBits) -> DispatchRes {
    DispatchRes::Exception(ExceptionType::Swi)
}
