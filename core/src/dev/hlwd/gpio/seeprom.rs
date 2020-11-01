
use std::io::Read;
use std::fs::File;
use crate::dev::hlwd::gpio::*;
use crate::mem::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeepromOp {
    Ewds    = 0,
    Wral    = 1,
    Eral    = 2,
    Ewen    = 3,
    Ext     = 4,
    Write   = 5,
    Read    = 6,
    Erase   = 7,
    Init,
}
impl From<u32> for SeepromOp {
    fn from(x: u32) -> Self {
        use SeepromOp::*;
        match x {
            0 => Ewds,
            1 => Wral,
            2 => Eral,
            3 => Ewen,
            4 => Ext,
            5 => Write,
            6 => Read,
            7 => Erase,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct SeepromState {
    pub in_buf: u32,
    pub num_bits: u32,
    pub out_buf: u16,
    pub state: SeepromOp,
    pub data: BigEndianMemory,
}
impl SeepromState {
    pub fn new() -> Self {
        SeepromState {
            in_buf: 0,
            num_bits: 0,
            out_buf: 0,
            state: SeepromOp::Init,
            data: BigEndianMemory::new(0x100, Some("seeprom.bin")),
        }
    }
}
impl SeepromState {
    pub fn reset(&mut self) {
        self.in_buf = 0;
        self.out_buf = 0;
        self.num_bits = 0;
        self.state = SeepromOp::Init;
    }

    pub fn step(&mut self, mosi: u32, input: u32) -> Option<u32> {
        use SeepromOp::*;

        // Shift in a bit
        self.in_buf = (self.in_buf << 1) | mosi;
        self.num_bits += 1;

        // Potentially change the state of the machine
        match self.num_bits {
            0x03 => {
                self.state = SeepromOp::from(self.in_buf);
            },
            0x05 => if self.state == Ext {
                self.state = SeepromOp::from(self.in_buf & 0x03);
            },
            0x0b => if self.state == Read {
                let addr = (self.in_buf & 0x7f) as usize;
                self.out_buf = self.data.read::<u16>(addr * 2);
                //println!("SEEPROM read {:04x} from {:x}", self.out_buf, addr);
            },
            0x1b => if self.state == Write {
                let addr = ((self.in_buf >> 16) & 0x7f) as usize;
                let data = (self.in_buf & 0xffff) as u16;
                self.data.write::<u16>(addr * 2, data);
                //println!("SEEPROM write {:04x} to {:x}", data, addr);
            },
            _ => {},
        }

        // Shift out bits from a read command
        if self.state == SeepromOp::Read && self.num_bits > 0x0b {
            if self.out_buf & (0x8000 >> self.num_bits - 0xc) != 0 {
                Some(input | GpioPin::SeepromMiso as u32)
            } else {
                Some(input & !(GpioPin::SeepromMiso as u32))
            }
        } else {
            None
        }
    }
}

impl GpioInterface {
    pub fn handle_seeprom(&mut self, val: u32) {
        let mosi = ((val & GpioPin::SeepromMosi as u32) != 0) as u32;
        let cs = (val & GpioPin::SeepromCs as u32) != 0;
        let clk_rise = (self.arm.output & GpioPin::SeepromClk as u32) == 0 
            && (val & GpioPin::SeepromClk as u32) != 0;

        // If CS is asserted and we're at the rising edge of the clock,
        // compute the next step of the serial/SPI state machine
        if !cs {
            self.seeprom.reset();
        } 
        if cs && clk_rise {
            let new_input = self.seeprom.step(mosi, self.arm.input);
            if new_input.is_some() {
                self.arm.input = new_input.unwrap();
            }
        }

        // Commit the value to the output register
        self.arm.output = val;
    }
}

