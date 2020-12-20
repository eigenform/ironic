
use crate::bus::*;
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

impl Bus {
    /// Dispatch a physical read access to some memory-mapped I/O device.
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

    /// Dispatch a physical write access to some memory-mapped I/O device.
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

    //pub fn schedule_task(&mut self, t: BusTask) {
    //    use BusTask::*;
    //    let latency = match t {
    //        Nand(_) | Aes(_) | Sha(_) => 4,
    //        Mi {..} => 0,
    //        SetRomDisabled(_) | SetMirrorEnabled(_) => 0,
    //    };
    //    self.task_queue.push(Task { kind: t, ctr: latency});
    //}


    /// Emulate a slice of work on the system bus.
    ///
    /// This drains all pending tasks that have been scheduled on any of the
    /// I/O devices.
    pub fn step(&mut self) {
        self.handle_step_hlwd();
        if !self.tasks.is_empty() {
            self.drain_tasks();
        }
    }

    /// Returns the state of the IRQ input signal attached to the CPU.
    pub fn irq_line(&mut self) -> bool {
        let dev = self.dev.read().unwrap();
        dev.hlwd.irq.irq_output
    }

    /// Dispatch all of the pending tasks on the Bus.
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

