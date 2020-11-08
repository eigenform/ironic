
use crate::cpu::*;
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
