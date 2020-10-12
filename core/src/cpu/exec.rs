//! Instruction execution, decoding, and dispatch.

pub mod arm;
pub mod thumb;


/// Result of dispatching an instruction.
#[derive(Debug)]
pub enum DispatchRes {
    /// There was some fatal error dispatching the instruction.
    FatalErr,
    /// This instruction was not executed because the condition failed.
    CondFailed,
    /// This instruction retired and resulted in a branch.
    RetireBranch,
    /// This instruction retired and the PC should be incremented.
    RetireOk,
}

