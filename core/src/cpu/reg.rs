//! Implementation of the register file.

use std::ops::{Index, IndexMut};

pub enum Reg { Lr, Sp, Ip, }

/// Top-level container for register state.
#[derive(Copy, Clone)]
pub struct RegisterFile {
    pub pc: u32,
    pub r: [u32; 15],
    pub cpsr: Psr,
}
impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile {
            pc: 0xffff_0000 + 8,
            r: [0; 15],
            cpsr: Psr(0x0000_00d3),
        }
    }

    pub fn cond_pass(&self, opcd: u32) -> bool {
        self.is_cond_satisfied(Cond::from((opcd & 0xf000_0000) >> 28))
    }

    fn is_cond_satisfied(&self, cond: Cond) -> bool {
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

impl Index<u32> for RegisterFile {
    type Output = u32;
    fn index(&self, index: u32) -> &u32 {
        match index {
            0..=14 => &self.r[index as usize],
            _ => panic!("Invalid index {} into register file", index),
        }
    }
}
impl IndexMut<u32> for RegisterFile {
    fn index_mut(&mut self, index: u32) -> &mut u32 {
        match index {
            0..=14 => &mut self.r[index as usize],
            _ => panic!("Invalid index {} into register file", index),
        }
    }
}
impl Index<Reg> for RegisterFile {
    type Output = u32;
    fn index(&self, index: Reg) -> &u32 {
        match index {
            Reg::Ip => &self.r[12],
            Reg::Sp => &self.r[13],
            Reg::Lr => &self.r[14],
        }
    }
}
impl IndexMut<Reg> for RegisterFile {
    fn index_mut(&mut self, index: Reg) -> &mut u32 {
        match index {
            Reg::Ip => &mut self.r[12],
            Reg::Sp => &mut self.r[13],
            Reg::Lr => &mut self.r[14],
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

/// CPU operating mode.
#[derive(Debug, PartialEq, Eq)]
pub enum Mode { 
    Usr = 0b10000, 
    Fiq = 0b10001, 
    Irq = 0b10010, 
    Svc = 0b10011, 
    Abt = 0b10111, 
    Und = 0b11011, 
    Sys = 0b11111,
}
impl Mode {
    pub fn is_exception(self) -> bool { self != Mode::Usr && self != Mode::Sys }
    pub fn is_privileged(self) -> bool { self != Mode::Usr }
}
impl From<u32> for Mode {
    fn from(x: u32) -> Self {
        use Mode::*;
        match x {
            0b10000 => Usr,
            0b10001 => Fiq,
            0b10010 => Irq,
            0b10011 => Svc,
            0b10111 => Abt,
            0b11011 => Und,
            0b11111 => Sys,
            _ => panic!("Invalid mode bits {:08x}", x),
        }
    }
}

/// Program status register.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Psr(u32);
impl Psr {
    fn set_bit(&mut self, idx: usize, val: bool) {
        self.0 = (self.0 & !(1 << idx)) | (val as u32) << idx
    }

    pub fn mode(&self) -> Mode { Mode::from(self.0 & 0x1f) }
    pub fn thumb(&self) -> bool { (self.0 & 0x0000_0020) != 0 }
    pub fn fiq_disable(&self) -> bool { (self.0 & 0x0000_0040) != 0 }
    pub fn irq_disable(&self) -> bool { (self.0 & 0x0000_0080) != 0 }
    pub fn imp_disable(&self) -> bool { (self.0 & 0x0000_0100) != 0 }

    pub fn q(&self) -> bool { (self.0 & 0x0800_0000) != 0 }
    pub fn v(&self) -> bool { (self.0 & 0x1000_0000) != 0 }
    pub fn c(&self) -> bool { (self.0 & 0x2000_0000) != 0 }
    pub fn z(&self) -> bool { (self.0 & 0x4000_0000) != 0 }
    pub fn n(&self) -> bool { (self.0 & 0x8000_0000) != 0 }

    pub fn set_mode(&mut self, mode: Mode) { self.0 = (self.0 & !0x1f) | mode as u32 }
    pub fn set_thumb(&mut self, val: bool) { self.set_bit(5, val); }
    pub fn set_fiq_disable(&mut self, val: bool) { self.set_bit(6, val); }
    pub fn set_irq_disable(&mut self, val: bool) { self.set_bit(7, val); }
    pub fn set_imp_disable(&mut self, val: bool) { self.set_bit(8, val); }

    pub fn set_q(&mut self, val: bool) { self.set_bit(27, val); }
    pub fn set_v(&mut self, val: bool) { self.set_bit(28, val); }
    pub fn set_c(&mut self, val: bool) { self.set_bit(29, val); }
    pub fn set_z(&mut self, val: bool) { self.set_bit(30, val); }
    pub fn set_n(&mut self, val: bool) { self.set_bit(31, val); }
}





