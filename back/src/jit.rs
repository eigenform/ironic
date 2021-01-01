
pub mod lut;
pub mod dispatch;

use std::sync::{Arc, RwLock};
use crate::jit::lut::*;
use crate::back::*;
use crate::decode::arm::ArmInst;
use crate::decode::thumb::ThumbInst;

use crate::ir::*;
use crate::lut::*;

use ironic_core::bus::*;
use ironic_core::cpu::{Cpu, CpuRes};
use ironic_core::cpu::reg::{Reg, Cond};
use ironic_core::cpu::excep::ExceptionType;


pub struct JitBackend {
    pub graph: IRGraph,
    pub lut: JitLut,
    pub bb: IRBlock,

    pub bus: Arc<RwLock<Bus>>,
    pub cpu: Cpu,
}
impl JitBackend {
    pub fn new(bus: Arc<RwLock<Bus>>) -> Self {
        JitBackend {
            bb: IRBlock::new(),
            graph: IRGraph::new(),
            lut: JitLut::new(),
            cpu: Cpu::new(bus.clone()),
            bus 
        }
    }
}

pub struct InstInfo {
    pub inst: ArmInst,
    pub cond: Option<Cond>,
}
impl InstInfo {
    pub fn new() -> Self {
        InstInfo {
            inst: ArmInst::Undefined,
            cond: None,
        }
    }
}

pub fn get_inst_info(opcd: u32) -> InstInfo {
    use ArmInst::*;
    let mut info = InstInfo::new();
    let cond = Cond::from((opcd & 0xf000_0000) >> 28);
    if cond != Cond::AL {
        info.cond = Some(cond);
    }
    let inst = ArmInst::decode(opcd);
    match inst {
        Undefined => panic!(""),
        _ => panic!("{:?}", inst),
    }

    info
}

impl JitBackend {
    /// Lift a block into the intermediate representation.
    pub fn lift(&mut self) {
        assert!(self.cpu.reg.cpsr.thumb() == false);
        let opcd = self.cpu.read32(self.cpu.read_fetch_pc());
        let info = get_inst_info(opcd);
    }
}


impl Backend for JitBackend {
    fn run(&mut self) {
        let pc = self.cpu.read_fetch_pc();

        match self.graph.blocks.get(&pc) {
            // If no block exists at this PC
            None => self.lift(),

            // If a block is already built for this PC
            Some(_) => panic!(""),
        }
    }
}

