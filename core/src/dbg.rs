
pub mod symbols;
pub mod ios;

use std::collections::HashMap;
use std::sync::{Arc,RwLock};
use crate::cpu;

#[derive(Debug)]
pub enum ControlMessage {
    AddBreakpoint(u32),
    Step(usize),
}

#[derive(Debug, PartialEq)]
pub enum CpuState {
    Running,
    Halted,
}

/// Container for runtime debugging state.
pub struct Debugger {
    pub cpu_msg: Option<ControlMessage>,
    /// Buffer for memory window.
    pub mem_window: Vec<u8>,
    /// Buffer for log entries.
    pub console_buf: Vec<LogEntry>,
    /// Periodically-updated copy of CPU register state.
    pub reg: cpu::reg::RegisterFile,

    pub sym: HashMap<u32, String>,
}
impl Debugger {
    pub fn new() -> Self {
        Debugger {
            mem_window: Vec::new(),
            console_buf: Vec::new(),
            reg: cpu::reg::RegisterFile::new(),
            cpu_msg: None,
            sym: symbols::parse_symbol_file("boot1c.sym"),
        }
    }

    pub fn get_symbol(&self, addr: u32) -> &str { 
        self.sym.get(&addr).unwrap() 
    }
}

/// The source for a particular entry in the log.
#[derive(Copy, Clone, Debug)]
pub enum LogLevel { 
    Cpu, 
    Emu, 
    Bus,
    Nand,
    Aes,
    Sha,
    Hlwd,
}

/// An entry in the console log.
pub struct LogEntry { 
    pub lvl: LogLevel, 
    pub data: String 
}

/// Write a message to the console log.
pub fn log(dbg: &Arc<RwLock<Debugger>>, lvl: LogLevel, s: &str) {
    let mut debugger = dbg.write().unwrap();
    debugger.console_buf.push(LogEntry{ lvl, data: s.to_string()});
}





