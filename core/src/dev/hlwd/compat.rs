use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;
use crate::bus::Bus;

/// Legacy disc drive interface.
#[derive(Default, Debug, Clone)]
pub struct DriveInterface {
    disr: u32,
    dicvr: u32,
    dicmdbuf: [u32; 3],
    dimar: u32,
    dilength: u32,
    dicr: u32,
    diimmbuf: u32,
    dicfg: u32,
}
impl MmioDevice for DriveInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x24 => self.dicfg,
            _ => panic!("DI read to undefined offset {:x}", off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        match off {
            _ => panic!("DI write {:08x} to undefined offset {:x}", val, off),
        }
        None
    }
}

/// Legacy EXI
#[derive(Default, Debug, Clone)]
pub struct EXInterface {
    /// Buffer for instructions used to bootstrap PPC-world
    pub ppc_bootstrap: [u32; 0x10],
}
impl MmioDevice for EXInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket { 
        let val = match off {
            0x40..=0x7c => self.ppc_bootstrap[(off - 0x40)/4],
            _ => panic!("EXI read to undef offset {:x}", off),
        };
        println!("EXI read {:08x} from {:x}", val, off);
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> { 
        println!("EXI write {:08x} to {:x}", val, off);
        match off { 
            0x40..=0x7c => self.ppc_bootstrap[(off - 0x40)/4] = val,
            _ => panic!("EXI write {:08x} to {:x}", val, off),
        }
        None
    }
}


/// Legacy memory interface.
#[derive(Clone)]
pub struct MemInterface {
    pub reg: [u16; 0x40],
    pub ddr_data: u16,
    pub ddr_addr: u16,
}
impl MemInterface {
    pub fn new() -> Self {
        MemInterface {
            reg: [0; 0x40],
            ddr_data: 0,
            ddr_addr: 0,
        }
    }
}
impl MmioDevice for MemInterface {
    type Width = u16;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x74 => self.ddr_addr,
            0x76 => self.ddr_data,
            _ => self.reg[off / 2],
        };
        BusPacket::Half(val)
    }
    fn write(&mut self, off: usize, val: u16) -> Option<BusTask> {
        let task = match off {
            0x74 => Some(BusTask::Mi { kind: IndirAccess::Read, data: val }),
            0x76 => Some(BusTask::Mi { kind: IndirAccess::Write, data: val }),
            _ => { self.reg[off / 2] = val; None }
        };
        task
    }
}


impl Bus {
    pub fn handle_task_mi(&mut self, kind: IndirAccess, data: u16) {
        let local_ref = self.dev.clone();
        let mut dev = local_ref.write().unwrap();
        let hlwd = &mut dev.hlwd;

        match kind {
            IndirAccess::Read => {
                assert!(data >= 0x0100);
                hlwd.mi.ddr_addr = data;
                let off = ((data * 2) - 0x0200) as usize;
                let res = hlwd.ddr.read(off);
                hlwd.mi.ddr_data = match res {
                    BusPacket::Half(val) => val,
                    _ => unreachable!(),
                };
            },
            IndirAccess::Write => {
                let ddr_addr = hlwd.mi.ddr_addr;
                assert!(ddr_addr >= 0x0100);
                let off = ((hlwd.mi.ddr_addr * 2) - 0x200) as usize;
                hlwd.ddr.write(off, data);
            }
        }
    }
}

