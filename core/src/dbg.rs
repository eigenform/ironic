
use std::sync::{Arc,RwLock};
use crate::cpu;

#[derive(Debug, PartialEq)]
pub enum CpuState {
    Running,
    Halted,
}

/// Container for runtime debugging state.
pub struct Debugger {
    pub cpu_state: CpuState,

    pub mem_window: Vec<u8>,

    /// Buffer for log entries.
    pub console_buf: Vec<LogEntry>,

    /// Periodically-updated copy of CPU register state.
    pub reg: cpu::reg::RegisterFile,

}
impl Debugger {
    pub fn new() -> Self {
        Debugger {
            cpu_state: CpuState::Halted,
            mem_window: Vec::new(),
            console_buf: Vec::new(),
            reg: cpu::reg::RegisterFile::new(),
        }
    }
}

/// The source for a particular entry in the log.
#[derive(Copy, Clone, Debug)]
pub enum LogLevel { 
    Cpu, 
    Emu, 
    Bus,
    Nand,
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

