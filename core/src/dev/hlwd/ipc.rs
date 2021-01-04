use crate::bus::task::*;
use crate::dev::hlwd::irq::*;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct IpcCtrlReg(pub u32);
impl IpcCtrlReg {
    pub fn fla0(&self) -> bool { self.0 & 0x0000_0001 != 0 }
    pub fn ack1(&self) -> bool { self.0 & 0x0000_0002 != 0 }
    pub fn ack0(&self) -> bool { self.0 & 0x0000_0004 != 0 }
    pub fn fla1(&self) -> bool { self.0 & 0x0000_0008 != 0 }
    pub fn int0(&self) -> bool { self.0 & 0x0000_0010 != 0 }
    pub fn int1(&self) -> bool { self.0 & 0x0000_0020 != 0 }
    pub fn trigger_int0(&self) -> bool { self.0 & 0x0000_0014 == 0x0000_0014 }
    pub fn trigger_int1(&self) -> bool { self.0 & 0x0000_0022 == 0x0000_0022 }
}

/// The inter-processor communication interface.
#[derive(Debug, Clone)]
pub struct IpcInterface {
    pub ppc_msg: u32,
    pub ppc_ctrl: IpcCtrlReg,
    pub arm_msg: u32,
    pub arm_ctrl: IpcCtrlReg,
}
impl IpcInterface {
    pub fn new() -> Self {
        IpcInterface {
            ppc_msg: 0,
            ppc_ctrl: IpcCtrlReg(0),
            arm_msg: 0,
            arm_ctrl: IpcCtrlReg(0),
        }
    }
    pub fn step(&mut self) -> Option<HollywoodIrq> {
        if self.ppc_ctrl.trigger_int0() || self.ppc_ctrl.trigger_int1() {
            return Some(HollywoodIrq::PpcIpc);
        }
        if self.arm_ctrl.trigger_int0() || self.arm_ctrl.trigger_int1() {
            return Some(HollywoodIrq::ArmIpc);
        }
        None
    }
}
impl IpcInterface {
    pub fn read_handler(&self, off: usize) -> u32 {
        match off {
            0x00 => self.ppc_msg,
            0x04 => self.ppc_ctrl.0,
            0x08 => self.arm_msg,
            0x0c => self.arm_ctrl.0,
            _ => unreachable!(),
        }
    }
    pub fn write_handler(&mut self, off: usize, val: u32) {
        match off {
            0x00 => self.ppc_msg = val,
            0x04 => {
                println!("IPC PPC CTRL write {:08x}", val);
                self.ppc_ctrl.0 &= 0x0000_0006;
                self.arm_ctrl.0 &= 0x0000_0039;
                self.ppc_ctrl.0 |= val & 0x0000_0039;
                self.ppc_ctrl.0 &= !(val & 0x0000_0006);
                let new = IpcCtrlReg(val);
                if new.fla0() { self.arm_ctrl.0 |= 0x0000_0004; }
                if new.fla1() { self.arm_ctrl.0 |= 0x0000_0002; }
                if new.ack0() { self.arm_ctrl.0 &= !0x0000_0001; }
                if new.ack1() { self.arm_ctrl.0 &= !0x0000_0008; }
            },
            0x08 => self.arm_msg = val,
            0x0c => {
                println!("IPC ARM CTRL write {:08x}", val);
                self.arm_ctrl.0 &= 0x0000_0006;
                self.ppc_ctrl.0 &= 0x0000_0039;
                self.arm_ctrl.0 |= val & 0x0000_0039;
                self.arm_ctrl.0 &= !(val & 0x0000_0006);
                let new = IpcCtrlReg(val);
                if new.fla0() { self.ppc_ctrl.0 |= 0x0000_0004; }
                if new.fla1() { self.ppc_ctrl.0 |= 0x0000_0002; }
                if new.ack0() { self.ppc_ctrl.0 &= !0x0000_0001; }
                if new.ack1() { self.ppc_ctrl.0 &= !0x0000_0008; }
            },
            _ => unreachable!(),
        }
    }
}

