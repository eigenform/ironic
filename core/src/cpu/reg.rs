//! CPU register definitions.

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
#[derive(Clone, Debug, PartialEq, Eq)]
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

    /// Replace the current status program register.
    pub fn write_cpsr(&mut self, target: Psr) { 
        let current_mode = self.cpsr.mode();
        if current_mode != target.mode() {
            self.swap_bank(target.mode());
        }
        self.cpsr = target;
    }
}


/// Functions for swapping between active registers and banked registers
impl RegisterFile {
    /// Swap the currently active registers with some set of banked registers.
    pub fn swap_bank(&mut self, target_mode: CpuMode) {
        use CpuMode::*;
        // Save the current mode's banked registers
        match self.cpsr.mode() {
            Usr | Sys => {
                self.bank.sys[0] = self.r[13];
                self.bank.sys[1] = self.r[14];
            },
            Svc => {
                self.bank.svc[0] = self.r[13];
                self.bank.svc[1] = self.r[14];
            },
            Abt => {
                self.bank.abt[0] = self.r[13];
                self.bank.abt[1] = self.r[14];
            },
            Und => {
                self.bank.und[0] = self.r[13];
                self.bank.und[1] = self.r[14];
            },
            Irq => {
                self.bank.irq[0] = self.r[13];
                self.bank.irq[1] = self.r[14];
            },
            Fiq => {
                self.bank.fiq[0] = self.r[8];
                self.bank.fiq[1] = self.r[9];
                self.bank.fiq[2] = self.r[10];
                self.bank.fiq[3] = self.r[11];
                self.bank.fiq[4] = self.r[12];
                self.bank.fiq[5] = self.r[13];
                self.bank.fiq[6] = self.r[14];
            },
        }

        // Load the target mode's banked registers
        match target_mode {
            Usr | Sys => {
                self.r[13] = self.bank.sys[0];
                self.r[14] = self.bank.sys[1];
            },
            Svc => {
                self.r[13] = self.bank.svc[0];
                self.r[14] = self.bank.svc[1];
            },
            Abt => {
                self.r[13] = self.bank.abt[0];
                self.r[14] = self.bank.abt[1];
            },
            Und => {
                self.r[13] = self.bank.und[0];
                self.r[14] = self.bank.und[1];
            },
            Irq => {
                self.r[13] = self.bank.irq[0];
                self.r[14] = self.bank.irq[1];
            },
            Fiq => {
                self.r[8] = self.bank.fiq[0];
                self.r[9] = self.bank.fiq[1];
                self.r[10] = self.bank.fiq[2];
                self.r[11] = self.bank.fiq[3];
                self.r[12] = self.bank.fiq[4];
                self.r[13] = self.bank.fiq[5];
                self.r[14] = self.bank.fiq[6];
            },
        }
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

