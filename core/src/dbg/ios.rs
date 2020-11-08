

#[derive(Debug)]
pub enum SyscallName {
    ThreadCancel    = 0x02,
    ThreadGetPid    = 0x04,
    ThreadSetPrio   = 0x09,
    MqueueCreate    = 0x0a,
    MqueueRecv      = 0x0e,
    MqueueRegisterHandler = 0x0f,
    TimerCreate     = 0x11,
    HeapAlloc       = 0x18,
    Open            = 0x1c,
    RegisterDevice  = 0x1b,
    SetUid          = 0x2b,
    SetGid          = 0x2d,
}
impl From<u32> for SyscallName {
    fn from(x: u32) -> Self {
        use SyscallName::*;
        match x {
            0x02 => ThreadCancel,
            0x04 => ThreadGetPid,
            0x09 => ThreadSetPrio,
            0x0a => MqueueCreate,
            0x0e => MqueueRecv,
            0x0f => MqueueRegisterHandler,
            0x11 => TimerCreate,
            0x18 => HeapAlloc,
            0x1c => Open,
            0x1b => RegisterDevice,
            0x2b => SetUid,
            0x2d => SetGid,
            _ => panic!("Couldn't resolve syscall idx={:02x}", x),
        }
    }
}

pub fn log_syscall(opcd: u32, pc: u32, lr: u32) {
    let syscall = SyscallName::from((opcd & 0x00ff_ffe0) >> 5);
    println!("IOS pc={:08x} lr={:08x} syscall={:?}", pc, lr, syscall);
}
