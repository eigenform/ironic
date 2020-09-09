#![feature(test)]

extern crate test;
use test::Bencher;

use ironic::mem::*;

const SEQ_ACCESS_LEN: usize = 0x400;

/// Issue 0x100 bytes of sequential reads, of type $prim.
macro_rules! def_bemem_read {
    ($prim:ty, $func_name:ident) => {
        #[bench]
        fn $func_name(b: &mut Bencher) {
            let mem = ironic::mem::BigEndianMemory::new(SEQ_ACCESS_LEN, None);
            let width = std::mem::size_of::<$prim>();
            b.iter(|| {
                let iter = SEQ_ACCESS_LEN / width;
                for i in 0..iter {
                    let _res = mem.read::<$prim>(i * width);
                }
            })
        }
    }
}

/// Issue 0x100 bytes of sequential reads, of type $prim.
macro_rules! def_bemem_write {
    ($prim:ty, $func_name:ident, $val:expr) => {
        #[bench]
        fn $func_name(b: &mut Bencher) {
            let mut mem = ironic::mem::BigEndianMemory::new(SEQ_ACCESS_LEN, None);
            let width = std::mem::size_of::<$prim>();
            b.iter(|| {
                let iter = SEQ_ACCESS_LEN / width;
                for i in 0..iter {
                    let _res = mem.write::<$prim>(i * width, $val);
                }
            })
        }
    }
}


def_bemem_read!(u8, seq_read_u8_bigendianmemory);
def_bemem_read!(u16, seq_read_u16_bigendianmemory);
def_bemem_read!(u32, seq_read_u32_bigendianmemory);
def_bemem_read!(u64, seq_read_u64_bigendianmemory);

def_bemem_write!(u8, seq_write_u8_bigendianmemory, 0xde);
def_bemem_write!(u16, seq_write_u16_bigendianmemory, 0xdead);
def_bemem_write!(u32, seq_write_u32_bigendianmemory, 0xdead_cafe);
def_bemem_write!(u64, seq_write_u64_bigendianmemory, 0xdead_cafe_dead_beef);


