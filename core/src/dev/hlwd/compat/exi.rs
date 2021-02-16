pub mod device;
use device::*;

use crate::bus::mmio::*;
use crate::bus::prim::*;
use crate::bus::task::*;

/// Representing user-configurable EXI clock freqencies.
#[derive(Debug, Clone, Copy)]
pub enum EXIFreq {
    Clk1Mhz, Clk2Mhz, Clk4Mhz, Clk8Mhz, Clk16Mhz, Clk32Mhz, Undef
}
impl From<u32> for EXIFreq {
    fn from(x: u32) -> Self {
        match x {
            0b000 => Self::Clk1Mhz,
            0b001 => Self::Clk2Mhz,
            0b010 => Self::Clk4Mhz,
            0b011 => Self::Clk8Mhz,
            0b100 => Self::Clk16Mhz,
            0b101 => Self::Clk32Mhz,
            0b110 | 0b111 => Self::Undef,
            _ => unreachable!(),
        }
    }
}

/// Representing an EXI transfer type.
#[derive(Debug, Clone, Copy)]
pub enum EXITransfer {
    Read, Write, ReadWrite, Undef,
}
impl From<u32> for EXITransfer {
    fn from(x: u32) -> Self {
        match x {
            0b00 => Self::Read,
            0b10 => Self::Write,
            0b01 => Self::ReadWrite,
            0b11 => Self::Undef,
            _ => unreachable!(),
        }
    }
}

/// Container for the state associated with an EXI channel, determined by the 
/// current value of the channel's status and control registers.
#[derive(Debug, Clone, Copy)]
pub struct ChannelState {
    /// Device connected flag
    pub ext: bool,

    /// External Insertion Interrupt Flag
    pub ext_int: bool,
    /// External Insertion Interrupt Mask
    pub ext_msk: bool,

    /// Currently-selected EXI device
    pub dev: Option<EXIDeviceKind>,
    /// Channel clock frequency
    pub clk: EXIFreq,

    /// Transfer Complete Interrupt Flag
    pub tc_int: bool,
    /// Transfer Complete Interrupt Mask
    pub tc_msk: bool,

    /// EXI Interrupt flag
    pub exi_int: bool,
    /// EXI Interrupt mask
    pub exi_msk: bool,

    /// Size of pending immediate transfer in bytes
    pub imm_len: u32,
    /// The type of pending transfer
    pub transfer_type: EXITransfer,
    /// DMA transfer mode (otherwise, immediate transfer)
    pub dma: bool,
    /// Transfer status bit
    pub transfer: bool,
}
impl ChannelState {
    fn from_chn(chn: usize, sts: u32, ctrl: u32) -> Self {
        // Status register bits
        let ext     = sts & 0x0000_1000 != 0;
        let ext_int = sts & 0x0000_0800 != 0;
        let ext_msk = sts & 0x0000_0400 != 0;
        let tc_int  = sts & 0x0000_0008 != 0;
        let tc_msk  = sts & 0x0000_0004 != 0;
        let exi_int = sts & 0x0000_0002 != 0;
        let exi_msk = sts & 0x0000_0001 != 0;

        let dev     = EXIDeviceKind::resolve(chn, (sts & 0x0000_0380) >> 7);
        let clk     = EXIFreq::from((sts & 0x0000_0070) >> 4);

        // Control register bits
        let imm_len = (ctrl & 0x0000_0030) >> 4;
        let transfer_type = EXITransfer::from((ctrl& 0x0000_000c) >> 2);
        let dma = ctrl & 0x0000_0002 != 0;
        let transfer = ctrl & 0x0000_0001 != 0;

        ChannelState {
            ext, ext_int, ext_msk, 
            dev, clk, 
            tc_int, tc_msk, 
            exi_int, exi_msk,
            imm_len, transfer_type, dma, transfer
        }
    }
}

/// Representing a single channel on the external interface.
#[derive(Debug, Clone)]
pub struct EXIChannel {
    /// Channel index
    idx: usize,
    /// Status register value
    pub csr: u32,
    /// DMA address register value
    pub mar: u32,
    /// DMA length register value
    pub len: u32,
    /// Control register value
    pub ctrl: u32,
    /// Immediate data register value
    pub data: u32,
    /// Channel state
    pub state: ChannelState,
}
impl EXIChannel {
    pub fn new(idx: usize) -> Self {
        EXIChannel {
            idx, csr: 0, mar: 0, len: 0, data: 0, ctrl: 0,
            state: ChannelState::from_chn(idx, 0, 0),
        }
    }
}

/// Per-channel read/write handlers.
impl EXIChannel {
    pub fn read(&self, off: usize) -> u32 {
        let res = match off {
            0x00 => self.csr,
            0x04 => self.mar,
            0x08 => self.len,
            0x0c => self.ctrl,
            0x10 => self.data,
            _ => panic!("EXI chn{} OOB read at {:08x}", self.idx, off),
        };
        println!("EXI chn{} read {:08x} from offset {:x}", self.idx, res, off);
        res
    }
    pub fn write(&mut self, off: usize, val: u32) {
        match off {
            0x00 => {
                self.csr = val;
                self.update_state();
            }
            0x04 => self.mar = val,
            0x08 => self.len = val,
            0x0c => {
                self.ctrl = val;
                self.update_state();
            },
            0x10 => self.data = val,
            _ => panic!("EXI chn{} OOB write {:08x} at {:08x}", 
                self.idx, val, off),
        }
    }

    pub fn update_state(&mut self) {
        self.state = ChannelState::from_chn(self.idx, self.csr, self.ctrl);

        if self.state.transfer {
            panic!("EXI transfer unimpl");
        }
    }
}


/// Legacy external interface (EXI).
#[derive(Debug, Clone)]
pub struct EXInterface {
    /// EXI Channel 0 state
    pub chan0: EXIChannel,
    /// EXI Channel 1 state
    pub chan1: EXIChannel,
    /// EXI Channel 2 state
    pub chan2: EXIChannel,
    /// Buffer for Broadway bootstrap instructions
    pub ppc_bootstrap: [u32; 0x10],
}
impl EXInterface {
    pub fn new() -> Self {
        EXInterface {
            chan0: EXIChannel::new(0),
            chan1: EXIChannel::new(1),
            chan2: EXIChannel::new(2),
            ppc_bootstrap: [0; 0x10],
        }
    }
}


impl MmioDevice for EXInterface {
    type Width = u32;
    fn read(&mut self, off: usize) -> BusPacket { 
        let val = match off {
            0x00..=0x10 => self.chan0.read(off),
            0x14..=0x24 => self.chan1.read(off - 0x14),
            0x28..=0x38 => self.chan2.read(off - 0x28),

            0x40..=0x7c => self.ppc_bootstrap[(off - 0x40)/4],
            _ => panic!("EXI read to undef offset {:x}", off),
        };
        BusPacket::Word(val)
    }
    fn write(&mut self, off: usize, val: u32) -> Option<BusTask> { 
        match off { 
            0x00..=0x10 => self.chan0.write(off, val),
            0x14..=0x24 => self.chan1.write(off - 0x14, val),
            0x28..=0x38 => self.chan2.write(off - 0x28, val),


            0x40..=0x7c => self.ppc_bootstrap[(off - 0x40)/4] = val,
            _ => panic!("EXI write {:08x} to {:x}", val, off),
        }
        None
    }
}


