
use ironic_core::bus::*;
use ironic_backend::interp::*;
use ironic_backend::jit::*;
use ironic_backend::back::*;

use std::sync::{Arc, RwLock};
use std::thread::Builder;
use std::env;

pub enum BackendType {
    Interpreter,
    JIT
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} {{interp|jit}}", args[0]);
        return;
    }

    // Let the user specify the backend
    let backend = match args[1].as_str() {
        "interp" => BackendType::Interpreter,
        "jit" => BackendType::JIT,
        _ => {
            println!("usage: {} {{interp|jit}}", args[0]);
            return;
        },
    };

    // All of the allocations live here, and we share references
    // between any threads we spin up.
    let bus = Arc::new(RwLock::new(Bus::new()));

    // Fork off the backend thread
    let emu_bus = bus.clone();
    let emu_thread = match backend {
        BackendType::Interpreter => {
            Builder::new().name("EmuThread".to_owned()).spawn(move || {
                let mut back = InterpBackend::new(emu_bus);
                back.run();
            }).unwrap()
        },
        BackendType::JIT => {
            Builder::new().name("EmuThread".to_owned()).spawn(move || {
                let mut back = JitBackend::new(emu_bus);
                back.run();
            }).unwrap()
        },
        _ => panic!("unimplemented backend"),
    };
    emu_thread.join().unwrap();

    let bus_ref = bus.write().unwrap();
    bus_ref.sram0.dump("/tmp/sram0.bin");
    bus_ref.sram1.dump("/tmp/sram1.bin");
    bus_ref.mem1.dump("/tmp/mem1.bin");
    bus_ref.mem2.dump("/tmp/mem2.bin");
}

