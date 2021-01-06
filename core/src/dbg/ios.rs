//! Logging for IOS syscalls.
//!
//! Note that a lot of this depends on the version being booted.
//! For now, it's sufficient to just assume we're on IOS58.

extern crate pretty_hex;
use pretty_hex::*;
use crate::cpu::Cpu;
use crate::cpu::reg::Reg;
use crate::cpu::mmu::prim::{Access, TLBReq};

/// NOTE: `skyeye-starlet` does something like this; wonder if there's a
/// better way of keeping track of the threads?
#[derive(Debug)]
pub enum ExecutionCtx {
    UNK,
    CRY,
    ES,
    FS,
    KRN,
    DIP,
    STM,
    OH0,
    OH1,
    SO,
    ETH,
    SDI,
}
impl From<u32> for ExecutionCtx {
    fn from(pc: u32) -> Self {
        use ExecutionCtx::*;
        match (pc & 0xffff_0000) >> 16 {
            0x1386 => CRY,
            0x138a => OH0,
            0x138b => OH1,
            0x13aa => ETH,
            0x13b6 => SO,
            0x2000 => FS,
            0x2010 => ES,
            0x2020 => DIP,
            0x2030 => STM,
            0x2040 => SDI,
            0xffff => KRN,
            _ => UNK,
        }
    }
}


/// Typed arguments to a syscall. 
pub enum ArgType { Ptr, StrPtr, Int, Uint }

/// Format arguments for some IOS syscall.
pub struct SyscallDef {
    pub name: &'static str,
    pub arg: &'static [ArgType],
}

/// Shorthand for declaring a syscall definition.
macro_rules! scdef {
    ($name:literal, $($arg:ident),*) => {
        SyscallDef { name: $name, arg: &[$(ArgType::$arg,)*] } 
    }
}

/// Read a NUL-terminated string from memory.  
/// 
/// NOTE: This is not particularly rigorous or safe.
pub fn read_string(cpu: &Cpu, ptr: u32) -> String {
    let paddr = cpu.translate(TLBReq::new(ptr, Access::Debug));

    let mut line_buf = [0u8; 64];
    cpu.bus.write().unwrap().dma_read(paddr, &mut line_buf);
    //println!("{:?}", line_buf.hex_dump());

    let mut end: Option<usize> = None;
    for (i, b) in line_buf.iter().enumerate() {
        if *b == 0x00 { end = Some(i); break; } 
    }
    let s = if end.is_some() {
        std::str::from_utf8(&line_buf[..=end.unwrap()]).unwrap()
    } else {
        std::str::from_utf8(&line_buf).unwrap()
    };
    s.trim_matches(char::from(0)).to_string()
}

pub fn get_syscall_desc(idx: u32) -> Option<SyscallDef> {

    // Ignore some syscalls
    match idx { 
        0x0a..=0x10 |
        //0x11..=0x13 | 0x15 |
        0x16 | 0x18 | 0x19 | 0x1a | 
        0x1d..=0x1f | 0x21 | 0x22 |
        0x63 | 0x68 | 0x6a | 0x6d |
        0x2a | 0x2f | 0x3f | 0x30 | 0x40 | 0x4f => return None,
        _ => { },
    }

    let res = Some(match idx {
        0x00 => scdef!("ThreadCreate", Ptr, Ptr, Ptr, Uint, Uint, Uint),
        0x02 => scdef!("ThreadCancel", ),
        0x03 => scdef!("ThreadGetID", ),
        0x04 => scdef!("ThreadGetPid", ),
        0x05 => scdef!("ThreadContinue", Uint),
        0x08 => scdef!("ThreadGetPrio", Uint), 
        0x09 => scdef!("ThreadSetPrio", Int, Int),
        0x0a => scdef!("MqueueCreate", Ptr, Int),
        0x0b => scdef!("MqueueDestroy", Ptr),
        0x0c => scdef!("MqueueSend", Uint, Uint, Uint),
        0x0e => scdef!("MqueueRecv", Ptr, Uint),
        0x0f => scdef!("MqueueRegisterHandler", Int, Int, Uint),
        0x10 => scdef!("MqueueDestroyHandler", Ptr, Ptr, Ptr),
        0x11 => scdef!("TimerCreate", Int, Int, Int, Uint),
        0x12 => scdef!("TimerRestart", Uint, Int, Int),
        0x13 => scdef!("TimerStop", Uint),
        0x14 => scdef!("TimerDestroy", Uint),
        0x15 => scdef!("TimerNow", ),
        0x16 => scdef!("HeapCreate", Ptr, Int),
        0x18 => scdef!("HeapAlloc", Int, Uint),
        0x19 => scdef!("HeapAllocAligned", Int, Uint, Uint),
        0x1a => scdef!("HeapFree", Int, Ptr),
        0x1b => scdef!("RegisterDevice", StrPtr, Int),
        0x1c => scdef!("Open", StrPtr, Int),
        0x1d => scdef!("Close", Int),
        0x1e => scdef!("Read", Int, Ptr, Uint),
        0x1f => scdef!("Write", Int, Ptr, Uint),
        0x21 => scdef!("Ioctl", Int, Uint, Ptr, Uint, Ptr, Uint),
        0x22 => scdef!("Ioctlv", Int, Uint, Uint, Uint, Ptr),
        0x2a => scdef!("ResourceReply", Ptr, Uint),
        0x2b => scdef!("SetUid", Int),
        0x2d => scdef!("SetGid", Int),
        0x2f => scdef!("AhbMemFlush", Int),
        0x30 => scdef!("CcAhbMemFlush", Int),
        0x32 => scdef!("EnableIrqDI", ),
        0x33 => scdef!("EnableIrqSDHC", ),
        0x34 => scdef!("EnableIrq", ),
        0x35 => scdef!("IobufPoolAccessNOP", ),
        0x3f => scdef!("SyncBeforeRead", Ptr),
        0x40 => scdef!("SyncAfterWrite", Ptr),
        0x41 => scdef!("PpcBoot", StrPtr),
        0x42 => scdef!("IosBoot", StrPtr),
        0x46 => scdef!("DIResetCheck", ),
        0x47 => scdef!("WhichKernel", Ptr, Ptr),
        0x4d => scdef!("KernelGetVersion", ),
        0x4f => scdef!("VirtToPhys", Ptr),
        0x50 => scdef!("DVDVideoSet", Uint),
        0x51 => scdef!("DVDVideoGet", ),
        0x52 => scdef!("EXICtrlBit4Toggle", Uint),
        0x54 => scdef!("SetAhbProt", Uint),
        0x55 => scdef!("GetBusClock", ),
        0x56 => scdef!("PokeGpio", Uint, Uint),
        0x59 => scdef!("LoadPPC", Ptr),
        0x5a => scdef!("LoadModule", StrPtr),
        0x63 => scdef!("IoscGetData", Uint, Uint, Uint),
        0x68 => scdef!("IoscEncryptAsync", Uint, Uint, Uint),
        0x6a => scdef!("IoscDecryptAsync", Uint, Uint, Uint),
        0x6d => scdef!("IoscGenBlockmac", Uint, Uint, Uint),
        _ => panic!("Couldn't resolve syscall idx={:02x}", idx),
    });
    res
}


/// Resolve information about an IOS syscall and its arguments.
pub fn resolve_syscall(cpu: &mut Cpu, opcd: u32) {
    // Get the syscall index (and ignore some)
    let idx = (opcd & 0x00ff_ffe0) >> 5;
    let res = get_syscall_desc(idx);
    if res.is_none() {
        return;
    }
    let syscall = res.unwrap();
    let mut arg_buf = String::new();
    for (idx, arg) in syscall.arg.iter().enumerate() {
        let val = cpu.reg[idx as u32];
        match arg {
            ArgType::Ptr => { 
                arg_buf.push_str(&format!("0x{:08x}", val));
            },
            ArgType::StrPtr => {
                let s = read_string(cpu, val);
                arg_buf.push_str(&format!("\"{}\"", s));
            },
            ArgType::Int => {
                arg_buf.push_str(&format!("{}", val));
            },
            ArgType::Uint => {
                arg_buf.push_str(&format!("0x{:x}", val));
            },
        }
        if idx < syscall.arg.len()-1 {
            arg_buf.push_str(", ");
        }
    }
    println!("IOS [{:?}] {}({}) (lr={:08x})", 
        ExecutionCtx::from(cpu.read_fetch_pc()),
        syscall.name, arg_buf, cpu.reg[Reg::Lr]
    );
}

