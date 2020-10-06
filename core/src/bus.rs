//! ## Overview
//! Implementing a bus is "interesting" because we can have some "fun" 
//! reasoning about how to design something performant and semantically 
//! somewhat like the thing we're trying to model.
//!
//! For us, a bus has the following requirements:
//!
//! 1. The layout of the physical memory map might change at runtime depending 
//!    on the state of some registers, so when a bus resolves a physical 
//!    address, it must use a reference to read that state.
//! 2. A [Cpu] must use a reference to perform memory accesses.
//! 3. I/O devices must use a reference to perform DMA accesses.
//!
//! In hardware, memory accesses are signals passed between asynchronous
//! systems. This begs some reflection on what it means to "pass a message" in 
//! software:
//!
//! 1. Function calls can be regarded as a way of synchronously messaging
//!    (in the sense that everything in a single thread of execution is 
//!    effectively synchronous).
//!
//! 2. Access to shared memory is an inherently asynchronous way of messaging
//!    between multiple threads of execution. In order to articulate necessary
//!    dependencies between events, we rely on some operating system's API.
//! 
//! ## Current Implementation: Scheduling work for devices
//! For now, we're single stepping an emulated CPU. This means that after each
//! CPU cycle we have the opportunity to service any work pending on the 
//! emulated bus.
//!
//! All of the references used to link system devices are currently wrapped in 
//! [Arc] and [RwLock]. This is not strictly necessary right now, but I figure
//! that locking will make it easier later if/when we decide to do some work
//! in a thread seperate from the CPU emulation.
//!
//! ## Current Implementation: Dispatching memory accesses
//! Right now there's a lot of code re-use when dispatching reads and writes.
//! At some point it would be nice to make it more generic, but it seems
//! difficult because we can't support accesses of all widths across all of 
//! the target devices.
//!

/// Abstractions for implementing a physical memory map.
pub mod prim;
/// Implements a decoder for physical addresses.
pub mod decode;
/// Implements read/write access dispatching to devices.
pub mod dispatch;
/// Interfaces for dealing with memory-mapped I/O devices.
pub mod mmio;
/// Functionality for scheduling/completing tasks on behalf of devices.
pub mod task;


use std::sync::{Arc,RwLock};

use crate::topo;
use crate::dbg;

use crate::bus::mmio::*;
use crate::bus::task::*;

pub type DbgRef = Arc<RwLock<dbg::Debugger>>;
pub type MemRef = Arc<RwLock<topo::SystemMemory>>;
pub type DevRef = Arc<RwLock<topo::SystemDevice>>;

/// Implementation of an emulated bus.
pub struct Bus {
    /// Reference to attached [Debugger].
    pub dbg: DbgRef,
    /// Reference to [SystemMemory].
    pub mem: MemRef,
    /// Reference to [SystemDevice].
    pub dev: DevRef,

    /// Queue for pending work on I/O devices.
    pub tasks: Vec<BusTask>,
}
impl Bus {
    pub fn new(dbg: DbgRef, mem: MemRef, dev: DevRef)-> Self {
        Bus { 
            dbg, mem, dev,
            tasks: Vec::new(),
        }
    }
}

