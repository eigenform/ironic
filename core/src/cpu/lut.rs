//! Traits defining lookup table functionality.

/// Implemented on some enumerated type representing an instruction.
pub trait Instruction {
    /// Type representing a numeric representation of an instruction.
    type Opcd;

    /// Decode an opcode into some representation of an instruction.
    fn decode(opcd: Self::Opcd) -> Self;
}

/// Implemented on some type contained in a lookup table.
pub trait InstLutEntry {
    /// Type representing an instruction.
    type Inst: Instruction;

    /// Convert from an instruction to a lookup table entry.
    fn from_inst(inst: Self::Inst) -> Self;
}

/// Implemented on some container for a lookup table.
pub trait InstLut {
    /// The number of entries in the lookup table.
    const LUT_SIZE: usize;

    /// Type representing an entry in the lookup table.
    type Entry: InstLutEntry;

    /// Type representing an instruction.
    type Instr: Instruction;

    /// Type used to index the lookup table.
    type Index;

    /// Initialize a new lookup table.
    fn create_lut(default_entry: Self::Entry) -> Self;

    /// Convert from an opcode to an index in the lookup table.
    fn opcd_to_idx(opcd: <Self::Instr as Instruction>::Opcd) -> Self::Index;

    /// Convert from an index to an entry (for building the table).
    fn idx_to_opcd(idx: Self::Index) -> <Self::Instr as Instruction>::Opcd;

    /// Lookup an entry in the table during runtime.
    fn lookup(&self, opcd: <Self::Instr as Instruction>::Opcd) -> Self::Entry;
}

