
/// Some type of indirect access (from memory interface to the DDR interface).
#[derive(Debug)]
pub enum IndirAccess { Read, Write }

/// Representing some device and piece of work to-be-completed by the bus.
#[derive(Debug)]
pub enum BusTask {
    /// A NAND interface command.
    Nand(u32),
    /// An AES interface command.
    Aes(u32),
    /// A SHA interface command.
    Sha(u32),

    /// Change the state of the boot ROM mapping
    SetRomDisabled(bool),
    /// Change the state of the SRAM mappings
    SetMirrorEnabled(bool),

    /// A read/write access request on the DDR interface.
    Mi { kind: IndirAccess, data: u16 },
}

/// An entry kept by the [Bus], representing some task to-be-completed.
pub struct Task {
    pub kind: BusTask,
    pub target_cycle: usize,
}

