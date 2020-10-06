
use ironic_core::dbg::*;
use ironic_core::cpu::*;
use ironic_core::bus::*;
use ironic_core::topo::*;
use std::sync::{Arc, RwLock};

fn main() {
    let dbg = Arc::new(RwLock::new(Debugger::new()));
    let mem = Arc::new(RwLock::new(SystemMemory::new()));
    let dev = Arc::new(RwLock::new(SystemDevice::new(dbg.clone())));
    let bus = Arc::new(RwLock::new(Bus::new(
            dbg.clone(), mem.clone(), dev.clone()
    )));


    let mut cpu = Cpu::new(dbg.clone(), bus.clone());

    for i in 0..800 {
        let res = cpu.step();
        match res {
            CpuRes::HaltEmulation => break,
            CpuRes::StepOk => {
                bus.write().unwrap().step();
            },
        }
    }

}
