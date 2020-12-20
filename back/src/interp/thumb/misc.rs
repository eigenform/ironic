
use crate::bits::thumb::*;
use crate::interp::DispatchRes;
use ironic_core::cpu::Cpu;
use ironic_core::cpu::excep::ExceptionType;

pub fn svc(_cpu: &mut Cpu, _op: MiscBits) -> DispatchRes {
    DispatchRes::Exception(ExceptionType::Swi)
}
