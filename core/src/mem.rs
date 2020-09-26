//! Containers for emulated memories.

use std::fs::File;
use std::io::Read;
use std::mem;

use crate::bus::AccessWidth;

/// An abstract memory device; backing allocated on the heap.
pub struct BigEndianMemory {
    /// Vector of bytes with the contents of this memory device.
    data: Vec<u8>,
}
impl BigEndianMemory {
    pub fn new(len: usize, init_fn: Option<&str>) -> Self {
        let data = if init_fn.is_some() {
            let filename = init_fn.unwrap();
            let mut f =
                File::open(filename).expect("Couldn't open file to initialize BigEndianMemory.");

            let mut data = vec![0u8; len];
            f.read(&mut data).unwrap();
            data
        } else {
            vec![0u8; len]
        };
        BigEndianMemory { data }
    }
}

impl BigEndianMemory {
    pub fn read<T: AccessWidth>(&self, off: usize) -> T {
        let src_len = mem::size_of::<T>();
        if off + src_len > self.data.len() {
            panic!("Out-of-bounds read at {:x}", off);
        }
        T::from_be_bytes(&self.data[off..off + src_len])
    }
    pub fn write<T: AccessWidth>(&mut self, off: usize, val: T) {
        let data = val.as_be();
        let src_slice: &[u8] = unsafe { data.as_bytes() };
        if off + src_slice.len() > self.data.len() {
            panic!("Out-of-bounds write at {:x}", off);
        }
        self.data[off..off + src_slice.len()].copy_from_slice(src_slice);
    }
}

impl BigEndianMemory {
    pub fn read_buf(&self, off: usize, dst: &mut [u8]) {
        if off + dst.len() > self.data.len() {
            panic!(
                "Out-of-bounds DMA read on BigEndianMemory, offset {:x}",
                off
            );
        }
        dst.copy_from_slice(&self.data[off..off + dst.len()]);
    }
    pub fn write_buf(&mut self, off: usize, src: &[u8]) {
        if off + src.len() > self.data.len() {
            panic!(
                "Out-of-bounds DMA write on BigEndianMemory, offset {:x}",
                off
            );
        }
        self.data[off..off + src.len()].copy_from_slice(src);
    }
}
