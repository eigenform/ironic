//! Abstractions for dealing with the guest's physical address space.

use std::mem;
use core::slice;
use std::convert::TryInto;

/// Helper functions implemented on numeric primitives.
pub unsafe trait AccessWidth: Sized {
    fn from_be_bytes(data: &[u8]) -> Self;
    fn from_le_bytes(data: &[u8]) -> Self;
    fn as_be(self) -> Self;
    fn as_le(self) -> Self;

    #[inline]
    fn as_ptr(&self) -> *const Self { self as *const Self }

    #[inline]
    fn as_mut(&mut self) -> *mut Self { self as *mut Self }

    #[inline]
    unsafe fn as_bytes(&self) -> &[u8] {
        slice::from_raw_parts(self.as_ptr() as *const u8, 
            mem::size_of_val(self))
    }

    #[inline]
    unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.as_mut() as *mut u8, 
            mem::size_of_val(self))
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
            fn as_be(self) -> Self { Self::to_be(self) }
            #[inline]
            fn as_le(self) -> Self { Self::to_le(self) }
        }
    }
}

impl_accesswidth!(u32);
impl_accesswidth!(u16);
impl_accesswidth!(u8);


/// Implemented on memory devices which are addressible in 32-bit words.
pub trait WordSupport {
    fn read32(&self, off: usize) -> u32;
    fn write32(&mut self, off: usize, val: u32);
}

/// Implemented on memory devices which are addressible in 16-bit half-words.
pub trait HalfWordSupport {
    fn read16(&self, off: usize) -> u16;
    fn write16(&mut self, off: usize, val: u16);
}

/// Implemented on memory devices which are addressible in 8-bit bytes.
pub trait ByteSupport {
    fn read8(&self, off: usize) -> u8;
    fn write8(&mut self, off: usize, val: u8);
}

/// Implemented on memory devices which support bulk (i.e. DMA) accesses.
pub trait DmaSupport {
    /// Read some number of bytes from memory at some offset.
    fn read_buf(&self, off: usize, dst: &mut [u8]);
    /// Write some number of bytes to memory at some offset.
    fn write_buf(&mut self, off: usize, src: &[u8]);
}

/// A marker trait for memory devices.
pub trait MemoryDevice {}

/// Kinds of memory accesses supported by a physical memory map.
#[derive(Debug, Clone, Copy)]
pub enum AccessType { 
    Write32, Read32,
    Write16, Read16,
    Write8, Read8,
    DmaWrite, DmaRead,
}


/// Implemented on some object responsible for resolving physical addresses.
///
/// This is a _trait_ because we expect that the act of resolving the target
/// of a physical memory access depends on some state of the system, which
/// might alter the way that physical addresses are decoded. Additionally, 
/// because all memory devices may not accept reads/writes for all types 
/// tagged with [AccessWidth], a type implementing [PhysMemMap] must also 
/// provide an additional layer of indirection which specifies which accesses
/// are supported for each particular memory device.
///
/// ## Example: A physical memory map with four devices
/// Here's an example of a physical memory map implementation which has four
/// memory devices:
///
/// ```
/// use std::sync::{Arc,Mutex};
/// use std::convert::TryInto;
/// use ironic::mem::back::*;
/// use ironic::bus::*;
///
/// // A mock MMIO device.
/// #[derive(Default)]
/// pub struct MyMmioDevice {
///     register_a: u32,
///     register_b: u32,
/// }
/// impl MemoryDevice for MyMmioDevice {}
/// impl MyMmioDevice {
///     fn read32(&self, off: usize) -> u32 {
///         match off {
///             0 => self.register_a,
///             4 => self.register_b,
///             _ => panic!("No register at offset {:x}", off),
///         }
///     }
///     fn write32(&mut self, off: usize, val: u32) {
///         match off {
///             0 => self.register_a = val,
///             4 => self.register_b = val,
///             _ => panic!("No register at offset {:x}", off),
///         }
///     }
/// }
///
/// // Container for the state of the physical memory map.
/// pub struct MyMap { 
///     foo: Arc<Mutex<BigEndianMemory>>,
///     bar: Arc<Mutex<BigEndianMemory>>,
///     baz: Arc<Mutex<BigEndianMemory>>,
///     mmio: Arc<Mutex<MyMmioDevice>>,
///     baz_enabled: bool,
/// }
///
/// // A unique identifier for each memory device.
/// #[derive(Debug, Clone, Copy)]
/// pub enum DevId { Foo, Bar, Mmio, Baz }
///
/// // A handle used to dispatch a memory access. 
/// #[derive(Debug, Clone, Copy)]
/// pub struct DevHandle { id: DevId, base: u32 }
///
/// impl PhysMemMap for MyMap {
///     type Addr = u32;
///     type Handle = DevHandle;
///
///     // Convert a physical address into a handle.
///     fn decode_phys_addr(&self, addr: &Self::Addr) -> Option<Self::Handle> {
///         match addr { 
///             0x1000_0000..=0x1fff_ffff => 
///                 if self.baz_enabled { 
///                     Some(DevHandle { id: DevId::Baz, base: 0x1000_0000 })
///                 } else { 
///                     Some(DevHandle { id: DevId::Foo, base: 0x1000_0000 })
///                 },
///             0x2000_0000..=0x3fff_ffff => 
///                 Some(DevHandle { id: DevId::Bar, base: 0x2000_0000 }),
///             0xc000_0000..=0xc000_ffff => 
///                 Some(DevHandle { id: DevId::Mmio, base: 0xc000_0000 }),
///             _ => return None,
///         }
///     }
///
///     // Use a handle to dispatch a 32-bit read.
///     fn disp_read32(&mut self, hdl: DevHandle, addr: u32) -> u32 {
///         let off = (addr - hdl.base) as usize;
///         match hdl.id {
///             DevId::Mmio => self.mmio.lock().unwrap().read32(off),
///             DevId::Foo => self.foo.lock().unwrap().read32(off),
///             DevId::Bar => self.bar.lock().unwrap().read32(off),
///             DevId::Baz => self.baz.lock().unwrap().read32(off),
///             _ => panic!(),
///         }
///     }
///
///     // Use a handle to dispatch a 32-bit write.
///     fn disp_write32(&mut self, hdl: DevHandle, addr: u32, val: u32) {
///         let off = (addr - hdl.base) as usize;
///         match hdl.id {
///             DevId::Mmio => self.mmio.lock().unwrap().write32(off, val),
///             DevId::Foo => self.foo.lock().unwrap().write32(off, val),
///             DevId::Bar => self.bar.lock().unwrap().write32(off, val),
///             DevId::Baz => self.baz.lock().unwrap().write32(off, val),
///             _ => panic!(),
///         }
///     }
///
///     // Use a handle to dispatch a 16-bit read.
///     fn disp_read16(&mut self, hdl: DevHandle, addr: u32) -> u16 {
///         let off = (addr - hdl.base) as usize;
///         match hdl.id {
///             DevId::Foo => self.foo.lock().unwrap().read16(off),
///             DevId::Bar => self.bar.lock().unwrap().read16(off),
///             DevId::Baz => self.baz.lock().unwrap().read16(off),
///             _ => panic!(),
///         }
///     }
///
///     // Use a handle to dispatch a 16-bit write.
///     fn disp_write16(&mut self, hdl: DevHandle, addr: u32, val: u16) {
///         let off = (addr - hdl.base) as usize;
///         match hdl.id {
///             DevId::Foo => self.foo.lock().unwrap().write16(off, val),
///             DevId::Bar => self.bar.lock().unwrap().write16(off, val),
///             DevId::Baz => self.baz.lock().unwrap().write16(off, val),
///             _ => panic!(),
///         }
///     }
///
///     // Use a handle to dispatch a 8-bit read.
///     fn disp_read8(&mut self, hdl: DevHandle, addr: u32) -> u8 {
///         let off = (addr - hdl.base) as usize;
///         match hdl.id {
///             DevId::Foo => self.foo.lock().unwrap().read8(off),
///             DevId::Bar => self.bar.lock().unwrap().read8(off),
///             DevId::Baz => self.baz.lock().unwrap().read8(off),
///             _ => panic!(),
///         }
///     }
///
///     // Use a handle to dispatch a 8-bit write.
///     fn disp_write8(&mut self, hdl: DevHandle, addr: u32, val: u8) {
///         let off = (addr - hdl.base) as usize;
///         match hdl.id {
///             DevId::Foo => self.foo.lock().unwrap().write8(off, val),
///             DevId::Bar => self.bar.lock().unwrap().write8(off, val),
///             DevId::Baz => self.baz.lock().unwrap().write8(off, val),
///             _ => panic!(),
///         }
///     }
/// }
///
/// let mut physmap = MyMap { 
///     foo: Arc::new(Mutex::new(BigEndianMemory::new(0x1000, None))),
///     bar: Arc::new(Mutex::new(BigEndianMemory::new(0x1000, None))),
///     baz: Arc::new(Mutex::new(BigEndianMemory::new(0x1000, None))),
///     mmio: Arc::new(Mutex::new(MyMmioDevice::default())),
///     baz_enabled: false,
/// };
///
/// physmap.write32(0xc000_0004, 0xdeadcafe);
/// assert_eq!(0xdeadcafe, physmap.read32(0xc000_0004));
///
///
/// ```

pub trait PhysMemMap { 
    /// A type representing a physical address on the guest machine.
    type Addr;
    /// A type representing a reference to a memory device.
    type Handle;

    /// Decode a physical address into a handle, used to dispatch an access
    /// to the appropriate memory device.
    fn decode_phys_addr(&self, addr: &Self::Addr) -> Option<Self::Handle>;

    /// Dispatches a 32-bit read to some memory device.
    fn disp_read32(&mut self, hdl: Self::Handle, addr: Self::Addr) -> u32;
    /// Dispatches a 16-bit read to some memory device.
    fn disp_read16(&mut self, hdl: Self::Handle, addr: Self::Addr) -> u16;
    /// Dispatches a 8-bit read to some memory device.
    fn disp_read8(&mut self, hdl: Self::Handle, addr: Self::Addr) -> u8;

    /// Dispatches a 32-bit write to some memory device.
    fn disp_write32(&mut self, hdl: Self::Handle, addr: Self::Addr, val: u32);
    /// Dispatches a 16-bit write to some memory device.
    fn disp_write16(&mut self, hdl: Self::Handle, addr: Self::Addr, val: u16);
    /// Dispatches a 8-bit write to some memory device.
    fn disp_write8(&mut self, hdl: Self::Handle, addr: Self::Addr, val: u8);

    fn read32(&mut self, addr: Self::Addr) -> u32 {
        self.disp_read32(self.decode_phys_addr(&addr).unwrap(), addr)
    }
    fn write32(&mut self, addr: Self::Addr, val: u32) {
        self.disp_write32(self.decode_phys_addr(&addr).unwrap(), addr, val)
    }
    fn read16(&mut self, addr: Self::Addr) -> u16 {
        self.disp_read16(self.decode_phys_addr(&addr).unwrap(), addr)
    }
    fn write16(&mut self, addr: Self::Addr, val: u16) {
        self.disp_write16(self.decode_phys_addr(&addr).unwrap(), addr, val)
    }
    fn read8(&mut self, addr: Self::Addr) -> u8 {
        self.disp_read8(self.decode_phys_addr(&addr).unwrap(), addr)
    }
    fn write8(&mut self, addr: Self::Addr, val: u8) {
        self.disp_write8(self.decode_phys_addr(&addr).unwrap(), addr, val)
    }
}


