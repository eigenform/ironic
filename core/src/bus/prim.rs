
use core::slice;
use std::convert::TryInto;
use std::mem;

/// Helper functions implemented on numeric primitives.
///
/// These let us easily convert between numeric primitives and byte slices.
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

/// Trait representing the abstract parts of a physical memory map.
///
/// These types are required by [PhysMemDecode] and [PhysMemDispatch].
pub trait PhysMemMap {
    /// A type representing a physical address on the guest machine.
    type Addr: Copy;
    /// A type representing a reference to a memory device.
    type Handle;
    /// A type representing a request for some memory access.
    type Req;
    /// A type representing a response to some memory access.
    type Resp;
}

/// Interface for decoding physical addresses into some abstract handle/token
/// for a particular memory device.
pub trait PhysMemDecode: PhysMemMap {
    /// Decode a physical address into a handle, used to dispatch an access
    /// to the appropriate memory device. Called by [PhysMemMap].
    fn decode_phys_addr(&self, addr: Self::Addr) -> Option<Self::Handle>;
}

/// Trait exposing read/write functions to physical memory.
pub trait PhysMemDispatch: PhysMemMap + PhysMemDecode {

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


    fn phys_read(&mut self, req: Self::Req) -> Option<Self::Resp>;
    fn phys_write(&mut self, req: Self::Req) -> Option<Self::Resp>;

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



/// Handle to a target for some physical memory access.
#[derive(Debug, Clone, Copy)]
pub struct DeviceHandle {
    pub dev: Device,
    //pub base: u32,
    pub mask: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum Device {
    Mem(MemDevice),
    Io(IoDevice),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemDevice {
    MaskRom, 
    Sram0, 
    Sram1, 
    Mem1, 
    Mem2,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IoDevice {
    Nand, 
    Aes, 
    Sha,

    Hlwd,
    Ahb,
    Di,
    Si,
    Exi,
    Mi,
    Ddr,
}


/// A message on the bus containing some value.
#[derive(Debug, Clone, Copy)]
pub enum BusPacket { Byte(u8), Half(u16), Word(u32) }

/// The width of an access on the bus.
#[derive(Debug, Clone, Copy)]
pub enum BusWidth { B, H, W }


#[derive(Debug)]
/// An abstract request on the bus.
pub struct BusReq {
    pub handle: DeviceHandle,
    pub msg: Option<BusPacket>,
}

#[derive(Debug)]
/// An abstract reply to a bus request.
pub struct BusRep {
    pub msg: Option<BusPacket>,
}


