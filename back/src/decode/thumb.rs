//! Thumb instruction decoder.

use crate::lut::Instruction;

#[derive(Clone, Debug)]
pub enum ThumbInst {
    SbcReg, CmpReg, OrrReg, BicReg, TstReg, EorReg, MvnReg, CmnReg, AdcReg,
    AndReg, MovReg, SubReg, AddReg, CmpRegAlt, AddRegAlt, MovRegAlt,
    MovRegShiftReg,

    RsbImm, AddImm, MovImm, SubImm, CmpImm, AddSpImm, SubSpImm,
    AddSpImmAlt, AddImmAlt, SubImmAlt, 

    StrbReg, LdrhReg, LdrbReg, StrReg, StrhReg, LdrReg, LdrsbReg, LdrshReg,

    StrhImm, StrImm, StrbImm, StrImmAlt, LdrhImm, LdrbImm, LdrImm, LdrImmAlt, 
    LdrLit, Stm, Ldm,

    Pop, Push, Mul,
    B, Bx, BlxReg, Svc, Bkpt, BAlt,

    Undefined,

    // These are exceptional (added by hand) until I decide sort how these
    // are decoded
    BlPrefix, BlImmSuffix, BlxImmSuffix,
}


impl Instruction for ThumbInst {
    type Opcd = u16;
    fn decode(opcd: u16) -> ThumbInst {
        use ThumbInst::*;
        match opcd & 0xffc0 {
            0x4240 => return RsbImm,
            0x4180 => return SbcReg,
            0x4280 => return CmpReg,
            0x4300 => return OrrReg,
            0x4380 => return BicReg,
            0x4200 => return TstReg,
            0x4040 => return EorReg,
            0x43c0 => return MvnReg,
            0x42c0 => return CmnReg,
            0x4140 => return AdcReg,
            0x4340 => return Mul,
            0x4000 => return AndReg,
            _ => {},
        }
        match opcd & 0xff80 {
            0xb000 => return AddSpImmAlt,
            0xb080 => return SubSpImm,
            0x4700 => return Bx,
            0x4780 => return BlxReg,
            _ => {},
        }
        match opcd & 0xff00 {
            0xdf00 => return Svc,
            0x4500 => return CmpRegAlt,
            0x4400 => return AddRegAlt,
            0x4600 => return MovReg,
            0xbe00 => return Bkpt,
            _ => {},
        }
        match opcd & 0xfe00 {
            0x1c00 => return AddImm,
            0x5800 => return LdrReg,
            0x5600 => return LdrsbReg,
            0x4000 => return MovRegShiftReg,
            0x5e00 => return LdrshReg,
            0x1e00 => return SubImm,
            0xbc00 => return Pop,
            0x1800 => return AddReg,
            0x5400 => return StrbReg,
            0x1a00 => return SubReg,
            0xb400 => return Push,
            0x5a00 => return LdrhReg,
            0x5c00 => return LdrbReg,
            0x5000 => return StrReg,
            0x5200 => return StrhReg,
            _ => {},
        }
        match opcd & 0xf800 {
            // Exceptional (added by hand)
            0xf000 => return BlPrefix,
            0xf800 => return BlImmSuffix,
            0xe800 => return BlxImmSuffix,

            0xe000 => return BAlt,
            0x2000 => return MovImm,
            0x3000 => return AddImmAlt,
            0xa800 => return AddSpImm,
            0x8000 => return StrhImm,
            0xc000 => return Stm,
            0x3800 => return SubImmAlt,
            0x2800 => return CmpImm,
            0x6000 => return StrImm,
            0x9000 => return StrImmAlt,
            0x7000 => return StrbImm,
            0x8800 => return LdrhImm,
            0x7800 => return LdrbImm,
            0xc800 => return Ldm,
            0x6800 => return LdrImm,
            0x9800 => return LdrImmAlt,
            0x4800 => return LdrLit,
            _ => {},
        }
        match opcd & 0xf000 {
            0xd000 => return B,
            _ => {},
        }
        match opcd & 0xe000 {
            0x0000 => return MovRegAlt,
            _ => {},
        }
        Undefined
    }
}

