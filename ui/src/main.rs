extern crate imgui;
mod support;
use imgui::*;

use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use ironic_core::dbg::{Debugger, LogLevel, log};
use ironic_core::mem::back::BigEndianMemory;
use ironic_core::topo::*;
use ironic_core::cpu::armv5::*;


/// The UI thread loop.
pub fn ui_thread_loop(ui: &mut Ui, dbg: Arc<RwLock<Debugger>>, _run: &mut bool) {
    let console = Window::new(im_str!("Console Output"))
        .size([400.0, 750.0], Condition::FirstUseEver);

    let cpu_ctx = Window::new(im_str!("CPU state"))
        .size([200.0, 350.0], Condition::FirstUseEver);

    console.build(ui, || {
        for entry in dbg.read().unwrap().console_buf.iter() {
            ui.text(format!(
                "[{:?}] {}", entry.lvl, entry.data
            ));
        }
        ui.separator();
    });

    cpu_ctx.build(ui, || {
        let d = dbg.read().unwrap();
        ui.columns(2, im_str!("Registers"), true);
        ui.set_current_column_width(100.0);
        ui.text(format!("pc= {:08x}", d.reg.pc));
        ui.text(format!("ip= {:08x}", d.reg.r[12]));
        ui.text(format!("sp= {:08x}", d.reg.r[13]));
        ui.text(format!("lr= {:08x}", d.reg.r[14]));
        ui.text(format!("r0= {:08x}", d.reg.r[0]));
        ui.text(format!("r1= {:08x}", d.reg.r[1]));
        ui.text(format!("r2= {:08x}", d.reg.r[2]));
        ui.text(format!("r3= {:08x}", d.reg.r[3]));
        ui.text(format!("r4= {:08x}", d.reg.r[4]));
        ui.text(format!("r5= {:08x}", d.reg.r[5]));
        ui.text(format!("r6= {:08x}", d.reg.r[6]));
        ui.text(format!("r7= {:08x}", d.reg.r[7]));
        ui.text(format!("r8= {:08x}", d.reg.r[8]));
        ui.text(format!("r9= {:08x}", d.reg.r[9]));
        ui.text(format!("r10={:08x}", d.reg.r[10]));
        ui.text(format!("r11={:08x}", d.reg.r[11]));
    });

}



/// Top-level emulator thread loop.
pub fn emu_thread_loop(dbg: Arc<RwLock<Debugger>>) {
    let mut cpu = Cpu::new(dbg.clone());
    let mut topology = Topology::new(
        dbg.clone(), 
        "/tmp/boot0.bin"
    );

    for _i in 0..20 {
        thread::sleep(Duration::from_secs(1));
        log(&cpu.dbg, LogLevel::Emu, "hello, world!");
    }
}






fn main() {
    let debugger = Arc::new(RwLock::new(Debugger::new()));

    // Spin a thread for the emulator core.
    let emu_debugger = debugger.clone();
    thread::spawn(move || {
        emu_thread_loop(emu_debugger);
    });

    // Run in the UI thread until we die.
    let ui_system = support::init(file!());
    ui_system.main_loop(move |run, ui| {
        ui_thread_loop(ui, debugger.clone(), run);
    });
}



