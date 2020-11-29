
use std::collections::VecDeque;
use crate::bus::mmio::MmioDevice;

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


pub struct Task {
    pub kind: BusTask,
    pub ctr: usize,
}

/// A queue of tasks to-be-completed by the bus.
pub struct TaskQueue { pub q: VecDeque<Task> }
impl TaskQueue {
    pub fn new() -> Self { 
        TaskQueue { q: VecDeque::new() } 
    }
}

