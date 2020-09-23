#![feature(test)]
extern crate test;
use std::sync::{Arc, Mutex};
use test::Bencher;

use ironic::bus::*;
use ironic::mem::back::BigEndianMemory;

#[derive(Debug, Clone, Copy)]
pub enum DevId {
    BeMemBox,
    BeMemLock,
}

#[derive(Debug, Clone, Copy)]
pub struct DevHandle {
    id: DevId,
    base: u32,
}

pub struct MyMap {
    foo_box: Box<BigEndianMemory>,
    foo_lock: Arc<Mutex<BigEndianMemory>>,
}
impl PhysMemMap for MyMap {}

impl PhysMemDecode for MyMap {
    type Addr = u32;
    type Handle = DevHandle;

    fn decode_phys_addr(&self, addr: &Self::Addr) -> Option<Self::Handle> {
        match addr {
            0x1000_0000..=0x1001_0000 => Some(DevHandle {
                id: DevId::BeMemBox,
                base: 0x1000_0000,
            }),
            0x2000_0000..=0x2001_0000 => Some(DevHandle {
                id: DevId::BeMemLock,
                base: 0x2000_0000,
            }),
            _ => return None,
        }
    }

    fn _read32(&mut self, hdl: DevHandle, addr: u32) -> u32 {
        let off = (addr - hdl.base) as usize;
        match hdl.id {
            DevId::BeMemBox => self.foo_box.read::<u32>(off),
            DevId::BeMemLock => self.foo_lock.lock().unwrap().read::<u32>(off),
        }
    }
    fn _write32(&mut self, hdl: DevHandle, addr: u32, val: u32) {
        let off = (addr - hdl.base) as usize;
        match hdl.id {
            DevId::BeMemBox => self.foo_box.write::<u32>(off, val),
            DevId::BeMemLock => self.foo_lock.lock().unwrap().write::<u32>(off, val),
        }
    }

    fn _read16(&mut self, _hdl: DevHandle, _addr: u32) -> u16 {
        panic!();
    }
    fn _write16(&mut self, _hdl: DevHandle, _addr: u32, _val: u16) {
        panic!();
    }
    fn _read8(&mut self, _hdl: DevHandle, _addr: u32) -> u8 {
        panic!();
    }
    fn _write8(&mut self, _hdl: DevHandle, _addr: u32, _val: u8) {
        panic!();
    }
}

macro_rules! make_physmap {
    () => {
        MyMap {
            foo_box: Box::new(BigEndianMemory::new(0x10_000, None)),
            foo_lock: Arc::new(Mutex::new(BigEndianMemory::new(0x10_000, None))),
        }
    };
}

#[bench]
fn bemem_boxed(b: &mut Bencher) {
    let mut physmap = make_physmap!();
    b.iter(|| {
        for i in 0..0x100 {
            let off = 0x1000_0000 + (i * 4);
            physmap.write32(off, 0xdeadcafe);
            assert_eq!(0xdeadcafe, physmap.read32(off));
        }
    })
}

#[bench]
fn bemem_locked(b: &mut Bencher) {
    let mut physmap = make_physmap!();
    b.iter(|| {
        for i in 0..0x100 {
            let off = 0x2000_0000 + (i * 4);
            physmap.write32(off, 0xdeadcafe);
            assert_eq!(0xdeadcafe, physmap.read32(off));
        }
    })
}
