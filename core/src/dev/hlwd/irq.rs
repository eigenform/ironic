
#[repr(u32)]
pub enum HollywoodIrq {
    Timer   = 0x0000_0001,
    Nand    = 0x0000_0002,
    Aes     = 0x0000_0004,
    Sha     = 0x0000_0008,

    Ehci    = 0x0000_0010,
    Ohci0   = 0x0000_0020,
    Ohci1   = 0x0000_0040,
    Sdhc    = 0x0000_0080,

    Wifi    = 0x0000_0100,

    PpcGpio = 0x0000_0400,
    ArmGpio = 0x0000_0800,

    RstBtn  = 0x0002_0000,
    Di      = 0x0004_0000,

    PpcIpc  = 0x4000_0000,
    ArmIpc  = 0x8000_0000,
}

#[repr(transparent)]
pub struct IrqBits(pub u32);
impl IrqBits {
    fn set(&mut self, irqnum: HollywoodIrq) { self.0 |= irqnum as u32; }
    fn toggle(&mut self, irqnum: HollywoodIrq) { self.0 ^= irqnum as u32; }
    fn unset(&mut self, irqnum: HollywoodIrq) { self.0 &= !(irqnum as u32); }

    pub fn timer(&self) -> bool     { (self.0 & 0x0000_0001) != 0 }
    pub fn nand(&self) -> bool      { (self.0 & 0x0000_0002) != 0 }
    pub fn aes(&self) -> bool       { (self.0 & 0x0000_0004) != 0 }
    pub fn sha(&self) -> bool       { (self.0 & 0x0000_0008) != 0 }
    pub fn ehci(&self) -> bool      { (self.0 & 0x0000_0010) != 0 }
    pub fn ochi0(&self) -> bool     { (self.0 & 0x0000_0020) != 0 }
    pub fn ochi1(&self) -> bool     { (self.0 & 0x0000_0040) != 0 }
    pub fn sdhc(&self) -> bool      { (self.0 & 0x0000_0080) != 0 }
    pub fn wifi(&self) -> bool      { (self.0 & 0x0000_0100) != 0 }

    pub fn ppcgpio(&self) -> bool   { (self.0 & 0x0000_0400) != 0 }
    pub fn armgpio(&self) -> bool   { (self.0 & 0x0000_0800) != 0 }

    pub fn rstbtn(&self) -> bool    { (self.0 & 0x0002_0000) != 0 }
    pub fn di(&self) -> bool        { (self.0 & 0x0004_0000) != 0 }
    pub fn ppcipc(&self) -> bool    { (self.0 & 0x4000_0000) != 0 }
    pub fn armipc(&self) -> bool    { (self.0 & 0x8000_0000) != 0 }
}

#[derive(Debug, Default, Clone)]
pub struct IrqInterface {
    pub arm_irq_status: u32,
    pub arm_irq_enable: u32,
    pub arm_fiq_enable: u32,
}
impl IrqInterface {
    pub fn read_handler(&self, off: usize) -> u32 {
        match off {
            0x08 => self.arm_irq_status,
            0x0c => self.arm_irq_enable,
            _ => panic!("Unhandled read on HLWD IRQ interface {:02x}", off),
        }
    }
    pub fn write_handler(&mut self, off: usize, val: u32) {
        match off {
            0x08 => self.arm_irq_status &= !val,
            0x0c => self.arm_irq_enable = val,
            0x10 => self.arm_fiq_enable = val,
            _ => panic!("Unhandled write {:08x} on HLWD IRQ interface {:02x}", 
                val, off),
        }
    }
}

