//! Helpers for implementing ALU operations.


pub fn sub_generic(rn: u32, val: u32) -> (u32, bool, bool, bool, bool) {
    let res = rn.wrapping_sub(val);
    let n = (res & 0x8000_0000) != 0;
    let z = res == 0;
    let c = rn.checked_sub(val).is_some();
    let v = (rn as i32).checked_sub(val as i32).is_none();
    (res, n, z, c, v)
}
pub fn add_generic(rn: u32, val: u32) -> (u32, bool, bool, bool, bool) {
    let res = rn.wrapping_add(val);
    let n = (res & 0x8000_0000) != 0;
    let z = res == 0;
    let c = rn.checked_add(val).is_none();
    let v = (rn as i32).checked_add(val as i32).is_none();
    (res, n, z, c, v)
}


/// Barrel shifter opcodes.
#[derive(Debug)]
pub enum ShiftType { Lsl = 0b00, Lsr = 0b01, Asr = 0b10, Ror = 0b11 }
impl From<u32> for ShiftType {
    fn from(x: u32) -> Self {
        match x {
            0b00 => ShiftType::Lsl, 0b01 => ShiftType::Lsr,
            0b10 => ShiftType::Asr, 0b11 => ShiftType::Ror,
            _ => unreachable!(),
        }
    }
}

/// Encoding-specific inputs to a barrel shifter operation.
pub enum ShiftArgs {
    /// Immediate shift (OpImm)
    Imm { imm12: u32, c_in: bool },

    /// Register shift by immediate (OpReg)
    Reg { rm: u32, stype: u32, imm5: u32, c_in: bool },

    /// Register shift by register (OpRegShiftReg)
    RegShiftReg { rm: u32, stype: u32, rs: u32, c_in: bool },
}


/// Logical shift left; works the same for reg/rsr arguments.
pub fn lsl(rm: u32, simm: u32, c_in: bool) -> (u32, bool) {
    if simm == 0 { 
        (rm, c_in) 
    } else if simm < 32 {
        let res = rm << simm;
        let c_out = (1 << (32 - simm) & res) != 0;
        (res, c_out)
    } else if simm == 32 {
        (0, (rm & 1) != 0)
    } else {
        (0, false)
    }
}

/// Logical shift right by immediate.
pub fn lsr_imm(rm: u32, simm: u32, _c_in: bool) -> (u32, bool) {
    if simm == 0 {
        (0, (rm & 0x8000_0000) != 0)
    } else {
        let res = rm >> simm;
        let c_out = (1 << (simm - 1) & res) != 0;
        (res, c_out)
    }
}
/// Logical shift right by register value.
pub fn lsr_reg(rm: u32, simm: u32, c_in: bool) -> (u32, bool) {
    if simm == 0 {
        (rm, c_in)
    } else if simm < 32 {
        let res = rm >> simm;
        let c_out = (1 << (simm - 1) & res) != 0;
        (res, c_out)
    } else if simm == 32 {
        (0, (rm & 0x8000_0000) != 0)
    } else {
        (0, false)
    }
}

/// Arithmetic shift right by immediate.
pub fn asr_imm(rm: u32, simm: u32, _c_in: bool) -> (u32, bool) {
    if simm == 0 {
        if (rm & 0x8000_0000) == 0 {
            (0, false)
        } else {
            (0xffff_ffff, true)
        }
    } else {
        let res = ((rm as i32) >> simm) as u32;
        let c_out = (1 << (simm - 1) & res) != 0;
        (res, c_out)
    }
}
/// Arithmetic shift right by register value.
pub fn asr_reg(rm: u32, simm: u32, c_in: bool) -> (u32, bool) {
    if simm == 0 {
        (rm, c_in)
    } else if simm < 32 {
        let res = ((rm as i32) >> simm) as u32;
        let c_out = (1 << (simm - 1) & res) != 0;
        (res, c_out)
    } else {
        if (rm & 0x8000_0000) == 0 {
            (0, false)
        } else {
            (0xffff_ffff, true)
        }
    }
}

pub fn ror_imm(rm: u32, simm: u32, c_in: bool) -> (u32, bool) {
    if simm == 0 {
        let res = (c_in as u32) << 31 | (rm >> 1);
        (res, (rm & 1) != 0)
    } else {
        let res = rm.rotate_right(simm);
        let c_out = (1 << (simm - 1) & res) != 0;
        (res, c_out)
    }
}
pub fn ror_reg(rm: u32, simm: u32, c_in: bool) -> (u32, bool) {
    if simm == 0 {
        (rm, c_in)
    } else {
        let imm = simm % 32;
        if imm == 0 {
            (rm, (rm & 0x8000_0000) != 0)
        } else {
            let res = rm.rotate_right(imm);
            let c_out = (1 << (imm - 1) & res) != 0;
            (res, c_out)
        }
    }
}





/// Rotate an immediate by a some immediate.
pub fn rot_by_imm(imm12: u32, c_in: bool) -> (u32, bool) {
    let (simm, imm8) = ((imm12 & 0xf00) >> 8, imm12 & 0xff);
    let val = imm8.rotate_right(simm * 2);
    let c_out = if simm == 0 { 
        c_in 
    } else { 
        (val & 0x8000_0000) != 0 
    };
    (val, c_out)
}

/// Shift the value of Rm by some immediate.
pub fn shift_by_imm(rm: u32, sop: u32, simm: u32, c_in: bool) -> (u32, bool) {
    use ShiftType::*;
    match ShiftType::from(sop) {
        Lsl => lsl(rm, simm, c_in),
        Lsr => lsr_imm(rm, simm, c_in),
        Asr => asr_imm(rm, simm, c_in),
        Ror => ror_imm(rm, simm, c_in),
    }
}

/// Shift the value of Rm by the value of Rs.
pub fn shift_by_reg(rm: u32, sop: u32, rs: u32, c_in: bool) -> (u32, bool) {
    use ShiftType::*;
    match ShiftType::from(sop) {
        Lsl => lsl(rm, rs, c_in),
        Lsr => lsr_reg(rm, rs, c_in),
        Asr => asr_reg(rm, rs, c_in),
        Ror => ror_reg(rm, rs, c_in),
    }
}

/// Dispatch some barrel shifter operation.
pub fn barrel_shift(args: ShiftArgs) -> (u32, bool) {
    match args {
        ShiftArgs::Imm { imm12, c_in } => 
            rot_by_imm(imm12, c_in),

        ShiftArgs::Reg { rm, stype, imm5, c_in } =>
            shift_by_imm(rm, stype, imm5, c_in ),

        ShiftArgs::RegShiftReg { rm, stype, rs, c_in } =>
            shift_by_reg(rm, stype, rs, c_in),
    }
}



#[derive(Debug, PartialEq)]
pub enum BitwiseOp { And, Orr, Eor, Bic }


