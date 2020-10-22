use crate::bus::prim::*;
use crate::bus::mmio::*;
use crate::bus::task::*;

const DDR_REG_LEN: usize = 0xca + 1;
const SEQ_REG_LEN: usize = 0x4c + 1;

#[derive(Clone)]
pub struct DdrInterface {
    pub ddr_reg: [u16; DDR_REG_LEN],
    pub seq_reg: [u16; SEQ_REG_LEN],

    pub seq_addr: u16,
    pub seq_data: u16,

    pub ahmflush: u16,
    pub ahmflush_ack: u16,
}
impl DdrInterface {
    pub fn new() -> Self {
        DdrInterface {
            ddr_reg: [0; DDR_REG_LEN],
            seq_reg: [0; SEQ_REG_LEN],
            seq_addr: 0,
            seq_data: 0,
            ahmflush: 0,
            ahmflush_ack: 0,
        }
    }
}

impl DdrInterface {
    pub fn seq_read(&mut self, data: u16) {
        self.seq_addr = data;
        self.seq_data = self.seq_reg[data as usize];
    }
    pub fn seq_write(&mut self, data: u16) {
        self.seq_reg[self.seq_addr as usize] = data;
    }
}


impl MmioDevice for DdrInterface {
    type Width = u16;
    fn read(&mut self, off: usize) -> BusPacket {
        let val = match off {
            0x28 => panic!("DDR ahmflush read unimplemented"),
            0x2a => self.ahmflush_ack,
            0xc4 => self.seq_data,
            0xc6 => self.seq_addr,
            _ => self.ddr_reg[off / 2],
        };
        BusPacket::Half(val)
    }
    fn write(&mut self, off: usize, val: u16) -> Option<BusTask> {
        match off {
            // Always immediately ACK some request
            0x28 => {
                self.ahmflush = val;
                self.ahmflush_ack = val;
            },
            0x2a => panic!("DDR ahmflush_ack write unimplemented"),
            0xc4 => self.seq_write(val),
            0xc6 => self.seq_read(val),
            _ => self.ddr_reg[off / 2] = val,
        }
        None
    }
}
