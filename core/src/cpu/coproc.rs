//! Coprocessor register definitions and functionality.

/// Set of potential side effects that p15 writes may elicit.
#[derive(Debug)]
pub enum CoprocTask {
    /// The state of the system control register has changed.
    ControlChange,
    /// The state of the translation table base register has changed.
    TtbrChange,
    /// The state of the domain access control register has changed.
    DacrChange,
    /// No side effects on other parts of the system need to be handled.
    None,
}


/// The system control register (p15 register 1).
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ControlRegister(pub u32);
impl ControlRegister {
    pub fn mmu_enabled(&self) -> bool     { (self.0 & 0x0000_0001) != 0 }
    pub fn afault_enabled(&self) -> bool  { (self.0 & 0x0000_0002) != 0 }
    pub fn dcache_enabled(&self) -> bool  { (self.0 & 0x0000_0004) != 0 }
    pub fn wbuffer_enabled(&self) -> bool { (self.0 & 0x0000_0008) != 0 }
    pub fn be_enabled(&self) -> bool      { (self.0 & 0x0000_0080) != 0 }
    pub fn sysprot_enabled(&self) -> bool { (self.0 & 0x0000_0100) != 0 }
    pub fn romprot_enabled(&self) -> bool { (self.0 & 0x0000_0200) != 0 }
    pub fn icache_enabled(&self) -> bool  { (self.0 & 0x0000_1000) != 0 }
    pub fn hivec_enabled(&self) -> bool   { (self.0 & 0x0000_2000) != 0 }
    pub fn thumb_disabled(&self) -> bool  { (self.0 & 0x0000_8000) != 0 }
}

/// Domain access control register (DACR).
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct DACRegister(pub u32);
impl DACRegister {
    /// Return the mode associated with the provided domain index (0-15).
    pub fn domain(&self, idx: u32) -> DomainMode {
        assert!(idx < 16);
        DomainMode::from((self.0 >> (idx * 2)) & 0b11)
    }
}

/// Access control modes associated with some memory domain.
#[derive(Debug)]
pub enum DomainMode {
    /// All accesses generate a domain fault.
    NoAccess = 0b00,
    /// Accesses are checked against permission bits in a TLB entry.
    Client   = 0b01,
    /// Unimplemented/unpredictable for ARMv5
    Reserved = 0b10,
    /// Accesses proceed without checking permission bits in a TLB entry.
    Manager  = 0b11,
}
impl From<u32> for DomainMode {
    fn from(x: u32) -> Self {
        use DomainMode::*;
        match x {
            0b00 => NoAccess, 0b01 => Client,
            0b10 => Reserved, 0b11 => Manager,
            _ => unreachable!("Invalid domain acess mode bits"),
        }
    }
}


/// Registers in the System control coprocessor.
#[derive(Debug)]
pub enum SystemControlReg {
    Control         = 1,
    PageControl     = 2,
    AccessControl   = 3,
    FaultStatus     = 5,
    FaultAddress    = 6,
    CacheControl    = 7,
    TlbControl      = 8,
    CacheLockdown   = 9,
    TlbLockdown     = 10,
}
impl From<u32> for SystemControlReg {
    fn from(x: u32) -> Self {
        use SystemControlReg::*;
        match x {
            1   => Control,
            2   => PageControl,
            3   => AccessControl,
            5   => FaultStatus,
            6   => FaultAddress,
            7   => CacheControl,
            8   => TlbControl,
            9   => CacheLockdown,
            10  => TlbLockdown,
            _ => panic!("Invalid p15 register number {}", x),
        }
    }
}


/// Container for the System Control coprocessor (p15).
pub struct SystemControl {
    /// System control register
    pub c1_ctrl: ControlRegister,

    /// Translation table base register 0
    pub c2_ttbr0: u32,

    /// Domain access control register
    pub c3_dacr: DACRegister,

    /// Fault status register (data)
    pub c5_dfsr: u32,
    /// Fault status register (instruction)
    pub c5_ifsr: u32,

    /// Fault address register (data)
    pub c6_dfar: u32,
}
impl SystemControl {
    pub fn new() -> Self {
        SystemControl {
            c1_ctrl: ControlRegister(0),
            c2_ttbr0: 0,
            c3_dacr: DACRegister(0),
            c5_dfsr: 0,
            c5_ifsr: 0,
            c6_dfar: 0,
        }
    }
    
    pub fn read(&self, reg: u32, crm: u32, opcd2: u32) -> u32 {
        use SystemControlReg::*;
        match SystemControlReg::from(reg) {
            Control => match (crm, opcd2) {
                (0, 0) => self.c1_ctrl.0,
                _ => panic!("Unimpl p15 read {:?} crm={} opcd2={}", 
                    SystemControlReg::from(reg), crm, opcd2),
            },
            _ => panic!("Unimpl p15 read {:?} crm={} opcd2={}", 
                SystemControlReg::from(reg), crm, opcd2),
        }
    }

    pub fn write(&mut self, val: u32, reg: u32, crm: u32, opcd2: u32) 
        -> CoprocTask {
        use SystemControlReg::*;
        match SystemControlReg::from(reg) {
            Control => match (crm, opcd2) {
                (0, 0) => {
                    self.c1_ctrl.0 = val;
                    return CoprocTask::ControlChange;
                },
                _ => panic!("Unimpl P15 write {:08x} {:?} crm={} opcd2={}",
                    val, SystemControlReg::from(reg), crm, opcd2),
            },

            PageControl => match (crm, opcd2) {
                (0, 0) => {
                    self.c2_ttbr0 = val;
                    return CoprocTask::TtbrChange;
                },
                _ => panic!("Unimpl P15 write {:08x} {:?} crm={} opcd2={}",
                    val, SystemControlReg::from(reg), crm, opcd2),
            },

            AccessControl => match (crm, opcd2) {
                (0, 0) => {
                    self.c3_dacr = DACRegister(val);
                    return CoprocTask::DacrChange;
                },
                _ => panic!("Unimpl P15 write {:08x} {:?} crm={} opcd2={}",
                    val, SystemControlReg::from(reg), crm, opcd2),
            },

            FaultStatus => match (crm, opcd2) {
                (0, 0) => self.c5_dfsr = val,
                (0, 1) => self.c5_ifsr = val,
                _ => panic!("Unimpl P15 write {:08x} {:?} crm={} opcd2={}",
                    val, SystemControlReg::from(reg), crm, opcd2),
            },
            FaultAddress => match (crm, opcd2) {
                (0, 0) => self.c6_dfar = val,
                _ => panic!("Unimpl P15 write {:08x} {:?} crm={} opcd2={}",
                    val, SystemControlReg::from(reg), crm, opcd2),
            },

            CacheControl => match (crm, opcd2) {
                (5, 0) => {}, //println!("P15 Invalidate entire icache"),
                (6, 0) => {}, //println!("P15 Invalidate entire dcache"),
                (10, 1) => {}, //println!("P15 Clean data cache line {:08x}", val),
                (10, 4) => {}, //println!("P15 Drain write buffer"),
                _ => panic!("Unimpl P15 write {:08x} {:?} crm={} opcd2={}",
                    val, SystemControlReg::from(reg), crm, opcd2),
            },

            TlbControl => match (crm, opcd2) {
                (7, 0) => {}, //println!("P15 Invalidate entire TLB"),
                _ => panic!("Unimpl P15 write {:08x} {:?} crm={} opcd2={}",
                    val, SystemControlReg::from(reg), crm, opcd2),
            },

            _ => panic!("Unimpl P15 write {:08x} {:?} crm={} opcd2={}", 
                val, SystemControlReg::from(reg), crm, opcd2),
        }
        CoprocTask::None
    }
}



