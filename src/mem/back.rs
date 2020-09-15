use std::mem;
use std::fs::File;
use std::io::Read;

use crate::bus::{
    AccessWidth, MemoryDevice,
    WordSupport, HalfWordSupport, ByteSupport, 
    DmaSupport
};

/// An abstract memory device; backing allocated on the heap.
pub struct BigEndianMemory { 
    /// Vector of bytes with the contents of this memory device.
    data: Vec<u8> 
}
impl BigEndianMemory {
    pub fn new(len: usize, init_fn: Option<&str>) -> Self { 
        let data = if init_fn.is_some() {
            let filename = init_fn.unwrap();
            let mut f = File::open(filename)
                .expect("Couldn't open file to initialize BigEndianMemory.");

            let mut data = vec![0u8; len];
            f.read(&mut data).unwrap();
            data
        } else {
            vec![0u8; len]
        };
        BigEndianMemory { data } 
    }
}

impl WordSupport for BigEndianMemory {
    fn read32(&self, off: usize) -> u32 {
        let src_len = mem::size_of::<u32>();
        if off + src_len > self.data.len() {
            panic!("Out-of-bounds, offset {:x}", off);
        }
        AccessWidth::from_be_bytes(&self.data[off..off + src_len])
    }
    fn write32(&mut self, off: usize, val: u32) {
        let data = val.to_be();
        let src_slice: &[u8] = unsafe { data.as_bytes() };
        if off + src_slice.len() > self.data.len() {
            panic!("Out-of-bounds, offset {:x}", off);
        }
        self.data[off..off+src_slice.len()].copy_from_slice(src_slice);
    }
}

impl HalfWordSupport for BigEndianMemory {
    fn read16(&self, off: usize) -> u16 {
        let src_len = mem::size_of::<u16>();
        if off + src_len > self.data.len() {
            panic!("Out-of-bounds, offset {:x}", off);
        }
        AccessWidth::from_be_bytes(&self.data[off..off + src_len])
    }
    fn write16(&mut self, off: usize, val: u16) {
        let data = val.to_be();
        let src_slice: &[u8] = unsafe { data.as_bytes() };
        if off + src_slice.len() > self.data.len() {
            panic!("Out-of-bounds, offset {:x}", off);
        }
        self.data[off..off+src_slice.len()].copy_from_slice(src_slice);
    }
}

impl ByteSupport for BigEndianMemory {
    fn read8(&self, off: usize) -> u8 {
        let src_len = mem::size_of::<u8>();
        if off + src_len > self.data.len() {
            panic!("Out-of-bounds, offset {:x}", off);
        }
        AccessWidth::from_be_bytes(&self.data[off..off + src_len])
    }
    fn write8(&mut self, off: usize, val: u8) {
        let data = val.to_be();
        let src_slice: &[u8] = unsafe { data.as_bytes() };
        if off + src_slice.len() > self.data.len() {
            panic!("Out-of-bounds, offset {:x}", off);
        }
        self.data[off..off+src_slice.len()].copy_from_slice(src_slice);
    }
}

impl DmaSupport for BigEndianMemory {
    fn read_buf(&self, off: usize, dst: &mut [u8]) {
        if off + dst.len() > self.data.len() { 
            panic!("Out-of-bounds DMA read on BigEndianMemory, offset {:x}", off);
        }
        dst.copy_from_slice(&self.data[off..off + dst.len()]);
    }
    fn write_buf(&mut self, off: usize, src: &[u8]) {
        if off + src.len() > self.data.len() { 
            panic!("Out-of-bounds DMA write on BigEndianMemory, offset {:x}", off);
        }
        self.data[off..off + src.len()].copy_from_slice(src);
    }
}

impl MemoryDevice for BigEndianMemory {}

