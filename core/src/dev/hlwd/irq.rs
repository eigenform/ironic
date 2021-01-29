
#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Default, Clone)]
#[repr(transparent)]
pub struct IrqBits(pub u32);
impl IrqBits {
    pub fn set(&mut self, irqnum: HollywoodIrq) { 
        self.0 |= irqnum as u32; 
    }
    pub fn toggle(&mut self, irqnum: HollywoodIrq) { 
        self.0 ^= irqnum as u32; 
    }
    pub fn unset(&mut self, irqnum: HollywoodIrq) { 
        self.0 &= !(irqnum as u32); 
    }
    pub fn is_set(&self, irqnum: HollywoodIrq) -> bool {
        (self.0 & irqnum as u32) != 0
    }

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
    /// Output IRQ line to the ARM side; set true when any IRQ is asserted
    pub arm_irq_output: bool,
    /// Output IRQ line to the PPC side; set true when any IRQ is asserted.
    pub ppc_irq_output: bool,

    pub ppc_irq_status: IrqBits,
    pub ppc_irq_enable: IrqBits,

    pub arm_irq_status: IrqBits,
    pub arm_irq_enable: IrqBits,

    pub arm_fiq_enable: IrqBits,
}
impl IrqInterface {

    pub fn read_handler(&self, off: usize) -> u32 {
        match off {
            0x08 => self.arm_irq_status.0,
            0x0c => self.arm_irq_enable.0,
            _ => panic!("Unhandled read on HLWD IRQ interface {:02x}", off),
        }
    }

    pub fn write_handler(&mut self, off: usize, val: u32) {
        match off {
            0x04 => {
                self.ppc_irq_enable.0 = val;
                println!("IRQ PPC enable={:08x}", val);
            },

            0x08 => {
                //println!("IRQ status bits {:08x} cleared", val);
                self.arm_irq_status.0 &= !val;
            },

            // NOTE: When a pin is disabled, does it clear the status bit?
            0x0c => {
                self.arm_irq_enable.0 = val;
                println!("IRQ ARM enable={:08x}", val);
            },

            0x10 => {
                self.arm_fiq_enable.0 = val;
            },
            _ => panic!("Unhandled write {:08x} on HLWD IRQ interface {:02x}", 
                val, off),
        }
        self.update_irq_lines();
    }
}

impl IrqInterface {
    /// Update the state of the output IRQ signal to both CPUs.
    fn update_irq_lines(&mut self) {
        if (self.arm_irq_status.0 & self.arm_irq_enable.0) == 0 {
            self.arm_irq_output = false;
        } else {
            self.arm_irq_output = true;
        }

        if (self.ppc_irq_status.0 & self.ppc_irq_enable.0) == 0 {
            self.ppc_irq_output = false;
        } else {
            self.ppc_irq_output = true;
        }
    }

    /// Returns true if the given IRQ is asserted on the ARM-side.
    pub fn arm_irq_pending(&self, irq: HollywoodIrq) -> bool {
        (self.arm_irq_status.0 & self.arm_irq_enable.0 == 0) &&
            (self.arm_irq_status.0 & irq as u32) == 0
    }

    /// Returns true if the given IRQ is asserted on the PPC-side.
    pub fn ppc_irq_pending(&self, irq: HollywoodIrq) -> bool {
        (self.ppc_irq_status.0 & self.ppc_irq_enable.0 == 0) &&
            (self.ppc_irq_status.0 & irq as u32) == 0
    }

    /// Assert a Hollywood IRQ.
    pub fn assert(&mut self, irq: HollywoodIrq) {
        if self.arm_irq_enable.is_set(irq) { self.arm_irq_status.set(irq); }
        if self.ppc_irq_enable.is_set(irq) { self.ppc_irq_status.set(irq); }
        self.update_irq_lines();
    }
}


