use std::sync::{Arc,RwLock};


use crate::cpu::armv5;

/// Container for state that we will copy out of the emulator thread, which
/// will eventually be consumed by the UI in some way.
pub struct Debugger {

    /// Buffer containing log lines to-be-displayed in some debug console.
    pub console_buf: Vec<LogEntry>,

    /// Copy of the CPU register file
    pub reg: armv5::reg::RegisterFile,
}
impl Debugger {
    pub fn new() -> Self {
        Debugger {
            console_buf: Vec::new(),
            reg: armv5::reg::RegisterFile::new(),
        }
    }
}


#[derive(Debug)]
pub enum LogLevel { Cpu, Emu, Bus }
pub struct LogEntry { 
    pub lvl: LogLevel, 
    pub data: String 
}

pub fn log(dbg: &Arc<RwLock<Debugger>>, lvl: LogLevel, s: &str) {
    let mut debugger = dbg.write().unwrap();
    debugger.console_buf.push(LogEntry{ lvl, data: s.to_string()});
}

