//! Coprocessor register definitions and functionality.

/// Container for the System Control coprocessor (p15).
pub struct SystemControl {
    pub cfg: ConfigRegister,
}
impl SystemControl {
    pub fn new() -> Self {
        SystemControl {
            cfg: ConfigRegister(0),
        }
    }
    
    pub fn read(&self, reg: u32, _crm: u32, _opcd2: u32) -> u32 {
        use SystemControlReg::*;
        match SystemControlReg::from(reg) {
            Config => self.cfg.0,
            _ => panic!("Unimpl p15 read on {:?}", SystemControlReg::from(reg)),
        }
    }

    pub fn write(&mut self, val: u32, reg: u32, _crm: u32, _opcd2: u32) {
        use SystemControlReg::*;
        match SystemControlReg::from(reg) {
            Config => self.cfg.0 = val,

            // NOTE: I'm leaving these unimplemented for now.
            TlbControl |
            PageControl | 
            AccessControl | 
            FaultStatus |
            FaultAddress |
            CacheControl => { 
                println!("CPU p15 write {:08x} to reg={:?} crm={} opcd2={}", 
                    val, SystemControlReg::from(reg), _crm, _opcd2); 
            },

            _ => panic!("Unimpl p15 write to {:?}", SystemControlReg::from(reg)),
        }
    }
}


/// Registers in the System control coprocessor.
#[derive(Debug)]
pub enum SystemControlReg {
    Config          = 1,
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
            1   => Config,
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

/// Cache management functions on p15 cr7.
pub enum CacheControlFunc {
    IcacheInvalidateGlobal,     // cr7, cr5, 0
    DcacheInvalidateGlobal      // cr7, cr6, 0
}
//impl CacheControlFunc {
//    fn from_u32(crm: u32, opcd2: u32) -> Self {
//        use CacheControlFunc::*;
//        match (crm, opcd2) {
//            (5, 0) => IcacheInvalidateGlobal,
//            (6, 0) => DcacheInvalidateGlobal,
//            _ => panic!("Unimpl cache control function crm={}, opcd2={}", 
//                crm, opcd2),
//        }
//    }
//}


#[repr(u32)]
pub enum CRFlag {
    MmuEnabled          = 0x0000_0001,
    AlignFault          = 0x0000_0002,
    UnifiedCache        = 0x0000_0004,
    WriteBuffer         = 0x0000_0008,
    BigEndian           = 0x0000_0080,
    BranchPrediction    = 0x0000_0800,
    ICache              = 0x0000_1000,
    HighVectors         = 0x0000_2000,
    RoundRobin          = 0x0000_4000,
    InterworkDisable    = 0x0000_8000,
}


#[repr(transparent)]
pub struct ConfigRegister(u32);
impl ConfigRegister {
    fn set(&mut self, flag: CRFlag) { self.0 |= flag as u32; }
    fn toggle(&mut self, flag: CRFlag) { self.0 ^= flag as u32; }
    fn unset(&mut self, flag: CRFlag) { self.0 &= !(flag as u32); }

    fn set_bit(&mut self, idx: usize, val: bool) {
        self.0 = (self.0 & !(1 << idx)) | (val as u32) << idx
    }

    pub fn m(&self) -> bool { (self.0 & 0x0000_0001) != 0 }
    pub fn a(&self) -> bool { (self.0 & 0x0000_0002) != 0 }
    pub fn c(&self) -> bool { (self.0 & 0x0000_0004) != 0 }
    pub fn w(&self) -> bool { (self.0 & 0x0000_0008) != 0 }
    pub fn p(&self) -> bool { (self.0 & 0x0000_0010) != 0 }
    pub fn d(&self) -> bool { (self.0 & 0x0000_0020) != 0 }
    pub fn l(&self) -> bool { (self.0 & 0x0000_0040) != 0 }
    pub fn b(&self) -> bool { (self.0 & 0x0000_0080) != 0 }
    pub fn s(&self) -> bool { (self.0 & 0x0000_0100) != 0 }
    pub fn r(&self) -> bool { (self.0 & 0x0000_0200) != 0 }
    pub fn f(&self) -> bool { (self.0 & 0x0000_0400) != 0 }
    pub fn z(&self) -> bool { (self.0 & 0x0000_0800) != 0 }
    pub fn i(&self) -> bool { (self.0 & 0x0000_1000) != 0 }
    pub fn v(&self) -> bool { (self.0 & 0x0000_2000) != 0 }
    pub fn rr(&self) -> bool {(self.0 & 0x0000_4000) != 0 }
    pub fn l4(&self) -> bool {(self.0 & 0x0000_8000) != 0 }

    pub fn set_m(&mut self, val: bool) { self.set_bit(0, val);}
    pub fn set_a(&mut self, val: bool) { self.set_bit(1, val);}
    pub fn set_c(&mut self, val: bool) { self.set_bit(2, val);}
    pub fn set_w(&mut self, val: bool) { self.set_bit(3, val);}
    pub fn set_p(&mut self, val: bool) { self.set_bit(4, val);}
    pub fn set_d(&mut self, val: bool) { self.set_bit(5, val);}
    pub fn set_l(&mut self, val: bool) { self.set_bit(6, val);}
    pub fn set_b(&mut self, val: bool) { self.set_bit(7, val);}
    pub fn set_s(&mut self, val: bool) { self.set_bit(8, val);}
    pub fn set_r(&mut self, val: bool) { self.set_bit(9, val);}
    pub fn set_f(&mut self, val: bool) { self.set_bit(10, val);}
    pub fn set_z(&mut self, val: bool) { self.set_bit(11, val);}
    pub fn set_i(&mut self, val: bool) { self.set_bit(12, val);}
    pub fn set_v(&mut self, val: bool) { self.set_bit(13, val);}
    pub fn set_rr(&mut self, val: bool) {self.set_bit(14, val);}
    pub fn set_l4(&mut self, val: bool) {self.set_bit(15, val);}

}


