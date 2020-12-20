
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
        //println!("pc={:08x} Excep {:x?}", self.read_fetch_pc(), e);

        let target_mode = CpuMode::from(e);

        // Get the address of the exception vector
        let target_pc = ExceptionType::get_vector(e);

        // Get the address the exception will return to
        let return_pc = self.read_fetch_pc()
            .wrapping_add(ExceptionType::get_pc_off(e, self.reg.cpsr.thumb()));

        match e {
            ExceptionType::Undef(opcd) => ios::resolve_syscall(self, opcd),
            _ => {},
        }

        // Save the CPSR to the target mode's SPSR, then change mode
        self.reg.save_cpsr(target_mode);
        self.set_mode(target_mode);

        // Disable Thumb and IRQs
        self.reg.cpsr.set_thumb(false);
        self.reg.cpsr.set_irq_disable(true);
        if e == ExceptionType::Fiq {
            self.reg.cpsr.set_fiq_disable(true);
        }

        // Setup the LR and write the new program counter
        self.reg[Reg::Lr] = return_pc;
        self.write_exec_pc(target_pc);
    }

    /// Return from an exception.
    pub fn exception_return(&mut self, dest_pc: u32) {
        assert_ne!(self.reg.cpsr.mode(), CpuMode::Usr);
        assert_ne!(self.reg.cpsr.mode(), CpuMode::Sys);

        //println!("pc={:08x} excep return (cpsr={:08x}, new_cpsr={:08x})", 
        //    self.read_fetch_pc(), self.reg.cpsr.0, 
        //    self.reg.spsr.read(self.reg.cpsr.mode()).0,
        //);

        let current_mode_spsr = self.reg.spsr.read(self.reg.cpsr.mode());
        self.reg.write_cpsr(current_mode_spsr);
        self.write_exec_pc(dest_pc & 0xffff_fffe);
    }
}
