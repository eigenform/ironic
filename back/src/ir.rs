//! An intermediate representation for dynamically recompiling guest code.

use std::fmt;
use std::collections::BTreeMap;


// Just going to use this for representing conditions, for now
use ironic_core::cpu::reg::Cond;
use crate::decode::arm::ArmInst;
use crate::decode::thumb::ThumbInst;

/// An abstract representation of a program.
pub struct IRGraph {
    /// The set of basic blocks associated with this graph.
    pub blocks: BTreeMap<u32, IRBlock>,
    /// Buffer for the state of the current basic block.
    pub current_block: IRBlock,
    /// The newest IR index.
    pub ridx: usize,
}
impl IRGraph {
    pub fn new() -> Self {
        IRGraph { 
            blocks: BTreeMap::new(),
            current_block: IRBlock::new(),
            ridx: 0,
        }
    }
    pub fn print(&self) {
        for op in self.current_block.expr.iter() {
            println!("{:?}", op);
        }
    }
}
impl IRGraph {
    /// Allocate a new register for the current block.
    pub fn allocate_reg(&mut self) -> IRReg {
        let reg = IRReg { id: self.ridx };
        self.current_block.reg.push(reg);
        self.ridx += 1;
        reg
    }
    /// Add some operation to the current block.
    pub fn op(&mut self, op: IROp) {
        self.current_block.expr.push(op);
    }
}

/// Representation of some primitive operation/expresion in the IR. 
#[derive(Clone)]
pub enum IROp {
    // x := a
    Id(IRReg, IRValue),
    // x := a + b
    Add(IRReg, IRReg, IRReg), 
    // x := a - b
    Sub(IRReg, IRReg, IRReg), 
    // x := Read<a>(b)
    Read(IRReg, IRWidth, IRReg),

}
impl fmt::Debug for IROp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use IROp::*;
        match self {
            Id(x, a) => 
                write!(f, "ir{} := {:?}", x.id, a),
            Add(x, a, b) => 
                write!(f, "ir{} := ir{} + ir{}", x.id, a.id, b.id),
            Sub(x, a, b) => 
                write!(f, "ir{} := ir{} - ir{}", x.id, a.id, b.id),
            Read(x, a, b) => 
                write!(f, "ir{} := Read{:?}(ir{})", x.id, a, b.id),
        }
    }
}

#[derive(Clone)]
pub enum IRInst {
    Arm(ArmInst, u32),
    Thumb(ThumbInst, u16),
}

/// A basic block of computations.
#[derive(Clone)]
pub struct IRBlock {
    pub inst: Vec<IRInst>,
    pub reg: Vec<IRReg>,
    pub expr: Vec<IROp>,
    pub cond_exit: Option<Cond>,
}
impl IRBlock {
    pub fn new() -> Self {
        IRBlock { 
            inst: Vec::new(),
            reg: Vec::new(), 
            expr: Vec::new(),
            cond_exit: None,
        }
    }
}

/// The result of some operation.
#[derive(Clone, Copy, Debug)]
pub struct IRReg { pub id: usize, }

/// A particular register in the target ISA.
pub type TargetReg = usize;

/// The width of some value.
#[derive(Clone, Debug)]
pub enum IRWidth { U8, U16, U32 }

/// An immediate value.
#[derive(Clone, Debug)]
pub struct IRImm {
    pub value: usize,
    pub width: IRWidth,
}

/// Representation of some value or free variable in the IR.
#[derive(Clone)]
pub enum IRValue {
    IReg(IRReg),
    TReg(TargetReg),
    Imm(IRImm),
}
impl fmt::Debug for IRValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use IRValue::*;
        match self {
            IReg(x) => write!(f, "ir{}", x.id),
            TReg(x) => write!(f, "tr{}", x),
            Imm(x) => {
                match x.width {
                    IRWidth::U8 => write!(f, "#0x{:02x}", x.value),
                    IRWidth::U16 => write!(f, "#0x{:04x}", x.value),
                    IRWidth::U32 => write!(f, "#0x{:08x}", x.value),
                }
            },
        }
    }
}
impl IRValue {
    pub fn u32(value: usize) -> Self {
        IRValue::Imm(IRImm { value, width: IRWidth::U32 })
    }
    pub fn tr(r: TargetReg) -> Self {
        IRValue::TReg(r)
    }
}


fn build_graph(g: &mut IRGraph) {
    let res = g.allocate_reg();
    let a = g.allocate_reg();
    let b = g.allocate_reg();

    // a := r0
    g.op(IROp::Id(a, IRValue::tr(0)));

    // b := 0xdeadbeef
    g.op(IROp::Id(b, IRValue::u32(0xdeadbeef)));

    // x := a + b
    g.op(IROp::Add(res, a, b));
}


