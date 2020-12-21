//! The interpreter backend.

pub mod arm;
pub mod thumb;
pub mod dispatch;
pub mod lut;

use std::sync::{Arc, RwLock};

use crate::lut::*;
use crate::back::*;
use crate::interp::lut::*;
use crate::interp::dispatch::DispatchRes;

use crate::decode::arm::ArmInst;
use crate::decode::thumb::ThumbInst;

use ironic_core::bus::*;
use ironic_core::cpu::{Cpu, CpuRes};
use ironic_core::cpu::reg::Reg;
use ironic_core::cpu::excep::ExceptionType;

/// Backend for interpreting-style emulation. 
///
/// Right now, the main loop works like this:
///
/// - Execute all pending work on the bus
/// - Update the state of any signals from the bus to the CPU
/// - Decode/dispatch an instruction, mutating the CPU state
///
/// For now it's sufficient to perfectly interleave bus and CPU cycles, but
/// maybe at some point it will become more efficient to let dispatched
/// instructions return some hint to the backend (requesting that a bus cycle
/// should be completed before the next instruction).

pub struct InterpBackend {
    /// Lookup tables, used to dispatch instructions.
    pub lut: InterpLut,

    /// Reference to a bus (attached to memories and devices).
    pub bus: Arc<RwLock<Bus>>,

    /// The CPU state.
    pub cpu: Cpu,

    /// Buffer for semi-hosting debug writes
    pub svc_buf: String,
}
impl InterpBackend {
    pub fn new(bus: Arc<RwLock<Bus>>) -> Self {
        InterpBackend {
            svc_buf: String::new(),
            lut: InterpLut::new(),
            cpu: Cpu::new(bus.clone()),
            bus,
        }
    }
}

impl InterpBackend {
    /// Write semi-hosting debug strings to stdout.
    pub fn svc_read(&mut self) {
        use ironic_core::cpu::mmu::prim::{TLBReq, Access};

        // On this exception, r1 contains a pointer to the buffer
        let r1 = self.cpu.reg.r[1];

        // We need to use an out-of-band request to the MMU here
        let paddr = self.cpu.translate(
            TLBReq::new(self.cpu.reg.r[1], Access::Debug)
        );

        // Pull the buffer out of guest memory
        let mut line_buf = [0u8; 16];
        self.bus.write().unwrap().dma_read(paddr, &mut line_buf);

        self.svc_buf += std::str::from_utf8(&line_buf).unwrap();
        if self.svc_buf.find('\n').is_some() {
            let string: String = self.svc_buf.chars()
                .take(self.svc_buf.find('\n').unwrap()).collect();
            println!("SVC {}", string);
            self.svc_buf.clear();
        }
    }

    /// Log IOS syscalls to stdout.
    pub fn syscall_log(&mut self, opcd: u32) {
        println!("IOS syscall {:08x}, lr={:08x}", opcd, self.cpu.reg[Reg::Lr]);
    }

    /// Write the current instruction to stdout.
    pub fn dbg_print(&mut self) {
        let pc = self.cpu.read_fetch_pc();
        if self.cpu.dbg_on && self.cpu.dbg_steps > 0 {
            if self.cpu.reg.cpsr.thumb() {
                let opcd = self.cpu.read16(pc);
                let inst = ThumbInst::decode(opcd);
                match inst {
                    ThumbInst::BlImmSuffix => return,
                    _ => {}
                }
                let name = format!("{:?}", ThumbInst::decode(opcd));
                println!("({:08x}) {:12} {:x?}", opcd, name, self.cpu.reg);
            } else {
                let opcd = self.cpu.read32(pc);
                let name = format!("{:?}", ArmInst::decode(opcd));
                println!("({:08x}) {:12} {:x?}", opcd, name, self.cpu.reg);
            };
            self.cpu.dbg_steps -= 1;
        }
    }

    /// Do a single step of the CPU.
    pub fn cpu_step(&mut self) -> CpuRes {
        assert!((self.cpu.read_fetch_pc() & 1) == 0);

        // Sample the IRQ line at the start of each step
        if !self.cpu.reg.cpsr.irq_disable() && self.cpu.irq_input {
            self.cpu.generate_exception(ExceptionType::Irq);
        }

        // Fetch/decode/execute an ARM or Thumb instruction depending on
        // the state of the Thumb flag in the CPSR.
        let disp_res = if self.cpu.reg.cpsr.thumb() {
            self.dbg_print();
            let opcd = self.cpu.read16(self.cpu.read_fetch_pc());
            let func = self.lut.thumb.lookup(opcd);
            func.0(&mut self.cpu, opcd)
        } else {
            self.dbg_print();
            let opcd = self.cpu.read32(self.cpu.read_fetch_pc());
            if self.cpu.reg.cond_pass(opcd) {
                let func = self.lut.arm.lookup(opcd);
                func.0(&mut self.cpu, opcd)
            } else {
                DispatchRes::CondFailed
            }
        };

        // Depending on the instruction, adjust the program counter
        let cpu_res = match disp_res {
            DispatchRes::RetireOk | 
            DispatchRes::CondFailed => {
                self.cpu.increment_pc(); 
                CpuRes::StepOk
            },
            DispatchRes::RetireBranch => {
                CpuRes::StepOk
            },
            DispatchRes::Exception(e) => {
                self.cpu.generate_exception(e);
                CpuRes::StepException(e)
            },
            DispatchRes::FatalErr => {
                println!("CPU halted at pc={:08x}", self.cpu.read_fetch_pc());
                CpuRes::HaltEmulation
            },
            _ => unreachable!(),
        };

        self.cpu.update_boot_status();
        cpu_res
    }
}

impl Backend for InterpBackend {
    fn run(&mut self) {
        for _step in 0..0x8000_0000usize {
            {
                let mut bus = self.bus.write().unwrap();
                bus.step();
                self.cpu.irq_input = bus.irq_line();
            }
            let res = self.cpu_step();
            match res {
                CpuRes::StepOk => {},
                CpuRes::HaltEmulation => break,
                CpuRes::StepException(e) => {
                    match e {
                        ExceptionType::Swi => self.svc_read(),
                        ExceptionType::Undef(_) => {},
                        _ => panic!("Unimplemented exception type"),
                    }
                },
            }
        }
        println!("CPU stopped at pc={:08x}", self.cpu.read_fetch_pc());
    }
}


