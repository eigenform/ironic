//! ## Notes
//! I've re-written this code more times than I'd like to admit, in an attempt
//! to make it less ugly. I guess this is fine.

use crate::bus::*;
use crate::bus::prim::*;

/// Top-level read/write functions.
impl Bus {
    pub fn read32(&mut self, addr: u32) -> u32 {
        let msg = self.do_read(addr, BusWidth::W);
        match msg { BusPacket::Word(res) => res, _ => unreachable!(), }
    }
    pub fn read16(&mut self, addr: u32) -> u16 {
        let msg = self.do_read(addr, BusWidth::H);
        match msg { BusPacket::Half(res) => res, _ => unreachable!(), }
    }
    pub fn read8(&mut self, addr: u32) -> u8 {
        let msg = self.do_read(addr, BusWidth::B);
        match msg { BusPacket::Byte(res) => res, _ => unreachable!(), }
    }

    pub fn write32(&mut self, addr: u32, val: u32) {
        self.do_write(addr, BusPacket::Word(val));
    }
    pub fn write16(&mut self, addr: u32, val: u16) {
        self.do_write(addr, BusPacket::Half(val));
    }
    pub fn write8(&mut self, addr: u32, val: u8) {
        self.do_write(addr, BusPacket::Byte(val));
    }

    pub fn dma_write(&mut self, addr: u32, buf: &[u8]) {
        self.do_dma_write(addr, buf);
    }
    pub fn dma_read(&mut self, addr: u32, buf: &mut [u8]) {
        self.do_dma_read(addr, buf);
    }

}

/// Decode a physical address. At this point, we know we either need to deal
/// with a plain memory device, or some memory-mapped I/O device.
impl Bus {
    /// Dispatch a read access.
    fn do_read(&mut self, addr: u32, width: BusWidth) -> BusPacket {
        let handle = self.decode_phys_addr(addr).unwrap_or_else(||
            panic!("Unresolved physical address {:08x}", addr)
        );

        let off = addr.wrapping_sub(handle.base) as usize;
        let resp = match handle.dev {
            Device::Mem(dev) => self.do_mem_read(dev, off, width),
            Device::Io(dev) => self.do_mmio_read(dev, off, width),
        };
        resp
    }

    /// Dispatch a write access.
    fn do_write(&mut self, addr: u32, msg: BusPacket) {
        let handle = self.decode_phys_addr(addr).unwrap_or_else(||
            panic!("Unresolved physical address {:08x}", addr)
        );

        let off = addr.wrapping_sub(handle.base) as usize;
        let _resp = match handle.dev {
            Device::Mem(dev) => self.do_mem_write(dev, off, msg),
            Device::Io(dev) => self.do_mmio_write(dev, off, msg),
        };
    }
}

/// Dispatch a read or write of some width to a memory device.
impl Bus {
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

/// Perform some DMA request.
impl Bus {
    fn do_dma_write(&mut self, addr: u32, buf: &[u8]) {
        use MemDevice::*;
        let handle = self.decode_phys_addr(addr).unwrap_or_else(||
            panic!("Unresolved physical address {:08x}", addr)
        );

        let off = addr.wrapping_sub(handle.base) as usize;
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

    fn do_dma_read(&mut self, addr: u32, buf: &mut [u8]) {
        use MemDevice::*;
        let handle = self.decode_phys_addr(addr).unwrap_or_else(||
            panic!("Unresolved physical address {:08x}", addr)
        );

        let off = addr.wrapping_sub(handle.base) as usize;
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


