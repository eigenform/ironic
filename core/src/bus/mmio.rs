
use crate::bus::*;
use crate::dbg::*;
use crate::bus::prim::*;
use crate::bus::task::*;

/// Interface used by the bus to perform some access on an I/O device.
pub trait MmioDevice {
    /// Width of accesses supported on this device.
    type Width;

    /// Handle a read, returning some result.
    fn read(&mut self, off: usize) -> BusPacket;
    /// Handle a write, optionally returning a task for the bus.
    fn write(&mut self, off: usize, val: Self::Width) -> Option<BusTask>;
}

/// Dispatch a read or write to some memory-mapped I/O device.
impl Bus {
    pub fn do_mmio_read(&mut self, dev: IoDevice, off: usize, width: BusWidth) -> BusPacket {
        use IoDevice::*;
        let mut dref = self.dev.write().unwrap();
        match (width, dev) {
            (BusWidth::W, Nand) => dref.nand.read(off),
            (BusWidth::W, Aes)  => dref.aes.read(off),
            (BusWidth::W, Sha)  => dref.sha.read(off),
            (BusWidth::W, Hlwd) => dref.hlwd.read(off),
            (BusWidth::W, Di) => dref.hlwd.di.read(off),
            _ => panic!("Unsupported read {:?} for {:?} at {:x}", width, dev, off),
        }
    }

    pub fn do_mmio_write(&mut self, dev: IoDevice, off: usize, msg: BusPacket) {
        use IoDevice::*;
        use BusPacket::*;
        let mut dref = self.dev.write().unwrap();
        let task = match (msg, dev) {
            (Word(val), Nand) => dref.nand.write(off, val),
            (Word(val), Aes)  => dref.aes.write(off, val),
            (Word(val), Sha)  => dref.sha.write(off, val),
            (Word(val), Hlwd) => dref.hlwd.write(off, val),
            (Word(val), Ahb)  => dref.hlwd.ahb.write(off, val),
            _ => panic!("Unsupported write {:?} for {:?} at {:x}", msg, dev, off),
        };
        if task.is_some() {
            self.tasks.push(task.unwrap());
        }
    }
}


impl Bus {
    pub fn step(&mut self) {
        if !self.tasks.is_empty() {
            log(&self.dbg, LogLevel::Bus, &format!(
                    "Completing {} tasks", self.tasks.len()));

            let mut tasks = std::mem::replace(&mut self.tasks, Vec::new());
            for task in tasks.drain(..) {
                match task {
                    BusTask::Nand(val) => self.handle_task_nand(val),
                    BusTask::Aes(val) => self.handle_task_aes(val),
                    BusTask::Sha(val) => self.handle_task_sha(val),
                }
            }
        }
    }
}

