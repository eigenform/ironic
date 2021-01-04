
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
            (BusWidth::W, Ohci0) => dref.ohci0.read(off),
            (BusWidth::W, Ohci1) => dref.ohci1.read(off),
            (BusWidth::W, Sdhc0) => dref.sd0.read(off),
            (BusWidth::W, Sdhc1) => dref.sd1.read(off),

            (BusWidth::W, Hlwd) => dref.hlwd.read(off),
            (BusWidth::W, Ahb)  => dref.hlwd.ahb.read(off),
            (BusWidth::W, Di)   => dref.hlwd.di.read(off),
            (BusWidth::W, Exi)  => dref.hlwd.exi.read(off),
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
            (Word(val), Ohci0)  => dref.ohci0.write(off, val),
            (Word(val), Ohci1)  => dref.ohci1.write(off, val),
            (Word(val), Sdhc0)  => dref.sd0.write(off, val),
            (Word(val), Sdhc1)  => dref.sd1.write(off, val),


            (Word(val), Hlwd) => dref.hlwd.write(off, val),
            (Word(val), Ahb)  => dref.hlwd.ahb.write(off, val),
            (Word(val), Exi)  => dref.hlwd.exi.write(off, val),
            (Word(val), Di)  => dref.hlwd.di.write(off, val),
            (Half(val), Mi)   => dref.hlwd.mi.write(off, val),
            (Half(val), Ddr)  => dref.hlwd.ddr.write(off, val),

            _ => panic!("Unsupported write {:?} for {:?} at {:x}", msg, dev, off),
        };

        // If the device returned some task, schedule it
        if task.is_some() {
            let t = task.unwrap();
            let c = match t {
                BusTask::Nand(_) => 0,
                BusTask::Aes(_) => 0,
                BusTask::Sha(_) => 0,

                BusTask::Mi{..} => 0,
                BusTask::SetRomDisabled(_) => 0,
                BusTask::SetMirrorEnabled(_) => 0,
            };
            self.tasks.push(Task { kind: t, target_cycle: self.cycle + c });
        }
    }
}


impl Bus {
    /// Emulate a slice of work on the system bus.
    pub fn step(&mut self) {
        self.handle_step_hlwd();
        if !self.tasks.is_empty() {
            self.drain_tasks();
        }
        self.cycle += 1;
    }

    /// Returns the state of the IRQ input signal attached to the CPU.
    pub fn irq_line(&mut self) -> bool {
        let dev = self.dev.read().unwrap();
        dev.hlwd.irq.irq_output
    }

    /// Dispatch all of the pending tasks on the Bus.
    fn drain_tasks(&mut self) {
        let mut idx = 0;
        while idx != self.tasks.len() {
            if self.tasks[idx].target_cycle == self.cycle {
                let task = self.tasks.remove(idx);
                match task.kind {
                    BusTask::Nand(x) => self.handle_task_nand(x),
                    BusTask::Aes(x) => self.handle_task_aes(x),
                    BusTask::Sha(x) => self.handle_task_sha(x),
                    BusTask::Mi{kind, data} => self.handle_task_mi(kind, data),
                    BusTask::SetRomDisabled(x) => self.rom_disabled = x,
                    BusTask::SetMirrorEnabled(x) => self.mirror_enabled = x,
                }
            } else {
                idx += 1;
            }
        }
    }
}

