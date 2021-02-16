use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;
use crate::bus::Bus;

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
        match kind {
            IndirAccess::Read => {
                assert!(data >= 0x0100);
                self.hlwd.mi.ddr_addr = data;
                let off = ((data * 2) - 0x0200) as usize;
                let res = self.hlwd.ddr.read(off);
                self.hlwd.mi.ddr_data = match res {
                    BusPacket::Half(val) => val,
                    _ => unreachable!(),
                };
            },
            IndirAccess::Write => {
                let ddr_addr = self.hlwd.mi.ddr_addr;
                assert!(ddr_addr >= 0x0100);
                let off = ((self.hlwd.mi.ddr_addr * 2) - 0x200) as usize;
                self.hlwd.ddr.write(off, data);
            }
        }
    }
}

