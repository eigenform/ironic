
pub mod register;
pub mod coproc;
pub mod dispatch;
pub mod decode;
pub mod func;
pub mod bits;

/// Container for an ARMv5-compatible CPU.
pub struct Cpu {
    pub pc: u32,
    pub reg: register::RegisterFile,
    pub lut: dispatch::Lut,
}
impl Cpu {
    pub fn new() -> Self { 
        Cpu {
            pc: 0xffff_0000,
            reg: register::RegisterFile::new(),
            lut: dispatch::Lut::new()
        }
    }
}
