//! Abstraction for managing/addressing emulated memories, which are embedded 
//! directly into the virtual address space of the running process.
//!
//! ## Limitations
//! Very low mappings are typically restricted by the Linux kernel for
//! security reasons (although you can alter the behaviour with the 
//! `vm.mmap_min_addr` sysctl knob; also, see `/proc/sys/vm/mmap_min_addr`).
//!

use std::ffi::CString;
use std::convert::TryInto;
use std::collections::HashMap;

extern crate libc;
use libc::{
    shm_open, shm_unlink, mmap, munmap, ftruncate,
    O_CREAT, O_RDWR, O_EXCL,
    MAP_SHARED, MAP_FIXED, MAP_FAILED,
    PROT_READ, PROT_WRITE,
    c_void, c_char,
};

/// Prefix bits set in a resulting virtual address.
///
/// This is a rudimentary way of carving a block in virtual memory which is
/// dedicated to emulating memory accesses.
/// 
/// _Note that the top of Linx user-land is_ `0x0000_7fff_ffff_ffff`.

const HOST_PREF: usize = 0x0000_1337_0000_0000;

/// Mask bits defining the width of the emulated physical address space.
///
/// This is a rudimentary way of sanitizing the target of memory accesses
/// that we emit which are intended to read or write a host memory region.
const HOST_MASK: usize = 0x0000_0000_ffff_ffff;


/// Container newtype, for converting from some guest \[physical\] address to 
/// the host's virtual address space.
#[repr(transparent)]
#[derive(Debug)]
pub struct HostAddr(pub u32);
impl HostAddr {
    #[inline]
    fn conv(self) -> usize { (self.0 as usize & HOST_MASK) | HOST_PREF }
    /// Return a mutable pointer to this address.
    #[inline]
    pub fn to_mut<T>(self) -> *mut T { HostAddr::conv(self) as *mut T }
    /// Return a const pointer to this address.
    #[inline]
    pub fn to_ptr<T>(self) -> *const T { HostAddr::conv(self) as *const T }
}

/// Perform a write on a host memory region.
#[inline]
pub unsafe fn host_write<T>(dst: HostAddr, src: T) {
    std::ptr::write::<T>(dst.to_mut::<T>(), src); 
}
/// Perform a read on a host memory region.
#[inline]
pub unsafe fn host_read<T>(dst: HostAddr) -> T {
    std::ptr::read::<T>(dst.to_ptr::<T>())
}


/// Container for keeping track of a particular mapping into the virtual
/// address space of this process, associated with some [HostMemBacking]. 
///
/// ## Implementation notes
/// A [HostMemRegion] is always associated with a [HostMemBacking], and is not
/// meant to be moved between them (for now). We only intend that a user
/// should be able to disable or enable a region, and not change the address,
/// length, or offset after they've been added to some backing (unless they
/// are intentionally destroyed).
///
/// Enabling/disabling a region directly corresponds some `mmap()` or `munmap()`
/// call with the arguments specified in the structure. Note that the file
/// descriptor associated with a [HostMemBacking] is freed up after all of the
/// mappings are destroyed.

#[derive(Debug)]
struct HostMemRegion {
    /// The intended emulated address corresponding to this mapping.
    addr: u32,
    /// The length of this mapping.
    len: usize,
    /// The offset of this mapping within the associated file descriptor.
    off: usize,

    /// A pointer to this mapping in the host's virtual address space.
    ///
    /// When this field is `Some`, the region is enabled/active. 
    /// Otherwise, when this field is `None`, the region is disabled/inactive.
    ptr: Option<*mut [u8]>,
}

/// Private, unsafe wrappers around mmap() and munmap() from libc.
impl HostMemRegion {
    unsafe fn munmap(ptr: *mut [u8], len: usize) {
        let res = munmap(ptr as *mut c_void, len);
        if res == -1 {
            panic!("munmap({:?}) returned -1", ptr);
        }
    }

    unsafe fn mmap(fd: i32, addr: *mut c_void, len: usize, off: usize) -> *mut [u8] {
        let res = mmap(addr, len, 
            PROT_READ | PROT_WRITE, 
            MAP_FIXED | MAP_SHARED, 
            fd, off.try_into().unwrap()
        );
        if res == MAP_FAILED {
            panic!("mmap() returned MAP_FAILED for {:?}", addr);
        }
        std::slice::from_raw_parts_mut(
            res as *mut u8, std::mem::size_of::<u8>() * len
        )
    }
}

/// Public interface to a [HostMemRegion].
impl HostMemRegion {
    /// Initialize a new (unmapped/disabled) region.
    pub fn new(addr: u32, len: usize, off: usize) -> Self {
        HostMemRegion { ptr: None, addr, off, len }
    }

    /// Enable this region, mapping it into our virtual address space.
    pub fn map(&mut self, shm_fd: i32) {
        if self.ptr.is_none() {
            let host_ptr = HostAddr(self.addr).to_mut::<c_void>();

            self.ptr = unsafe {
                Some(HostMemRegion::mmap(shm_fd, host_ptr, self.len, self.off))
            };
        } else {
            panic!("Couldn't map region");
        }
    }

    /// Disable this region, unmapping it from our virtual address space.
    pub fn unmap(&mut self) {
        if self.ptr.is_some() {
            unsafe { HostMemRegion::munmap(self.ptr.unwrap(), self.len) };
            self.ptr = None;
        }
    }
}


/// Container used to keep track of an emulated memory region which will be
/// mapped into the virtual address space of the process.
///
/// Type `T` indicates the type of key used to access particular regions.
///
/// Right now, the dimensions of memory regions added by a user are not 
/// intended to be changed - only enabled or disabled.
///
/// ## Safety
/// Right now this entire mechanism is **highly unsafe** - if you're using 
/// this, you are _necessarily_ using raw pointers to issue reads/writes to 
/// memory without respect for any of Rust's safety guarantees.
///
/// ## Example usage
/// Consider an emulated CPU that performs memory accesses on certain 
/// addresses corresponding to simple memories. Here, [HostMemBacking] 
/// carves out regions in our own virtual address space, which we can use
/// to do accesses natively.
///
/// ```
/// use std::hash::Hash;
/// use ironic::mem::host::{
///     HostMemBacking, HostAddr,
///     host_read, host_write
/// };
///
/// #[derive(Eq, PartialEq, Hash)]
/// enum MemId {
///     Foo, FooMirror
/// }
///
/// // Create a new backing object and add some regions.
/// let mut mem = HostMemBacking::<MemId>::new("my-memory", 0x0010_000);
/// mem.add_region(MemId::Foo,       0xd000_0000, 0x0001_0000, 0);
/// mem.add_region(MemId::FooMirror, 0xd008_0000, 0x0001_0000, 0);
///
/// // Enable/map the regions.
/// mem.map(MemId::Foo);
/// mem.map(MemId::FooMirror);
///
/// // Do something - i.e. have an emulated CPU use raw pointers 
/// // to perform memory accesses natively.
/// unsafe { 
///     host_write::<u32>(HostAddr(0xd000_8000), 0xdeadbeef);
///     assert_eq!(0xdeadbeef, host_read::<u32>(HostAddr(0xd008_8000)));
/// }
///
/// // Disable/unmap a region.
/// mem.unmap(MemId::FooMirror);
/// ```
///
/// ## Implementation notes
/// A [HostMemBacking] is a shared memory object of some fixed size, created
/// by calling `shm_open()`. The set of associated regions are mappings from 
/// "some offset in the shared memory object" to "some region in the virtual 
/// address space of the running process."
///

#[derive(Debug)]
pub struct HostMemBacking<T: Eq + std::hash::Hash> {
    /// A unique name representing the shared memory object.
    pub name: CString,
    /// The length of this memory region in bytes.
    pub len: usize,
    /// Some set of mappings associated with this backing.
    regions: HashMap<T, HostMemRegion>,
    /// File descriptor for the shared memory object.
    fd: i32,
}

/// Private, unsafe wrapper for creating a shared memory object.
impl <T: Eq + std::hash::Hash> HostMemBacking<T> {
    unsafe fn create_shm(name: *const c_char, len: usize) -> i32 {
        let fd = shm_open(name, O_RDWR | O_CREAT | O_EXCL, 0o600);
        if fd < 0 {
            panic!("shm_open for object {:?} failed", name);
        } else {
            shm_unlink(name);
        }
        if ftruncate(fd, len.try_into().unwrap()) < 0 {
            panic!("ftruncate() for {:?} ({:x?} bytes) failed", name, len);
        } else {
            fd
        }
    }
}

/// Public interface to a [HostMemBacking].
impl <T: Eq + std::hash::Hash> HostMemBacking<T> {
    pub fn new(backing_name: &str, len: usize) -> Self {
        let name = CString::new(
            format!("{}-{}", backing_name, std::process::id())
        ).unwrap();

        let fd = unsafe {
            HostMemBacking::<T>::create_shm(name.as_ptr(), len)
        };
        HostMemBacking { name, len, fd,
            regions: HashMap::<T, HostMemRegion>::new(),
        }
    }

    /// Disable a region and remove it from this backing.
    pub fn del_region(&mut self, key: T) { 
        self.regions.remove(&key); 
    }

    /// Add a memory region to this backing.
    pub fn add_region(&mut self, key: T, addr: u32, len: usize, off: usize) {
        let region = HostMemRegion::new(addr, len, off);
        self.regions.insert(key, region);
    }

    pub fn map(&mut self, key: T) {
        self.regions.get_mut(&key).unwrap().map(self.fd);
    }
    pub fn unmap(&mut self, key: T) {
        self.regions.get_mut(&key).unwrap().unmap();
    }
}

impl <T:Eq + std::hash::Hash> Drop for HostMemBacking<T> {
    fn drop(&mut self) {
        for (_key, region) in self.regions.iter_mut() {
            region.unmap();
        }
        self.regions.clear();
    }
}

