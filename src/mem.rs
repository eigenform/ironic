//! Containers for emulated memories.

pub mod host;

use std::mem;
use core::slice;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;

/// A trait marking supported memory access widths.
///
/// In order to have a nice interface for generically implementing read/write
/// accesses on some kind of emulated memory, we expect this trait to be 
/// implemented on all numeric primitives whose widths are supported.

pub unsafe trait AccessWidth {

    /// Convert from big-endian bytes into some type.
    fn from_be_bytes(data: &[u8]) -> Self where Self: Sized;

    /// Convert from little-endian bytes into some type.
    fn from_le_bytes(data: &[u8]) -> Self where Self: Sized;

    /// Convert to big-endian.
    fn as_be(self) -> Self where Self: Sized;

    /// Convert to little-endian.
    fn as_le(self) -> Self where Self: Sized;

    /// Return a reference to this value as a slice.
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] where Self: Sized {
        unsafe {
            let len = mem::size_of_val(self);
            slice::from_raw_parts(self as *const Self as *const u8, len)
        }
    }

    /// Return a reference to this value as a mutable slice.
    #[inline(always)]
    fn as_bytes_mut(&mut self) -> &mut [u8] where Self: Sized {
        unsafe {
            let len = mem::size_of_val(self);
            slice::from_raw_parts_mut(self as *mut Self as *mut u8, len)
        }
    }
}

/// Macro to make implementing AccessWidth a bit less verbose.
#[macro_export]
macro_rules! impl_accesswidth_for_type { 
    ($type:ident) => {
        unsafe impl AccessWidth for $type {
            #[inline(always)]
            fn from_be_bytes(data: &[u8]) -> Self where Self: Sized {
                Self::from_be_bytes(data.try_into().unwrap())
            }
            #[inline(always)]
            fn from_le_bytes(data: &[u8]) -> Self where Self: Sized {
                Self::from_le_bytes(data.try_into().unwrap())
            }

            #[inline(always)]
            fn as_be(self) -> Self where Self: Sized { Self::to_be(self) }

            #[inline(always)]
            fn as_le(self) -> Self where Self: Sized { Self::to_le(self) }
        }
    }
}


/// Implemented on memory devices that support typical read/write accesses.
pub trait BusMemory {
    /// Perform a read access of width T at some offset in memory.
    fn read<T: AccessWidth + Copy>(&self, off: usize) -> T;
    /// Perform a write access of width T at some offset in memory.
    fn write<T: AccessWidth + Copy>(&mut self, off: usize, data: T);
}

/// Implemented on memory devices that support bulk (i.e. "DMA") accesses.
pub trait DmaMemory {
    /// Read some number of bytes from memory at some offset.
    fn read_buf(&self, off: usize, dst: &mut [u8]);
    /// Write some number of bytes to memory at some offset.
    fn write_buf(&mut self, off: usize, src: &[u8]);
}



/// An abstract memory device, with backing allocated on the heap.
pub struct BigEndianMemory { 
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

impl BusMemory for BigEndianMemory {
    fn read<T: AccessWidth>(&self, off: usize) -> T {
        let src_len = mem::size_of::<T>();
        if off + src_len > self.data.len() { 
            panic!("Out-of-bounds read on BigEndianMemory, offset {:x}", off);
        }
        T::from_be_bytes(&self.data[off..off + src_len])
    }
    fn write<T: AccessWidth>(&mut self, off: usize, data: T) {
        let val = data.as_be();
        let src_slice: &[u8] = val.as_bytes();
        if off + src_slice.len() > self.data.len() { 
            panic!("Out-of-bounds write on BigEndianMemory, offset {:x}", off); 
        }
        self.data[off..off+src_slice.len()].copy_from_slice(src_slice);
    }
}

impl DmaMemory for BigEndianMemory {
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

impl_accesswidth_for_type!(u64);
impl_accesswidth_for_type!(u32);
impl_accesswidth_for_type!(u16);
impl_accesswidth_for_type!(u8);
