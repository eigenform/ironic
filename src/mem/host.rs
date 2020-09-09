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
pub struct HostMemRegion<A> {
    /// Pointer to this mapping in the host's virtual address space.
    ptr: Option<*mut [u8]>,
    /// The emulated physical address corresponding to this mapping.
    addr: A,
    /// The length of this mapping.
    len: usize,
}

/// Container used to keep track of an emulated memory region which will be
/// mapped into the virtual address space of the process.
///
/// - Type `T` indicates the type of key used to access particular regions.
/// - Type `A` indicates the width of the address space.

#[derive(Debug)]
pub struct HostMemBacking<A, T: Eq + std::hash::Hash> {
    /// A human-readable name for this memory backing.
    pub name: CString,
    /// Some set of mappings associated with this backing.
    pub regions: HashMap<T, HostMemRegion<A>>,
    /// The length of this memory region.
    pub len: usize,
    /// File descriptor for the shared memory backing this memory region.
    fd: i32,
}

/// Private, unsafe wrappers around mmap() and munmap() from libc.
impl <A> HostMemRegion<A> {
    unsafe fn munmap(ptr: *mut [u8], len: usize) {
        let res = munmap(ptr as *mut c_void, len);
        if res == -1 {
            panic!("munmap({:?}) returned -1", ptr);
        }
    }

    unsafe fn mmap(shm_fd: i32, vaddr: usize, len: usize) -> *mut [u8] {
        let addr = vaddr as *mut c_void;
        let res = mmap(addr, len,
            PROT_READ | PROT_WRITE, MAP_FIXED | MAP_SHARED, shm_fd, 0
        );
        if res == MAP_FAILED {
            panic!("mmap() returned {:?}", addr);
        }
        std::slice::from_raw_parts_mut(
            res as *mut u8, std::mem::size_of::<u8>() * len
        )
    }
}

/// Private, unsafe wrapper for creating a shared memory object.
impl <A, T: Eq + std::hash::Hash> HostMemBacking<A, T> {
    unsafe fn create_shm(name: *const c_char, len: usize) -> i32 {
        let fd = shm_open(name, O_RDWR | O_CREAT | O_EXCL, 0o600);
        if fd == 1 {
            panic!("shm_open for object {:?} failed", name);
        } else {
            shm_unlink(name);
        }
        if ftruncate(fd, len.try_into().unwrap()) < 0 {
            panic!("ftruncate() for object {:?} ({:x?} bytes) failed", name, len);
        } else {
            fd
        }
    }
}

/// Public interface to a [HostMemRegion].
impl <A> HostMemRegion<A> {
    /// Initialize a new (unmapped/disabled) region.
    pub fn new(addr: A, len: usize) -> Self {
        HostMemRegion { ptr: None, addr, len }
    }

    /// Enable this region, mapping it into our virtual address space.
    pub fn enable(&mut self, shm_fd: i32, vaddr: usize) {
        if self.ptr.is_none() {
            self.ptr = unsafe {
                Some(HostMemRegion::<A>::mmap(shm_fd, vaddr, self.len))
            };
        } else {
            panic!("Couldn't enable region");
        }
    }

    /// Disable this region, unmapping it from our virtual address space.
    pub fn disable(&mut self) {
        if self.ptr.is_some() {
            unsafe { HostMemRegion::<A>::munmap(self.ptr.unwrap(), self.len) };
            self.ptr = None;
        } else {
            panic!("Couldn't disable region");
        }
    }
}

/// Public interface to a [HostMemBacking].
impl <A, T: Eq + std::hash::Hash> HostMemBacking<A, T> {
    pub fn new(backing_name: &str, len: usize) -> Self {
        let name = CString::new(format!("{}-{}", backing_name, std::process::id()))
            .unwrap();

        let fd = unsafe {
            HostMemBacking::<A, T>::create_shm(name.as_ptr(), len)
        };

        HostMemBacking { name, len, fd,
            regions: HashMap::<T, HostMemRegion<A>>::new(),
        }
    }

    pub fn del_region(&mut self, key: &T) { self.regions.remove(key); }
    pub fn add_region(&mut self, key: T, addr: A, len: usize) {
        self.regions.insert(key, HostMemRegion::new(addr, len));
    }

    pub fn enable_region(&mut self, key: T, host_addr: usize) {
        self.regions.get_mut(&key).unwrap()
            .enable(self.fd, host_addr);
    }
    pub fn disable_region(&mut self, key: T) {
        self.regions.get_mut(&key).unwrap()
            .disable();
    }

}


