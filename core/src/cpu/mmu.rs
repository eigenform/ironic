//! Implementation of a memory-management unit.

/// Definitions for types related to the MMU/TLB.
pub mod prim;

use std::sync::{Arc, RwLock};
use crate::cpu::coproc::{ControlRegister, DACRegister, DomainMode};
use crate::cpu::mmu::prim::*;
use crate::cpu::CpuMode;
use crate::bus::*;

/// State associated with the memory-management unit.
///
/// TODO: Don't keep local copies of CPU state, seems hacky.
pub struct Mmu {
    /// Reference to the system bus.
    pub bus: Arc<RwLock<Bus>>,
    /// MMU-local copy of the translation table base register.
    pub ttbr: u32,
    /// MMU-local copy of the domain access control register.
    pub dacr: DACRegister,
    /// MMU-local copy of the p15 r1 system control register.
    pub ctrl: ControlRegister,
    /// The current CPU mode.
    pub cpu_mode: CpuMode,
}
impl Mmu {
    pub fn new(bus: Arc<RwLock<Bus>>) -> Self {
        Mmu { 
            bus, 
            cpu_mode: CpuMode::Svc,
            ttbr: 0, 
            dacr: DACRegister(0),
            ctrl: ControlRegister(0),
        }
    }
}

/// These are the top-level "public" functions (called in the context of the
/// CPU) providing read/write access to memories.
impl Mmu {
    pub fn read32(&self, addr: u32) -> u32 {
        let paddr = self.translate(TLBReq::new(addr, Access::Read));
        self.bus.write().unwrap().read32(paddr)
    }
    pub fn read16(&self, addr: u32) -> u16 {
        let paddr = self.translate(TLBReq::new(addr, Access::Read));
        self.bus.write().unwrap().read16(paddr)
    }
    pub fn read8(&self, addr: u32) -> u8 {
        let paddr = self.translate(TLBReq::new(addr, Access::Read));
        self.bus.write().unwrap().read8(paddr)
    }

    pub fn write32(&self, addr: u32, val: u32) {
        let paddr = self.translate(TLBReq::new(addr, Access::Write));
        self.bus.write().unwrap().write32(paddr, val);
    }
    pub fn write16(&self, addr: u32, val: u32) {
        let paddr = self.translate(TLBReq::new(addr, Access::Write));
        self.bus.write().unwrap().write16(paddr, val as u16);
    }
    pub fn write8(&self, addr: u32, val: u32) {
        let paddr = self.translate(TLBReq::new(addr, Access::Write));
        self.bus.write().unwrap().write8(paddr, val as u8);
    }
}

impl Mmu {
    fn resolve_section(&self, req: TLBReq, d: SectionDescriptor) -> u32 {
        let ctx = self.get_ctx(d.domain());
        if ctx.validate(&req, d.ap()) {
            d.base_addr() | req.vaddr.section_idx() 
        } else {
            panic!("Domain access faults are unimplemented");
        }
    }

    fn resolve_coarse(&self, req: TLBReq, d: CoarseDescriptor) -> u32 {
        let desc = self.l2_fetch(req.vaddr, L1Descriptor::Coarse(d));
        match desc {
            L2Descriptor::SmallPage(entry) => {
                let ctx = self.get_ctx(d.domain());
                if ctx.validate(&req, entry.get_ap(req.vaddr)) {
                    entry.base_addr() | req.vaddr.small_page_idx()
                } else {
                    panic!("Domain access faults are unimplemented");
                }
            },
            _ => panic!("L2 descriptor {:?} unimplemented, vaddr={:08x}", 
                desc, req.vaddr.0),
        }
    }
}


impl Mmu {

    /// Get the context for computing permissions associated with some PTE.
    pub fn get_ctx(&self, dom: u32) -> PermissionContext {
        PermissionContext { 
            domain_mode: self.dacr.domain(dom),
            is_priv: self.cpu_mode.is_privileged(),
            sysprot: self.ctrl.sysprot_enabled(),
            romprot: self.ctrl.romprot_enabled(),
        }
    }

    /// Given some virtual address, return the first-level PTE.
    fn l1_fetch(&self, vaddr: VirtAddr) -> L1Descriptor {
        let addr = (self.ttbr & 0xffff_c000) | vaddr.l1_idx() << 2;
        let val = self.bus.write().unwrap().read32(addr);
        L1Descriptor::from_u32(val)
    }

    /// Given some virtual address and a particular first-level PTE, return
    /// the second-level PTE.
    fn l2_fetch(&self, vaddr: VirtAddr, d: L1Descriptor) -> L2Descriptor {
        let addr = match d {
            L1Descriptor::Coarse(e) => {
                e.base_addr() | vaddr.l2_idx_coarse() << 2
            },
            _ => unreachable!(),
        };
        let val = self.bus.write().unwrap().read32(addr);
        L2Descriptor::from_u32(val)
    }
}

/// Implement virtual-to-physical translation.
///
/// TODO: In reality, I'm pretty sure that the MMU just emits loads in the
/// same way that normal CPU accesses are performed. Considering that we
/// have no cache, this will make accesses very slow. Eventually I'll find 
/// a way to make this faster. 

impl Mmu {
    /// Translate a virtual address into a physical address.
    pub fn translate(&self, req: TLBReq) -> u32 {
        if !self.ctrl.mmu_enabled() { 
            return req.vaddr.0; 
        }

        let desc = self.l1_fetch(req.vaddr);
        let paddr = match desc {
            L1Descriptor::Section(entry) => self.resolve_section(req, entry),
            L1Descriptor::Coarse(entry) => self.resolve_coarse(req, entry),
            _ => panic!("TLB first-level descriptor {:?} unimplemented", desc),
        };
        //println!("translated vaddr={:08x} -> paddr={:08x}", vaddr.0, paddr);
        paddr
    }
}


