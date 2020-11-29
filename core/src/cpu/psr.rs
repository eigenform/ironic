
use crate::cpu::reg::CpuMode;

/// Program status register.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Psr(pub u32);
impl Psr {
    fn set_bit(&mut self, idx: usize, val: bool) {
        self.0 = (self.0 & !(1 << idx)) | (val as u32) << idx
    }

    pub fn mode(&self) -> CpuMode { CpuMode::from(self.0 & 0x1f) }
    pub fn thumb(&self) -> bool { (self.0 & 0x0000_0020) != 0 }
    pub fn fiq_disable(&self) -> bool { (self.0 & 0x0000_0040) != 0 }
    pub fn irq_disable(&self) -> bool { (self.0 & 0x0000_0080) != 0 }

    pub fn q(&self) -> bool { (self.0 & 0x0800_0000) != 0 }
    pub fn v(&self) -> bool { (self.0 & 0x1000_0000) != 0 }
    pub fn c(&self) -> bool { (self.0 & 0x2000_0000) != 0 }
    pub fn z(&self) -> bool { (self.0 & 0x4000_0000) != 0 }
    pub fn n(&self) -> bool { (self.0 & 0x8000_0000) != 0 }

    pub fn set_mode(&mut self, mode: CpuMode) { 
        self.0 = (self.0 & !0x1f) | mode as u32 
    }
    pub fn set_thumb(&mut self, val: bool) { self.set_bit(5, val); }
    pub fn set_fiq_disable(&mut self, val: bool) { self.set_bit(6, val); }
    pub fn set_irq_disable(&mut self, val: bool) { self.set_bit(7, val); }

    pub fn set_q(&mut self, val: bool) { self.set_bit(27, val); }
    pub fn set_v(&mut self, val: bool) { self.set_bit(28, val); }
    pub fn set_c(&mut self, val: bool) { self.set_bit(29, val); }
    pub fn set_z(&mut self, val: bool) { self.set_bit(30, val); }
    pub fn set_n(&mut self, val: bool) { self.set_bit(31, val); }
}


/// Saved program status registers.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SavedStatusBank {
    /// SVC mode saved program status register.
    pub svc: Psr,
    /// ABT mode saved program status register.
    pub abt: Psr,
    /// UND mode saved program status register.
    pub und: Psr,
    /// IRQ mode saved program status register.
    pub irq: Psr,
    /// FIQ mode saved program status register.
    pub fiq: Psr,
}
impl SavedStatusBank {
    pub fn new() -> Self {
        SavedStatusBank {
            svc: Psr(0x0000_0000),
            abt: Psr(0x0000_0000),
            und: Psr(0x0000_0000),
            irq: Psr(0x0000_0000),
            fiq: Psr(0x0000_0000),
        }
    }

    /// Write the SPSR for the provided mode.
    pub fn write(&mut self, mode: CpuMode, val: Psr) {
        use CpuMode::*;
        match mode {
            Svc => self.svc = val,
            Abt => self.abt = val,
            Und => self.und = val,
            Irq => self.irq = val,
            Fiq => self.fiq = val,
            _ => panic!("Invalid mode {:?} for SPSR write", mode),
        }
    }

    /// Read the SPSR for the provided mode.
    pub fn read(&self, mode: CpuMode) -> Psr {
        use CpuMode::*;
        match mode {
            Svc => self.svc,
            Abt => self.abt,
            Und => self.und,
            Irq => self.irq,
            Fiq => self.fiq,
            _ => panic!("Invalid mode {:?} for SPSR read", mode),
        }
    }
}


