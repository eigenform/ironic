//! Implementation of exception behavior.

use crate::cpu::*;
use crate::dbg::ios;
use crate::cpu::reg::*;

/// Different types of exceptions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExceptionType {
    //Reset,
    Undef(u32),
    Swi,
    Pabt,
    Dabt,
    Irq,
    Fiq,
}

impl ExceptionType {
    /// Return the exception vector address for this exception.
    pub fn get_vector(self) -> u32 {
        use ExceptionType::*;
        match self {
            //Reset => 0xffff_0000,
            Undef(_) => 0xffff_0004,
            Swi   => 0xffff_0008,
            Pabt  => 0xffff_000c,
            Dabt  => 0xffff_0010,
            Irq   => 0xffff_0018,
            Fiq   => 0xffff_001c,
        }
    }

    /// Get the offset from the PC associated with this type of exeception.
    pub fn get_pc_off(self, thumb: bool) -> u32 {
        use ExceptionType::*;
        //if self == Reset { 
        //    panic!("Reset exceptions are unimplemented"); 
        //}
        if thumb { 
            match self { 
                Swi | Undef(_) => 2, 
                Pabt | Fiq | Irq => 4, 
                Dabt => 8, 
            }
        } else { 
            match self { 
                Swi | Undef(_) | Pabt | Fiq | Irq => 4, 
                Dabt => 8, 
            }
        }
    }
}

/// For obtaining the CPU mode associated with some type of exception.
impl From<ExceptionType> for CpuMode {
    fn from(e: ExceptionType) -> Self {
        use ExceptionType::*;
        match e {
            //Reset => CpuMode::Svc,
            Undef(_) => CpuMode::Und,
            Swi   => CpuMode::Svc,
            Pabt  => CpuMode::Abt,
            Dabt  => CpuMode::Abt,
            Irq   => CpuMode::Irq,
            Fiq   => CpuMode::Fiq,
        }
    }
}

/// Define the behavior for dealing with exceptions.
impl Cpu {

    /// Change CPU state to reflect the fact that we've entered an exception.
    pub fn generate_exception(&mut self, e: ExceptionType) {
        assert_ne!(e, ExceptionType::Swi);
        let current_pc = self.read_fetch_pc();

        let old_cpsr = self.reg.cpsr;
        let target_mode = CpuMode::from(e);
        let target_pc = ExceptionType::get_vector(e);

        // Get the address the exception will return to
        let return_pc = self.read_fetch_pc().wrapping_add(
            ExceptionType::get_pc_off(e, self.reg.cpsr.thumb()));


        match e {
            ExceptionType::Undef(opcd) => ios::resolve_syscall(self, opcd),
            _ => {},
        }


        // Build the new CPSR for the target mode and swap into it
        let mut new_cpsr = old_cpsr;
        new_cpsr.set_mode(target_mode);
        new_cpsr.set_thumb(false);
        new_cpsr.set_irq_disable(true);
        if e == ExceptionType::Fiq {
            new_cpsr.set_fiq_disable(true);
        }
        self.reg.write_cpsr(new_cpsr);

        // Save the old CPSR in the exception mode's SPSR bank
        self.reg.spsr.write(target_mode, old_cpsr);

        // The return value is stored in the target mode's LR
        self.reg[Reg::Lr] = return_pc;
        // The exception vector is written to the program counter 
        self.write_exec_pc(target_pc);

        if self.current_exception.is_none() {
            self.current_exception = Some(e);
        } else {
            panic!("pc={:08x} CPU tried to take {:x?} exception inside {:x?} exception",
                current_pc, e, self.current_exception.unwrap());
        }

    }

    /// Return from an exception.
    pub fn exception_return(&mut self, dest_pc: u32) {
        assert_ne!(self.reg.cpsr.mode(), CpuMode::Usr);
        assert_ne!(self.reg.cpsr.mode(), CpuMode::Sys);

        let current_mode = self.reg.cpsr.mode();
        let spsr = self.reg.spsr.read(current_mode);
        let target_mode = spsr.mode();
        self.reg.write_cpsr(spsr);
        self.write_exec_pc(dest_pc & 0xffff_fffe);

        if self.current_exception.is_some() {
            self.current_exception = None
        } else {
            println!("CPU returned from nonexistent exception");
        }
    }
}
