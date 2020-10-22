extern crate imgui;
mod support;
use imgui::*;

use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::ops::Index;

use ironic_core::dbg::*;
use ironic_core::cpu::*;
use ironic_core::bus::*;
use ironic_core::topo::*;

/// State related to drawing the UI.
pub struct UiState {
    // Channel to CPU thread
    pub tx: Sender<CpuMsg>,

    // Memory window state
    pub addr_str: ImString,
    pub mem_addr: u32,
    pub mem_buf: [u8; 0x100],

    // Console/log state
    pub log_emu: bool,
    pub log_cpu: bool,
    pub log_bus: bool,
    pub log_nand: bool,
    pub log_hlwd: bool,
    pub console_str: ImString,
}
impl UiState {
    pub fn new(tx: Sender<CpuMsg>) -> Self {
        UiState {
            tx,
            addr_str: ImString::with_capacity(8),
            mem_buf: [0; 0x100],
            mem_addr: 0x0000_0000,

            log_emu: true,
            log_cpu: true,
            log_bus: true,
            log_nand: true,
            log_hlwd: true,
            console_str: ImString::with_capacity(64),
        }
    }
}
impl Index<LogLevel> for UiState {
    type Output = bool;
    fn index(&self, index: LogLevel) -> &bool {
        match index {
            LogLevel::Cpu => &self.log_cpu,
            LogLevel::Emu => &self.log_emu,
            LogLevel::Bus => &self.log_bus,
            LogLevel::Nand => &self.log_nand,
            LogLevel::Hlwd => &self.log_hlwd,
            _ => panic!(),
        }
    }
}

/// Render a line of hexdump data from some slice.
pub fn hexdump_line(addr: u32, buf: &[u8]) -> String {
    let mut res = String::new();
    res.push_str(&format!("{:08x}: ", addr));
    for byte in buf.iter() {
        res.push_str(&format!("{:02x} ", *byte));
    }
    for byte in buf.iter() { 
        if byte.is_ascii() && !byte.is_ascii_control() {
            res.push_str(&format!("{}", std::ascii::escape_default(*byte)));
        } else {
            res.push_str(".");
        }
    }
    res
}

/// Format a hex string into u32
pub fn hex_to_u32(s: &str) -> u32 {
    let prefixed: Vec<&str> = s.split("x").collect();

    // Catch cases where we prefix with '0x'
    if prefixed.len() > 1 {
        u32::from_str_radix(prefixed[1], 16).unwrap_or(0)
    } else {
        u32::from_str_radix(s, 16).unwrap_or(0)
    }
}

pub fn handle_console_command(ui: &Ui, state: &mut UiState, 
    dbg: &Arc<RwLock<Debugger>>) -> Option<CpuMsg> {
    let tokens: Vec<&str> = state.console_str.to_str().split_whitespace().collect();

    let cmd = match tokens[0] {
        "s" | "step" => {
            if tokens.len() > 1 {
                let num_steps = usize::from_str_radix(tokens[1], 10).unwrap_or(0);
                Some(CpuMsg::Step(num_steps))
            } else if tokens.len() == 1 {
                Some(CpuMsg::Step(1))
            } else {
                None
            }
        },
        "b" | "break" => {
            if tokens.len() > 1 {
                let bp_addr = hex_to_u32(tokens[1]);
                Some(CpuMsg::Break(bp_addr))
            } else {
                None
            }
        },
        _ => None,
    };
    state.console_str.clear();
    cmd
}

/// Build the console window.
pub fn build_console_window(ui: &Ui, dbg: &Arc<RwLock<Debugger>>, 
    state: &mut UiState) {

    ui.checkbox(im_str!("Emulator"), &mut state.log_emu);
    ui.same_line(0.0);
    ui.checkbox(im_str!("CPU"), &mut state.log_cpu);
    ui.same_line(0.0);
    ui.checkbox(im_str!("Bus"), &mut state.log_bus);
    ui.same_line(0.0);
    ui.checkbox(im_str!("NAND"), &mut state.log_nand);
    ui.same_line(0.0);
    ui.checkbox(im_str!("Hlwd"), &mut state.log_hlwd);


    ChildWindow::new("log").size([0.0, 350.0]).border(true).build(ui, || {
        for entry in dbg.read().unwrap().console_buf.iter() {
            if state[entry.lvl] {
                ui.text(format!("[{:?}] {}", entry.lvl, entry.data));
            }
        }
        if ui.scroll_y() >= ui.scroll_max_y() {
            ui.set_scroll_here_y_with_ratio(0.0);
        }
    });

    if ui.input_text(im_str!(""), &mut state.console_str)
        .enter_returns_true(true).build() {
        let cmd = handle_console_command(ui, state, &dbg);
        if cmd.is_some() {
            match state.tx.send(cmd.unwrap()) {
                Ok(_) => {},
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}


/// Build the CPU state window.
pub fn build_cpu_window(ui: &Ui, dbg: &Arc<RwLock<Debugger>>) {
    let d = dbg.read().unwrap();
    ui.columns(2, im_str!("Registers"), true);
    ui.set_current_column_width(200.0);

    // You probably want to adjust for the fact that it's ahead
    ui.text(format!("pc= {:08x}", d.reg.pc - 8));

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
}

/// Build the memory window.
pub fn build_mem_window(ui: &Ui, bus: &Arc<RwLock<Bus>>, state: &mut UiState) {

    // On 'Enter', read memory at the address into our buffer
    ui.set_next_item_width(100.0);
    if ui.input_text(im_str!("Address"), &mut state.addr_str)
        .enter_returns_true(true).chars_hexadecimal(true).build() {

            // Convert input string to u32, aligned to 0x10 bytes
            let addr = u32::from_str_radix(state.addr_str.to_str(), 16)
                .unwrap() & 0xffff_fff0;
            state.mem_addr = addr;

            // If this address is valid, read 0x100 bytes into our buffer.
            let mut busref = bus.write().unwrap();
            if busref.decode_phys_addr(addr).is_some() {
                busref.dma_read(addr, &mut state.mem_buf);
            }
    }

    // Render the buffer
    ui.separator();
    for i in 0..0x10 {
        let addr = state.mem_addr + (i as u32 * 0x10);
        let cur = i * 0x10;
        let tail = (i * 0x10) + 0x10;
        ui.text(hexdump_line(addr, &state.mem_buf[cur..tail]));
    }
}

/// The UI thread loop.
pub fn ui_thread_loop(ui: &mut Ui, state: &mut UiState, 
    dbg: Arc<RwLock<Debugger>>, bus: Arc<RwLock<Bus>>, _run: &mut bool) {

    let cpu_ctx = Window::new(im_str!("CPU state"))
        .position([5.0, 5.0], Condition::Always)
        .size([500.0, 500.0], Condition::Always);

    let mem_win = Window::new(im_str!("Memory window"))
        .position([520.0, 5.0], Condition::Always)
        .size([860.0, 500.0], Condition::Always);

    let console = Window::new(im_str!("Console Output"))
        .menu_bar(true)
        .position([5.0, 510.0], Condition::Always)
        .size([1195.0, 500.0], Condition::Always);

    // Render all of the windows
    console.build(ui, || { build_console_window(ui, &dbg, state) });
    mem_win.build(ui, || { build_mem_window(ui, &bus, state) });
    cpu_ctx.build(ui, || { build_cpu_window(ui, &dbg) });
}

#[derive(Clone, Copy, Debug)]
pub enum CpuMsg {
    Step(usize),
    Break(u32),
}


/// Top-level emulator thread loop.
pub fn emu_thread_loop(rx: Receiver<CpuMsg>, dbg: Arc<RwLock<Debugger>>, 
    bus: Arc<RwLock<Bus>>) {
    let mut cpu = Cpu::new(dbg.clone(), bus.clone());
    let mut bp: Option<u32> = None;

    'main: loop {

        // Just block here for a message
        let msg = rx.recv();
        if msg.is_err() {
            panic!("what {:?}", msg.unwrap());
        }

        match msg.unwrap() {

            // Set a breakpoint
            CpuMsg::Break(bp_addr) => {
                log(&dbg, LogLevel::Emu, &format!(
                    "Setting breakpoint at 0x{:08x}", bp_addr));
                bp = Some(bp_addr);
            },

            // Run the CPU for some number of steps
            CpuMsg::Step(num_steps) => {
                for _i in 0..num_steps {
                    let res = cpu.step();
                    match res { 
                        CpuRes::HaltEmulation => break 'main,
                        CpuRes::StepOk => {
                            bus.write().unwrap().step();
                        },
                    }
                    if bp.is_some() && (cpu.read_fetch_pc() == bp.unwrap()) {
                        break;
                    }
                }
            },
        }
    }

    log(&cpu.dbg, LogLevel::Emu, "Emulation thread halted");
}


fn main() {
    let (ctrl_tx, ctrl_rx) = channel::<CpuMsg>();
    let debugger = Arc::new(RwLock::new(Debugger::new()));
    let mem = Arc::new(RwLock::new(SystemMemory::new()));
    let dev = Arc::new(RwLock::new(SystemDevice::new(debugger.clone())));
    let bus = Arc::new(RwLock::new(Bus::new(
        debugger.clone(), 
        mem.clone(), 
        dev.clone()
    )));

    // Spin up a thread for the emulator core.
    let emu_debugger = debugger.clone();
    let emu_bus = bus.clone();
    thread::spawn(move || {
        emu_thread_loop(ctrl_rx, emu_debugger, emu_bus);
    });

    // Run in the UI thread until we die, I guess.
    let ui_system = support::init(file!());
    let mut ui_state = UiState::new(ctrl_tx);
    ui_system.main_loop(move |run, ui| {
        ui_thread_loop(ui, 
            &mut ui_state, 
            debugger.clone(), 
            bus.clone(), 
            run
        );
    });
}



