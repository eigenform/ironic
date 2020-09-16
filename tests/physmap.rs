use std::sync::{Arc,Mutex};
use ironic::mem::back::BigEndianMemory;
use ironic::bus::*;

// A mock MMIO device.
#[derive(Default)]
pub struct MyMmioDevice {
    register_a: u32,
    register_b: u32,
}
impl MyMmioDevice {
    fn read32(&self, off: usize) -> u32 {
        match off {
            0 => self.register_a,
            4 => self.register_b,
            _ => panic!("No register at offset {:x}", off),
        }
    }
    fn write32(&mut self, off: usize, val: u32) {
        match off {
            0 => self.register_a = val,
            4 => self.register_b = val,
            _ => panic!("No register at offset {:x}", off),
        }
    }
}

// A unique identifier for each memory device.
#[derive(Debug, Clone, Copy)]
pub enum DevId { Foo, Bar, Mmio, Baz }

// A handle used to dispatch a memory access. 
#[derive(Debug, Clone, Copy)]
pub struct DevHandle { id: DevId, base: u32 }


// Container for the state of the physical memory map.
pub struct MyMap { 
    foo: Box<BigEndianMemory>,
    bar: Box<BigEndianMemory>,
    baz: Box<BigEndianMemory>,
    mmio: Box<MyMmioDevice>,
    baz_enabled: bool,
}
impl PhysMemMap for MyMap {}

// Implement the physical address decoder.
impl PhysMemDecode for MyMap {
    type Addr = u32;
    type Handle = DevHandle;

    // Convert a physical address into a handle.
    fn decode_phys_addr(&self, addr: &Self::Addr) -> Option<Self::Handle> {
        match addr { 
            0x1000_0000..=0x1fff_ffff => 
                if self.baz_enabled { 
                    Some(DevHandle { id: DevId::Baz, base: 0x1000_0000 })
                } else { 
                    Some(DevHandle { id: DevId::Foo, base: 0x1000_0000 })
                },
            0x2000_0000..=0x3fff_ffff => 
                Some(DevHandle { id: DevId::Bar, base: 0x2000_0000 }),
            0xc000_0000..=0xc000_ffff => 
                Some(DevHandle { id: DevId::Mmio, base: 0xc000_0000 }),
            _ => return None,
        }
    }

    fn _read32(&mut self, hdl: DevHandle, addr: u32) -> u32 {
        let off = (addr - hdl.base) as usize;
        match hdl.id {
            DevId::Mmio => self.mmio.read32(off),
            DevId::Foo => self.foo.read::<u32>(off),
            DevId::Bar => self.bar.read::<u32>(off),
            DevId::Baz => self.baz.read::<u32>(off),
        }
    }
    fn _write32(&mut self, hdl: DevHandle, addr: u32, val: u32) {
        let off = (addr - hdl.base) as usize;
        match hdl.id {
            DevId::Mmio => self.mmio.write32(off, val),
            DevId::Foo => self.foo.write::<u32>(off, val),
            DevId::Bar => self.bar.write::<u32>(off, val),
            DevId::Baz => self.baz.write::<u32>(off, val),
        }
    }
    fn _read16(&mut self, hdl: DevHandle, addr: u32) -> u16 {
        let off = (addr - hdl.base) as usize;
        match hdl.id {
            DevId::Foo => self.foo.read::<u16>(off),
            DevId::Bar => self.bar.read::<u16>(off),
            DevId::Baz => self.baz.read::<u16>(off),
            _ => panic!(),
        }
    }
    fn _write16(&mut self, hdl: DevHandle, addr: u32, val: u16) {
        let off = (addr - hdl.base) as usize;
        match hdl.id {
            DevId::Foo => self.foo.write::<u16>(off, val),
            DevId::Bar => self.bar.write::<u16>(off, val),
            DevId::Baz => self.baz.write::<u16>(off, val),
            _ => panic!(),
        }
    }
    fn _read8(&mut self, hdl: DevHandle, addr: u32) -> u8 {
        let off = (addr - hdl.base) as usize;
        match hdl.id {
            DevId::Foo => self.foo.read::<u8>(off),
            DevId::Bar => self.bar.read::<u8>(off),
            DevId::Baz => self.baz.read::<u8>(off),
            _ => panic!(),
        }
    }
    fn _write8(&mut self, hdl: DevHandle, addr: u32, val: u8) {
        let off = (addr - hdl.base) as usize;
        match hdl.id {
            DevId::Foo => self.foo.write::<u8>(off, val),
            DevId::Bar => self.bar.write::<u8>(off, val),
            DevId::Baz => self.baz.write::<u8>(off, val),
            _ => panic!(),
        }
    }
}


#[test]
fn make_physmap() {
    let mut physmap = MyMap { 
        foo: Box::new(BigEndianMemory::new(0x1000, None)),
        bar: Box::new(BigEndianMemory::new(0x1000, None)),
        baz: Box::new(BigEndianMemory::new(0x1000, None)),
        mmio: Box::new(MyMmioDevice::default()),
        baz_enabled: false,
    };

    physmap.write32(0xc000_0004, 0xdeadcafe);
    assert_eq!(0xdeadcafe, physmap.read32(0xc000_0004));
}

