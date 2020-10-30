
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
            (BusWidth::W, Ehci) => dref.ehci.read(off),

            (BusWidth::W, Hlwd) => dref.hlwd.read(off),
            (BusWidth::W, Ahb)  => dref.hlwd.ahb.read(off),
            (BusWidth::W, Di)   => dref.hlwd.di.read(off),
            (BusWidth::H, Mi)   => dref.hlwd.mi.read(off),
            (BusWidth::H, Ddr)  => dref.hlwd.ddr.read(off),
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
            (Word(val), Ehci)  => dref.ehci.write(off, val),

            (Word(val), Hlwd) => dref.hlwd.write(off, val),
            (Word(val), Ahb)  => dref.hlwd.ahb.write(off, val),
            (Half(val), Mi)   => dref.hlwd.mi.write(off, val),
            (Half(val), Ddr)  => dref.hlwd.ddr.write(off, val),

            _ => panic!("Unsupported write {:?} for {:?} at {:x}", msg, dev, off),
        };
        if task.is_some() {
            self.tasks.push(task.unwrap());
        }
    }
}


impl Bus {

    /// Emulate a slice of work on the Bus.
    pub fn step(&mut self) {
        self.handle_step_hlwd();

        if !self.tasks.is_empty() {
            self.drain_tasks();
        }
    }

    /// Dispatch all pending tasks on the Bus.
    fn drain_tasks(&mut self) {
        let mut tasks = std::mem::replace(&mut self.tasks, Vec::new());
        for task in tasks.drain(..) {
            match task {
                BusTask::Nand(val) => self.handle_task_nand(val),
                BusTask::Aes(val) => self.handle_task_aes(val),
                BusTask::Sha(val) => self.handle_task_sha(val),
                BusTask::Mi{kind, data} => self.handle_task_mi(kind, data),

                BusTask::SetRomDisabled(x) => {
                    println!("BUS ROM disabled={:?}", x);
                    self.rom_disabled = x;
                },
                BusTask::SetMirrorEnabled(x) => {
                    println!("BUS SRAM mirror enabled={:?}", x);
                    self.mirror_enabled = x;
                }
            }
        }
    }
}

