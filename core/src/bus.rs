//! Abstractions for dealing with the guest's physical address space.

use core::slice;
use std::convert::TryInto;
use std::mem;

/// Helper functions implemented on numeric primitives.
pub unsafe trait AccessWidth: Sized {
    fn from_be_bytes(data: &[u8]) -> Self;
    fn from_le_bytes(data: &[u8]) -> Self;
    fn as_be(self) -> Self;
    fn as_le(self) -> Self;

    #[inline]
    fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    #[inline]
    fn as_mut(&mut self) -> *mut Self {
        self as *mut Self
    }

    #[inline]
    unsafe fn as_bytes(&self) -> &[u8] {
        slice::from_raw_parts(self.as_ptr() as *const u8, mem::size_of_val(self))
    }

    #[inline]
    unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.as_mut() as *mut u8, mem::size_of_val(self))
    }
}

/// Macro to make implementing AccessWidth a bit less verbose.
macro_rules! impl_accesswidth {
    ($type:ident) => {
        unsafe impl AccessWidth for $type {
            #[inline]
            fn from_be_bytes(data: &[u8]) -> Self {
                Self::from_be_bytes(data.try_into().unwrap())
            }
            #[inline]
            fn from_le_bytes(data: &[u8]) -> Self {
                Self::from_le_bytes(data.try_into().unwrap())
            }
            #[inline]
            fn as_be(self) -> Self {
                Self::to_be(self)
            }
            #[inline]
            fn as_le(self) -> Self {
                Self::to_le(self)
            }
        }
    };
}

impl_accesswidth!(u32);
impl_accesswidth!(u16);
impl_accesswidth!(u8);

/// Interface for decoding physical addresses into some abstract handle/token
/// for a particular memory device.
///
/// These are used by the interface exposed by [PhysMemMap].
pub trait PhysMemDecode {
    /// A type representing a physical address on the guest machine.
    type Addr: Copy;
    /// A type representing a reference to a memory device.
    type Handle;

    /// Decode a physical address into a handle, used to dispatch an access
    /// to the appropriate memory device. Called by [PhysMemMap].
    fn decode_phys_addr(&self, addr: Self::Addr) -> Option<Self::Handle>;

    /// Dispatches a 32-bit read to some memory device.
    fn _read32(&mut self, hdl: Self::Handle, addr: Self::Addr) -> u32;
    /// Dispatches a 16-bit read to some memory device.
    fn _read16(&mut self, hdl: Self::Handle, addr: Self::Addr) -> u16;
    /// Dispatches a 8-bit read to some memory device.
    fn _read8(&mut self, hdl: Self::Handle, addr: Self::Addr) -> u8;

    /// Dispatches a 32-bit write to some memory device.
    fn _write32(&mut self, hdl: Self::Handle, addr: Self::Addr, val: u32);
    /// Dispatches a 16-bit write to some memory device.
    fn _write16(&mut self, hdl: Self::Handle, addr: Self::Addr, val: u16);
    /// Dispatches a 8-bit write to some memory device.
    fn _write8(&mut self, hdl: Self::Handle, addr: Self::Addr, val: u8);
}

/// Top-level trait, providing read/write functions to physical memory.
///
/// This trait marks some type responsible for resolving physical addresses.
/// A type implementing [PhysMemMap] must necessarily implement [PhysMemDecode].

pub trait PhysMemMap: PhysMemDecode {
    fn read32(&mut self, addr: Self::Addr) -> u32 {
        self._read32(self.decode_phys_addr(addr).unwrap(), addr)
    }
    fn write32(&mut self, addr: Self::Addr, val: u32) {
        self._write32(self.decode_phys_addr(addr).unwrap(), addr, val)
    }
    fn read16(&mut self, addr: Self::Addr) -> u16 {
        self._read16(self.decode_phys_addr(addr).unwrap(), addr)
    }
    fn write16(&mut self, addr: Self::Addr, val: u16) {
        self._write16(self.decode_phys_addr(addr).unwrap(), addr, val)
    }
    fn read8(&mut self, addr: Self::Addr) -> u8 {
        self._read8(self.decode_phys_addr(addr).unwrap(), addr)
    }
    fn write8(&mut self, addr: Self::Addr, val: u8) {
        self._write8(self.decode_phys_addr(addr).unwrap(), addr, val)
    }
}
