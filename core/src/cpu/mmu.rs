//! Implementation of the memory-management unit.

pub mod prim;

use crate::cpu::mmu::prim::*;
use crate::cpu::Cpu;

/// These are the top-level "public" functions providing read/write accesses.
///
/// Right now, in order to perform any memory accesses, we must acquire a
/// mutable reference to the bus. This is expensive.
impl Cpu {
    pub fn read32(&mut self, addr: u32) -> u32 {
        let paddr = self.translate(TLBReq::new(addr, Access::Read));
        let res = self.bus.write().unwrap().read32(paddr);
        res
    }
    pub fn read16(&mut self, addr: u32) -> u16 {
        let paddr = self.translate(TLBReq::new(addr, Access::Read));
        let res = self.bus.write().unwrap().read16(paddr);
        res
    }
    pub fn read8(&mut self, addr: u32) -> u8 {
        let paddr = self.translate(TLBReq::new(addr, Access::Read));
        let res = self.bus.write().unwrap().read8(paddr);
        res
    }

    pub fn write32(&mut self, addr: u32, val: u32) {
        let paddr = self.translate(TLBReq::new(addr, Access::Write));
        self.bus.write().unwrap().write32(paddr, val);
    }
    pub fn write16(&mut self, addr: u32, val: u32) {
        let paddr = self.translate(TLBReq::new(addr, Access::Write));
        self.bus.write().unwrap().write16(paddr, val as u16);
    }
    pub fn write8(&mut self, addr: u32, val: u32) {
        let paddr = self.translate(TLBReq::new(addr, Access::Write));
        self.bus.write().unwrap().write8(paddr, val as u8);
    }
}

/// These are the functions used to perform virtual-to-physical translation.
impl Cpu {
    /// Resolve a section descriptor, returning a physical address.
    fn resolve_section(&self, req: TLBReq, d: SectionDescriptor) -> u32 {
        let ctx = self.get_ctx(d.domain());
        if ctx.validate(&req, d.ap()) {
            d.base_addr() | req.vaddr.section_idx() 
        } else {
            panic!("Domain access faults are unimplemented, vaddr={:08x}",
                req.vaddr.0);
        }
    }

    /// Resolve a coarse descriptor, returning a physical address.
    fn resolve_coarse(&self, req: TLBReq, d: CoarseDescriptor) -> u32 {
        let desc = self.l2_fetch(req.vaddr, L1Descriptor::Coarse(d));
        match desc {
            L2Descriptor::SmallPage(entry) => {
                let ctx = self.get_ctx(d.domain());
                if ctx.validate(&req, entry.get_ap(req.vaddr)) {
                    entry.base_addr() | req.vaddr.small_page_idx()
                } else {
                    panic!("Domain access faults are unimplemented, vaddr={:08x}",
                        req.vaddr.0);
                }
            },
            _ => panic!("L2 descriptor {:?} unimplemented, vaddr={:08x}", 
                desc, req.vaddr.0),
        }
    }

    /// Get the context for computing permissions associated with some PTE.
    fn get_ctx(&self, dom: u32) -> PermissionContext {
        PermissionContext { 
            domain_mode: self.p15.c3_dacr.domain(dom),
            is_priv: self.reg.cpsr.mode().is_privileged(),
            sysprot: self.p15.c1_ctrl.sysprot_enabled(),
            romprot: self.p15.c1_ctrl.romprot_enabled(),
        }
    }

    /// Given some virtual address, return the first-level PTE.
    fn l1_fetch(&self, vaddr: VirtAddr) -> L1Descriptor {
        let addr = (self.p15.c2_ttbr0 & 0xffff_c000) | vaddr.l1_idx() << 2;
        let val = self.bus.write().unwrap().read32(addr);
        let res = L1Descriptor::from_u32(val);
        match res {
            L1Descriptor::Fault(_) => {
                panic!("pc={:08x} L1 Fault descriptor unimpl, vaddr={:08x}", 
                    self.read_fetch_pc(), vaddr.0);
            },
            _ => {},
        }
        res
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

    /// Translate a virtual address into a physical address.
    pub fn translate(&self, req: TLBReq) -> u32 {
        if self.p15.c1_ctrl.mmu_enabled() {
            let desc = self.l1_fetch(req.vaddr);
            match desc {
                L1Descriptor::Section(entry) => self.resolve_section(req, entry),
                L1Descriptor::Coarse(entry) => self.resolve_coarse(req, entry),
                _ => panic!("TLB first-level descriptor {:?} unimplemented", desc),
            }
        } else {
            req.vaddr.0
        }
    }
}

