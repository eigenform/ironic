//! Abstraction for managing emulated memories which are embedded directly 
//! into the virtual address space of the running process.
//!
//! ## Limitations
//! Very low mappings are typically restricted by the Linux kernel for
//! security reasons. You can alter the behaviour with the `vm.mmap_min_addr`
//! sysctl knob (also, see `/proc/sys/vm/mmap_min_addr`).

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


/// Container for keeping track of a particular mapping from the virtual
/// address space of the current process to some backing memory.
///
/// Type `A` indicates the width of the address space.

#[derive(Debug)]
pub struct HostMemRegion {
    /// Pointer to this mapping in the host's virtual address space.
    ptr: Option<*mut [u8]>,
    /// The length of this mapping.
    len: usize,
}

/// Private, unsafe wrappers around mmap() and munmap() from libc.
impl HostMemRegion {
    unsafe fn munmap(ptr: *mut [u8], len: usize) {
        let res = munmap(ptr as *mut c_void, len);
        if res == -1 {
            panic!("munmap({:?}) returned -1", ptr);
        }
    }

    unsafe fn mmap(fd: i32, vaddr: usize, len: usize, off: usize) -> *mut [u8] {
        let addr = vaddr as *mut c_void;
        let res = mmap(addr, len, 
            PROT_READ | PROT_WRITE, 
            MAP_FIXED | MAP_SHARED, 
            fd, off.try_into().unwrap()
        );
        if res == MAP_FAILED {
            panic!("mmap() returned {:?}", addr);
        }
        std::slice::from_raw_parts_mut(
            res as *mut u8, std::mem::size_of::<u8>() * len
        )
    }
}

/// Public interface to a [HostMemRegion].
impl HostMemRegion {
    /// Initialize a new (unmapped/disabled) region.
    pub fn new(len: usize) -> Self {
        HostMemRegion { ptr: None, len }
    }

    /// Enable this region, mapping it into our virtual address space.
    pub fn enable(&mut self, shm_fd: i32, off: usize, vaddr: usize) {
        if self.ptr.is_none() {
            self.ptr = unsafe {
                Some(HostMemRegion::mmap(shm_fd, vaddr, self.len, off))
            };
        } else {
            panic!("Couldn't enable region");
        }
    }

    /// Disable this region, unmapping it from our virtual address space.
    pub fn disable(&mut self) {
        if self.ptr.is_some() {
            unsafe { HostMemRegion::munmap(self.ptr.unwrap(), self.len) };
            self.ptr = None;
        }
    }
}


/// Container used to keep track of an emulated memory region which will be
/// mapped into the virtual address space of the process.
///
/// - Type `T` indicates the type of key used to access particular regions.
/// - Type `A` indicates the width of the address space.

#[derive(Debug)]
pub struct HostMemBacking<T: Eq + std::hash::Hash> {
    /// A human-readable name for this memory backing.
    pub name: CString,
    /// Some set of mappings associated with this backing.
    pub regions: HashMap<T, HostMemRegion>,
    /// The length of this memory region.
    pub len: usize,
    /// File descriptor for the shared memory backing this memory region.
    fd: i32,
}

/// Private, unsafe wrapper for creating a shared memory object.
impl <T: Eq + std::hash::Hash> HostMemBacking<T> {
    unsafe fn create_shm(name: *const c_char, len: usize) -> i32 {
        let fd = shm_open(name, O_RDWR | O_CREAT | O_EXCL, 0o600);
        if fd == 1 {
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

    pub fn del_region(&mut self, key: T) { self.regions.remove(&key); }

    pub fn add_region(&mut self, key: T, off: usize, addr: usize, len: usize) {
        let mut region = HostMemRegion::new(len);
        region.enable(self.fd, off, addr);
        self.regions.insert(key, region);
    }

    pub fn enable_region(&mut self, key: T, off: usize, host_addr: usize) {
        self.regions.get_mut(&key).unwrap()
            .enable(self.fd, off, host_addr);
    }
    pub fn disable_region(&mut self, key: T) {
        self.regions.get_mut(&key).unwrap()
            .disable();
    }
}

impl <T:Eq + std::hash::Hash> Drop for HostMemBacking<T> {
    fn drop(&mut self) {
        for (key, region) in self.regions.iter_mut() {
            region.disable();
        }
        self.regions.clear();
    }
}

