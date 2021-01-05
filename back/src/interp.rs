//! The interpreter backend.

pub mod arm;
pub mod thumb;
pub mod dispatch;
pub mod lut;

use std::sync::{Arc, RwLock};
use std::iter::FromIterator;

use crate::lut::*;
use crate::back::*;
use crate::interp::lut::*;
use crate::interp::dispatch::DispatchRes;

use crate::decode::arm::ArmInst;
use crate::decode::thumb::ThumbInst;

use ironic_core::bus::*;
use ironic_core::cpu::{Cpu, CpuRes, BootStatus};
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
    /// Write semihosting debug strings to stdout.
    pub fn svc_read(&mut self) {
        use ironic_core::cpu::mmu::prim::{TLBReq, Access};

        // On the SVC calls, r1 should contain a pointer to some buffer.
        // They might be virtual addresses, so we need to do an out-of-band
        // request to MMU code in order to resolve the actual location.
        let paddr = self.cpu.translate(
            TLBReq::new(self.cpu.reg.r[1], Access::Debug)
        );

        // Pull the buffer out of guest memory
        let mut line_buf = [0u8; 16];
        self.bus.write().unwrap().dma_read(paddr, &mut line_buf);

        let s = std::str::from_utf8(&line_buf).unwrap()
            .trim_matches(char::from(0));
        self.svc_buf += s;

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
        if self.cpu.dbg_on {
            if self.cpu.reg.cpsr.thumb() {
                let opcd = self.cpu.read16(pc);
                let inst = ThumbInst::decode(opcd);
                match inst {
                    ThumbInst::BlImmSuffix => return,
                    _ => {}
                }
                //let name = format!("{:?}", ThumbInst::decode(opcd));
                //println!("({:08x}) {:12} {:x?}", opcd, name, self.cpu.reg);
                println!("{:?}", self.cpu.reg);
            } else {
                //let opcd = self.cpu.read32(pc);
                //let name = format!("{:?}", ArmInst::decode(opcd));
                //println!("({:08x}) {:12} {:x?}", opcd, name, self.cpu.reg);
                println!("{:?}", self.cpu.reg);
            };
        }
    }

    /// Patch containing a call to ThreadCancel()
    const THREAD_CANCEL_PATCH: [u8; 0x8] = [
        // e3a00000 mov     r0, #0
        //0xe3, 0xa0, 0x00, 0x00,
        // e3a01006 mov     r1, #6
        //0xe3, 0xa0, 0x10, 0x06,
        // e6000050 .word   0xe6000050
        0xe6, 0x00, 0x00, 0x50,
        // e12fff1e bx      lr
        0xe1, 0x2f, 0xff, 0x1e,
    ];

    /// Skyeye intentionally kills a bunch of threads, specifically NCD, KD,
    /// WL, and WD; presumably to avoid having to deal with emulating WLAN.
    pub fn hotpatch_check(&mut self) {
        use ironic_core::cpu::mmu::prim::{TLBReq, Access};
        if self.cpu.boot_status == BootStatus::Kernel {
            let pc = self.cpu.read_fetch_pc();
            let vaddr = match pc {
                0x13d9_0024 | // NCD
                0x13db_0024 | // KD
                0x13ed_0024 | // WL
                0x13eb_0024 => Some(pc), // WD
                _ => None
            };
            if vaddr.is_none() { 
                return; 
            } else {
                let paddr = self.cpu.translate(
                    TLBReq::new(vaddr.unwrap(), Access::Debug)
                );
                println!("DBG hotpatching module entrypoint {:08x}", paddr);
                println!("{:?}", self.cpu.reg);
                self.bus.write().unwrap().dma_write(paddr, 
                    &Self::THREAD_CANCEL_PATCH);
            }
        }
    }

    /// Do a single step of the CPU.
    pub fn cpu_step(&mut self) -> CpuRes {
        assert!((self.cpu.read_fetch_pc() & 1) == 0);

        // Sample the IRQ line and potentially generate an IRQ exception
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
            DispatchRes::RetireBranch => { CpuRes::StepOk },
            DispatchRes::RetireOk | 
            DispatchRes::CondFailed => {
                self.cpu.increment_pc(); 
                CpuRes::StepOk
            },

            // NOTE: Skyeye doesn't take SWI exceptions at all, but I wonder
            // why this is permissible. What does the hardware actually do?
            DispatchRes::Exception(e) => {
                if e == ExceptionType::Swi {
                    self.cpu.increment_pc();
                    CpuRes::Semihosting
                } else {
                    self.cpu.generate_exception(e);
                    CpuRes::StepException(e)
                }
            },

            DispatchRes::FatalErr => {
                println!("CPU halted at pc={:08x}", self.cpu.read_fetch_pc());
                CpuRes::HaltEmulation
            },
        };

        self.cpu.update_boot_status();
        cpu_res
    }
}

impl Backend for InterpBackend {
    fn run(&mut self) {
        for _step in 0..0x8000_0000usize {

            self.hotpatch_check();

            // Take ownership of the bus to deal with any pending tasks
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
                        ExceptionType::Undef(_) => {},
                        ExceptionType::Irq => {},
                        _ => panic!("Unimplemented exception type {:?}", e),
                    }
                },
                CpuRes::Semihosting => {
                    self.svc_read();
                }
            }
        }
        println!("CPU stopped at pc={:08x}", self.cpu.read_fetch_pc());
    }
}


