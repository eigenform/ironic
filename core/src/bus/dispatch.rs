//! ## Notes
//! I've re-written this code more times than I'd like to admit, in an attempt
//! to make it less ugly. I guess this is fine.

use crate::bus::*;
use crate::bus::prim::*;

/// Top-level read/write functions for performing physical memory accesses.
impl Bus {
    /// Perform a 32-bit physical memory read.
    pub fn read32(&mut self, addr: u32) -> u32 {
        let msg = self.do_read(addr, BusWidth::W);
        match msg { BusPacket::Word(res) => res, _ => unreachable!(), }
    }

    /// Perform a 16-bit physical memory read.
    pub fn read16(&mut self, addr: u32) -> u16 {
        let msg = self.do_read(addr, BusWidth::H);
        match msg { BusPacket::Half(res) => res, _ => unreachable!(), }
    }

    /// Perform an 8-bit physical memory read.
    pub fn read8(&mut self, addr: u32) -> u8 {
        let msg = self.do_read(addr, BusWidth::B);
        match msg { BusPacket::Byte(res) => res, _ => unreachable!(), }
    }

    /// Perform a 32-bit physical memory write.
    pub fn write32(&mut self, addr: u32, val: u32) {
        self.do_write(addr, BusPacket::Word(val));
    }
    /// Perform a 16-bit physical memory write.
    pub fn write16(&mut self, addr: u32, val: u16) {
        self.do_write(addr, BusPacket::Half(val));
    }
    /// Perform an 8-bit physical memory write.
    pub fn write8(&mut self, addr: u32, val: u8) {
        self.do_write(addr, BusPacket::Byte(val));
    }

    /// Perform a DMA write operation.
    pub fn dma_write(&mut self, addr: u32, buf: &[u8]) {
        self.do_dma_write(addr, buf);
    }
    /// Perform a DMA read operation.
    pub fn dma_read(&mut self, addr: u32, buf: &mut [u8]) {
        self.do_dma_read(addr, buf);
    }

}

impl Bus {
    /// Dispatch a physical read access (to memory, or some I/O device).
    fn do_read(&mut self, addr: u32, width: BusWidth) -> BusPacket {
        let handle = self.decode_phys_addr(addr).unwrap_or_else(||
            panic!("Unresolved physical address {:08x}", addr)
        );

        let off = (addr & handle.mask) as usize;
        let resp = match handle.dev {
            Device::Mem(dev) => self.do_mem_read(dev, off, width),
            Device::Io(dev) => self.do_mmio_read(dev, off, width),
        };
        resp
    }

    /// Dispatch a physical write access (to memory, or some I/O device).
    fn do_write(&mut self, addr: u32, msg: BusPacket) {
        let handle = self.decode_phys_addr(addr).unwrap_or_else(||
            panic!("Unresolved physical address {:08x}", addr)
        );

        let off = (addr & handle.mask) as usize;
        let _resp = match handle.dev {
            Device::Mem(dev) => self.do_mem_write(dev, off, msg),
            Device::Io(dev) => self.do_mmio_write(dev, off, msg),
        };
    }
}

impl Bus {
    /// Dispatch a physical read access to some memory device.
    fn do_mem_read(&mut self, dev: MemDevice, off: usize, width: BusWidth) -> BusPacket {
        use MemDevice::*;
        use BusPacket::*;
        let mem = self.mem.write().unwrap();
        let target_ref = match dev {
            MaskRom => &mem.mrom,
            Sram0   => &mem.sram0,
            Sram1   => &mem.sram1,
            Mem1    => &mem.mem1,
            Mem2    => &mem.mem2,
        };
        match width {
            BusWidth::W => Word(target_ref.read::<u32>(off)),
            BusWidth::H => Half(target_ref.read::<u16>(off)),
            BusWidth::B => Byte(target_ref.read::<u8>(off)),
        }
    }

    /// Dispatch a physical write access to some memory device.
    fn do_mem_write(&mut self, dev: MemDevice, off: usize, msg: BusPacket) {
        use MemDevice::*;
        use BusPacket::*;
        let mut mem = self.mem.write().unwrap();
        let target_ref = match dev {
            MaskRom => panic!("Writes on mask ROM are unsupported"),
            Sram0   => &mut mem.sram0,
            Sram1   => &mut mem.sram1,
            Mem1    => &mut mem.mem1,
            Mem2    => &mut mem.mem2,
        };
        match msg {
            Word(val) => target_ref.write::<u32>(off, val),
            Half(val) => target_ref.write::<u16>(off, val),
            Byte(val) => target_ref.write::<u8>(off, val),
        }
    }
}

impl Bus {
    /// Dispatch a DMA write to some memory device.
    fn do_dma_write(&mut self, addr: u32, buf: &[u8]) {
        use MemDevice::*;
        let handle = self.decode_phys_addr(addr).unwrap_or_else(||
            panic!("Unresolved physical address {:08x}", addr)
        );

        let off = (addr & handle.mask) as usize;
        let mut mem = self.mem.write().unwrap();
        match handle.dev {
            Device::Mem(dev) => { match dev {
                MaskRom => panic!("Bus error: DMA write on mask ROM"),
                Sram0 => mem.sram0.write_buf(off, buf),
                Sram1 => mem.sram1.write_buf(off, buf),
                Mem1 => mem.mem1.write_buf(off, buf),
                Mem2 => mem.mem2.write_buf(off, buf),
            }},
            _ => panic!("Bus error: DMA write on memory-mapped I/O region"),
        }
    }

    /// Dispatch a DMA read to some memory device.
    fn do_dma_read(&mut self, addr: u32, buf: &mut [u8]) {
        use MemDevice::*;
        let handle = self.decode_phys_addr(addr).unwrap_or_else(||
            panic!("Unresolved physical address {:08x}", addr)
        );

        let off = (addr & handle.mask) as usize;
        let mem = self.mem.write().unwrap();
        match handle.dev {
            Device::Mem(dev) => { match dev {
                MaskRom => panic!("Bus error: DMA read on mask ROM"),
                Sram0 => mem.sram0.read_buf(off, buf),
                Sram1 => mem.sram1.read_buf(off, buf),
                Mem1 => mem.mem1.read_buf(off, buf),
                Mem2 => mem.mem2.read_buf(off, buf),
            }},
            _ => panic!("Bus error: DMA read on memory-mapped I/O region"),
        }
    }
}


