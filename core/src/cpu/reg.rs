
use crate::cpu::psr::*;

/// Token for a particular register.
pub enum Reg { Lr, Sp, Ip }

/// CPU operating mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CpuMode { 
    Usr = 0b10000, 
    Fiq = 0b10001, 
    Irq = 0b10010, 
    Svc = 0b10011, 
    Abt = 0b10111, 
    Und = 0b11011, 
    Sys = 0b11111,
}
impl CpuMode {
    pub fn is_privileged(self) -> bool { self != CpuMode::Usr }
    pub fn is_exception(self) -> bool { 
        self != CpuMode::Usr && self != CpuMode::Sys 
    }
}
impl From<u32> for CpuMode {
    fn from(x: u32) -> Self {
        use CpuMode::*;
        match x {
            0b10000 => Usr, 0b10001 => Fiq,
            0b10010 => Irq, 0b10011 => Svc,
            0b10111 => Abt, 0b11011 => Und,
            0b11111 => Sys,
            _ => panic!("Invalid mode bits {:08x}", x),
        }
    }
}

/// Condition field used when decoding instructions.
#[derive(Debug, PartialEq, Eq)]
pub enum Cond {
    EQ = 0b0000, NE = 0b0001,
    CS = 0b0010, CC = 0b0011,
    MI = 0b0100, PL = 0b0101,
    VS = 0b0110, VC = 0b0111,
    HI = 0b1000, LS = 0b1001,
    GE = 0b1010, LT = 0b1011,
    GT = 0b1100, LE = 0b1101,
    AL = 0b1110,
}
impl From<u32> for Cond {
    fn from(x: u32) -> Self {
        use Cond::*;
        match x {
            0b0000 => EQ, 0b0001 => NE,
            0b0010 => CS, 0b0011 => CC,
            0b0100 => MI, 0b0101 => PL,
            0b0110 => VS, 0b0111 => VC,
            0b1000 => HI, 0b1001 => LS,
            0b1010 => GE, 0b1011 => LT,
            0b1100 => GT, 0b1101 => LE,
            0b1110 => AL,
            _ => panic!("Invalid condition bits {:08x}", x),
        }
    }
}


/// The set of banked registers for all operating modes.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct RegisterBank {
    /// General-purpose registers (shared among all modes).
    pub gen: [u32; 13],

    pub sys: [u32; 2],
    pub svc: [u32; 2],
    pub abt: [u32; 2],
    pub und: [u32; 2],
    pub irq: [u32; 2],
    pub fiq: [u32; 8],
}
impl RegisterBank {
    /// Return a mutable iterator over the banks used by the given mode.
    pub fn get_mode_iter(&mut self, mode: CpuMode) -> impl Iterator<Item=&mut u32> {
        use CpuMode::*;
        match mode {
            Usr | 
            Sys => self.gen.iter_mut().take(13).chain(self.sys.iter_mut()),
            Svc => self.gen.iter_mut().take(13).chain(self.svc.iter_mut()),
            Abt => self.gen.iter_mut().take(13).chain(self.abt.iter_mut()),
            Und => self.gen.iter_mut().take(13).chain(self.und.iter_mut()),
            Irq => self.gen.iter_mut().take(13).chain(self.irq.iter_mut()),
            Fiq => self.gen.iter_mut().take(7).chain(self.fiq.iter_mut()),
        }
    }
}


/// Top-level container for register state.
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct RegisterFile {
    /// The currently-active set of general-purpose registers.
    pub r: [u32; 15],
    /// The program counter.
    pub pc: u32,
    /// The set of banked registers.
    pub bank: RegisterBank,
    /// The current program status register.
    pub cpsr: Psr,
    /// The saved program status registers.
    pub spsr: SavedStatusBank,
}
impl RegisterFile {
    pub fn new() -> Self {
        let mut init_cpsr = Psr(0x0000_0000);
        init_cpsr.set_mode(CpuMode::Svc);
        init_cpsr.set_thumb(false);
        init_cpsr.set_fiq_disable(true);
        init_cpsr.set_irq_disable(true);

        RegisterFile {
            r: [0; 15],
            pc: 0xffff_0000 + 8,
            cpsr: init_cpsr,
            bank: RegisterBank::default(),
            spsr: SavedStatusBank::new(),
        }
    }
}

/// Functions for dealing with the current program status register.
impl RegisterFile {

    /// Write the current status program register.
    pub fn write_cpsr(&mut self, target: Psr) { 
        if self.cpsr.mode() != target.mode() {
            self.swap_bank(target.mode());
        }
        //println!("CPU Wrote cpsr={:08x}, mode={:?}", target.0, target.mode());
        self.cpsr = target;
    }

    /// Save the current CPSR to some mode's SPSR.
    pub fn save_cpsr(&mut self, target_mode: CpuMode) {
        self.spsr.write(target_mode, self.cpsr);
    }
}


/// Functions for swapping between active registers and banked registers
impl RegisterFile {
    /// Swap the currently active registers with some set of banked registers.
    pub fn swap_bank(&mut self, target_mode: CpuMode) {
        self.save_current_bank();
        self.load_bank(target_mode);
    }

    /// Save active registers to the bank for the current mode.
    fn save_current_bank(&mut self) {
        let mut iter = self.bank.get_mode_iter(self.cpsr.mode());
        for i in 0..15 { *iter.next().unwrap() = self.r[i]; }
    }

    /// Load the bank for the provided mode into the active registers.
    fn load_bank(&mut self, target_mode: CpuMode) {
        let mut iter = self.bank.get_mode_iter(target_mode);
        for i in 0..15 { self.r[i] = *iter.next().unwrap(); }
    }
}


/// These functions are used for determining whether or not some condition is
/// satisfied when dispatching/executing some instruction.
impl RegisterFile {
    pub fn cond_pass(&self, opcd: u32) -> bool {
        self.is_cond_satisfied(Cond::from((opcd & 0xf000_0000) >> 28))
    }
    pub fn is_cond_satisfied(&self, cond: Cond) -> bool {
        use Cond::*;
        match cond {
            EQ => self.cpsr.z(), NE => !self.cpsr.z(),
            CS => self.cpsr.c(), CC => !self.cpsr.c(),
            MI => self.cpsr.n(), PL => !self.cpsr.n(),
            VS => self.cpsr.v(), VC => !self.cpsr.v(),

            HI => self.cpsr.c() && !self.cpsr.z(), 
            LS => !self.cpsr.c() || self.cpsr.z(),

            GE => self.cpsr.n() == self.cpsr.v(), 
            LT => self.cpsr.n() != self.cpsr.v(),

            GT => !self.cpsr.z() && (self.cpsr.n() == self.cpsr.v()), 
            LE => self.cpsr.z() || (self.cpsr.n() != self.cpsr.v()),
            AL => true,
        }
    }
}

impl std::ops::Index<u32> for RegisterFile {
    type Output = u32;
    fn index(&self, index: u32) -> &u32 {
        match index {
            0..=14 => &self.r[index as usize],
            _ => panic!("Invalid index {} into register file", index),
        }
    }
}
impl std::ops::IndexMut<u32> for RegisterFile {
    fn index_mut(&mut self, index: u32) -> &mut u32 {
        match index {
            0..=14 => &mut self.r[index as usize],
            _ => panic!("Invalid index {} into register file", index),
        }
    }
}
impl std::ops::Index<u16> for RegisterFile {
    type Output = u32;
    fn index(&self, index: u16) -> &u32 {
        match index {
            0..=14 => &self.r[index as usize],
            _ => panic!("Invalid index {} into register file", index),
        }
    }
}
impl std::ops::IndexMut<u16> for RegisterFile {
    fn index_mut(&mut self, index: u16) -> &mut u32 {
        match index {
            0..=14 => &mut self.r[index as usize],
            _ => panic!("Invalid index {} into register file", index),
        }
    }
}
impl std::ops::Index<Reg> for RegisterFile {
    type Output = u32;
    fn index(&self, index: Reg) -> &u32 {
        match index {
            Reg::Ip => &self.r[12],
            Reg::Sp => &self.r[13],
            Reg::Lr => &self.r[14],
        }
    }
}
impl std::ops::IndexMut<Reg> for RegisterFile {
    fn index_mut(&mut self, index: Reg) -> &mut u32 {
        match index {
            Reg::Ip => &mut self.r[12],
            Reg::Sp => &mut self.r[13],
            Reg::Lr => &mut self.r[14],
        }
    }
}

impl std::fmt::Debug for RegisterFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pc = if self.cpsr.thumb() { self.pc - 4 } else { self.pc - 8 };
        let cpsr = format!("[{:?}|{}{}{}|{}{}{}{}{}]", self.cpsr.mode(),
            if self.cpsr.thumb() { "T" } else { "-" },
            if self.cpsr.irq_disable() { "-" } else { "I" },
            if self.cpsr.fiq_disable() { "-" } else { "F" },
            if self.cpsr.n() { "N" } else { "-" },
            if self.cpsr.z() { "Z" } else { "-" },
            if self.cpsr.c() { "C" } else { "-" },
            if self.cpsr.v() { "V" } else { "-" },
            if self.cpsr.q() { "Q" } else { "-" },
        );
        write!(f, "{} {:08x}: {:08x?}", cpsr, pc, self.r)?;
        Ok(())
    }
}




