
use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

#[derive(Default)]
pub struct OhcInterface {
    pub idx: usize,

    pub rev: u32,
    pub ctrl: u32,
    pub cmd_status: u32,
    pub int_status: u32,
    pub int_en: u32,
    pub int_dis: u32,

    pub hcca: u32,
    pub ed_period_current: u32,
    pub ed_ctrl_head: u32,
    pub ed_ctrl_current: u32,
    pub ed_bulk_head: u32,
    pub ed_bulk_current: u32,
    pub ed_done_head: u32,

    pub frame_interval: u32,
    pub frame_remaining: u32,
    pub frame_number: u32,
    pub period_start: u32,
    pub ls_threshold: u32,

    pub rh_desc_a: u32,
    pub rh_desc_b: u32,
    pub rh_status: u32,
    pub rh_port_status_a: u32,
    pub rh_port_status_b: u32,

}

impl MmioDevice for OhcInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x00 => 0x0000_0110,
            // NOTE: Everything is wired to 0 in skyeye; good enough for now
            0x04 |
            0x08 |
            0x48 |
            0x4c |
            0x50 => 0,

            _ => panic!("OHCI#{} read at {:x} unimpl", self.idx, off),
        };
        println!("OH{} read {:08x} at {:x}", self.idx, val, off);
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> {
        println!("OH{} write {:08x} at {:x}", self.idx, val, off);
        match off {
            0x04 => self.ctrl = val,
            0x08 => self.cmd_status = val,
            0x0c => self.int_status = val,
            0x10 => self.int_en = val,
            0x14 => self.int_dis = val,
            0x18 => self.hcca = val,
            0x20 => self.ed_ctrl_head = val,
            0x28 => self.ed_bulk_head = val,
            0x34 => self.frame_interval = val,
            0x40 => self.period_start = val,
            0x48 => self.rh_desc_a = val,
            0x4c => self.rh_desc_b = val,
            0x50 => self.rh_status = val,
            _ => panic!("OHCI#{} write {:08x} at {:x} unimpl", self.idx, val, off),
        }
        None
    }
}
