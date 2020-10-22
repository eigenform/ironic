
use ironic_core::dbg::*;
use ironic_core::cpu::*;
use ironic_core::cpu::reg::*;
use ironic_core::bus::*;
use ironic_core::topo::*;

use std::sync::{Arc, RwLock};
use std::thread::Builder;

//use std::fs::File;
//use std::io::Write;
//use std::time::Instant;
//use std::sync::mpsc::{channel, Sender, Receiver};


fn main() {

    // These are all resources that might be shared between threads.
    let dbg = Arc::new(RwLock::new(Debugger::new()));
    let mem = Arc::new(RwLock::new(SystemMemory::new()));
    let dev = Arc::new(RwLock::new(SystemDevice::new(dbg.clone())));
    let bus = Arc::new(RwLock::new(Bus::new(
            dbg.clone(), mem.clone(), dev.clone()
    )));

    // The CPU runs in this thread, using references to the resources above.
    let emu_dbg = dbg.clone();
    let emu_bus = bus.clone();
    let emu_thread = Builder::new().name("EmuThread".to_owned()).spawn(move || {
        let mut ctx = EmuThreadContext::new(emu_dbg, emu_bus);
        ctx.run_slice(0x4_000_000);
    }).unwrap();

    emu_thread.join().unwrap();

    // Drain messages from the log buffer

    let mut loglines = {
        let mut d = dbg.write().unwrap();
        std::mem::replace(&mut d.console_buf, Vec::new())
    };
    for line in loglines.drain(..) {
        println!("[{:?}] {}", line.lvl, line.data);
    }
}

