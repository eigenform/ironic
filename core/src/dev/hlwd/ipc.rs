use crate::bus::task::*;
use crate::dev::hlwd::irq::*;

#[derive(Clone, Default, Debug)]
pub struct MailboxState {
    pub ppc_req: bool,
    pub ppc_ack: bool,
    pub ppc_req_int: bool,
    pub ppc_ack_int: bool,

    pub arm_req: bool,
    pub arm_ack: bool,
    pub arm_req_int: bool,
    pub arm_ack_int: bool,
}
impl MailboxState {

    // Working on the best way to represent this, this might be wrong
    // init - set 0x38
    // sendrequest - set 0x1, clear 0x2,0x4, 0x8
    // replyhandler - set 0x4, clear 0x1,0x2,0x8; then set 0x8, clear 0x1,0x2,0x4
    // ackhandler - set 0x2, clear 0x1,0x4,0x8; then set 0x8,clear 0x1,0x2,0x4

    pub fn ppc_ctrl_write(&mut self, x: u32) {
        self.arm_req = x & 0x0000_0001 != 0;
        self.arm_ack = x & 0x0000_0008 != 0;
        if x & 0x0000_0002 != 0 { self.ppc_ack = false; }
        if x & 0x0000_0004 != 0 { self.ppc_req = false; }
        self.ppc_req_int = x & 0x0000_0010 != 0;
        self.ppc_ack_int = x & 0x0000_0020 != 0;
    }
    pub fn arm_ctrl_write(&mut self, x: u32) {
        self.ppc_req = x & 0x0000_0001 != 0;
        self.ppc_ack = x & 0x0000_0008 != 0;
        if x & 0x0000_0002 != 0 { self.arm_ack = false; }
        if x & 0x0000_0004 != 0 { self.arm_req = false; }
        self.arm_req_int = x & 0x0000_0010 != 0;
        self.arm_ack_int = x & 0x0000_0020 != 0;

    }
    pub fn ppc_ctrl_read(&self) -> u32 {
        let mut res = 0;
        res |= (self.arm_req as u32) << 0;
        res |= (self.ppc_ack as u32) << 1;
        res |= (self.ppc_req as u32) << 2;
        res |= (self.arm_ack as u32) << 3;
        res |= (self.ppc_req_int as u32) << 4;
        res |= (self.ppc_ack_int as u32) << 5;
        res
    }
    pub fn arm_ctrl_read(&self) -> u32 {
        let mut res = 0;
        res |= (self.ppc_req as u32) << 0;
        res |= (self.arm_ack as u32) << 1;
        res |= (self.arm_req as u32) << 2;
        res |= (self.ppc_ack as u32) << 3;
        res |= (self.arm_req_int as u32) << 4;
        res |= (self.arm_ack_int as u32) << 5;
        res
    }

}

/// The inter-processor communication interface.
#[derive(Debug, Clone)]
pub struct IpcInterface {
    pub ppc_msg: u32,
    pub arm_msg: u32,
    pub state: MailboxState,

}
impl IpcInterface {
    pub fn new() -> Self {
        IpcInterface {
            ppc_msg: 0, arm_msg: 0,
            state: MailboxState::default(),
        }
    }

    /// Returns true if a PPC IPC interrupt is currently asserted.
    pub fn assert_ppc_irq(&self) -> bool {
        (self.state.ppc_req_int && self.state.ppc_req) || 
        (self.state.ppc_ack_int && self.state.ppc_ack)
    }
    /// Returns true is an ARM IPC interrupt is currently asserted.
    pub fn assert_arm_irq(&self) -> bool {
        (self.state.arm_req_int && self.state.arm_req) || 
        (self.state.arm_ack_int && self.state.arm_ack)
    }
}

impl IpcInterface {
    pub fn read_handler(&self, off: usize) -> u32 {
        match off {
            0x00 => self.ppc_msg,
            0x04 => self.state.ppc_ctrl_read(),
            0x08 => self.arm_msg,
            0x0c => self.state.arm_ctrl_read(),
            _ => unreachable!(),
        }
    }
    pub fn write_handler(&mut self, off: usize, val: u32) {
        match off {
            0x00 => {
                println!("IPC PPC MSG write {:08x}", val);
                self.ppc_msg = val;
            }
            0x04 => {
                println!("IPC PPC CTRL write {:08x}", val);
                self.state.ppc_ctrl_write(val);
            },
            0x08 => {
                println!("IPC ARM MSG write {:08x}", val);
                self.arm_msg = val;
            },
            0x0c => {
                println!("IPC ARM CTRL write {:08x}", val);
                self.state.arm_ctrl_write(val);
            },
            _ => unreachable!(),
        }
    }
}

