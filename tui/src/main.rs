
use ironic_core::dbg::*;
use ironic_core::cpu::*;
use ironic_core::cpu::reg::*;
use ironic_core::bus::*;
use ironic_core::topo::*;
use std::sync::{Arc, RwLock};
use std::fs::File;
use std::io::Write;

fn main() {
    let dbg = Arc::new(RwLock::new(Debugger::new()));
    let mem = Arc::new(RwLock::new(SystemMemory::new()));
    let dev = Arc::new(RwLock::new(SystemDevice::new(dbg.clone())));
    let bus = Arc::new(RwLock::new(Bus::new(
            dbg.clone(), mem.clone(), dev.clone()
    )));


    let mut cpu = Cpu::new(dbg.clone(), bus.clone());
    let mut reg_fd = File::create("/tmp/ironic.log").unwrap();

    for i in 0..10000 {
        // Make a copy of the registers, normalize PC.
        let mut regs = cpu.reg;
        regs.pc -= 8;

        // Write register state
        let state = unsafe {
            std::slice::from_raw_parts_mut(
                (&mut regs as *mut RegisterFile) as *mut u8,
                std::mem::size_of::<RegisterFile>()
            )
        };
        reg_fd.write(state).unwrap();

        // Single step the CPU
        let res = cpu.step();
        match res {
            CpuRes::HaltEmulation => {
                println!("Halted after {} steps", i);
                break;
            },
            CpuRes::StepOk => {
                bus.write().unwrap().step();
            },
        }
    }

}
