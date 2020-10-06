
use std::collections::VecDeque;
use crate::bus::Bus;
use crate::bus::mmio::MmioDevice;

/// Implemented on the bus for a specific I/O device; used to handle a task.
pub trait TaskHandler<T: MmioDevice> {
    fn handle_task(&mut self, val: u32);
}

/// Representing some device and piece of work to-be-completed by the bus.
#[derive(Debug)]
pub enum BusTask {
    Nand(u32),
    Aes(u32),
    Sha(u32),
}

/// A queue of tasks to-be-completed by the bus.
pub struct TaskQueue { pub q: VecDeque<BusTask> }
impl TaskQueue {
    pub fn new() -> Self { 
        TaskQueue { q: VecDeque::new() } 
    }
}


