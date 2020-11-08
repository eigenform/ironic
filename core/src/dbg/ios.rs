//! Logging for IOS syscalls.
//!
//! Note that a lot of this depends on the version being booted.
//! For now, it's sufficient to just assume we're on IOS58.

extern crate pretty_hex;
use pretty_hex::*;
use crate::cpu::Cpu;
use crate::cpu::mmu::prim::{Access, TLBReq};

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

pub fn get_syscall_desc(opcd: u32) -> SyscallDef {
    let idx = (opcd & 0x00ff_ffe0) >> 5;
    match idx {
        0x02 => scdef!("ThreadCancel", ),
        0x04 => scdef!("ThreadGetPid", ),
        0x09 => scdef!("ThreadSetPrio", Int, Int),
        0x0a => scdef!("MqueueCreate", Ptr, Int),
        0x0e => scdef!("MqueueRecv", Ptr, Int),
        0x0f => scdef!("MqueueRegisterHandler", Int, Int, Uint),
        0x11 => scdef!("TimerCreate", Int, Int, Int, Uint),
        0x18 => scdef!("HeapAlloc", Int, Uint),
        0x1c => scdef!("Open", StrPtr, Int),
        0x1b => scdef!("RegisterDevice", StrPtr, Int),
        0x2b => scdef!("SetUid", Int),
        0x2d => scdef!("SetGid", Int),
        _ => panic!("Couldn't resolve syscall idx={:02x}", idx),
    }
}

/// Read a NUL-terminated string from memory.  
/// 
/// NOTE: This is not particularly rigorous or safe.
pub fn read_string(cpu: &Cpu, ptr: u32) -> String {
    let paddr = cpu.mmu.translate(TLBReq::new(ptr, Access::Debug));

    let mut line_buf = [0u8; 32];
    cpu.mmu.bus.write().unwrap().dma_read(paddr, &mut line_buf);
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
    s.to_string()
}


/// Resolve information about an IOS syscall and its arguments.
pub fn resolve_syscall(cpu: &Cpu, opcd: u32) {
    let syscall = get_syscall_desc(opcd);
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
    println!("IOS {}({})", syscall.name, arg_buf);
}

