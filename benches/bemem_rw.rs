#![feature(test)]

extern crate test;
use test::Bencher;

use ironic::mem::*;
use ironic::mem::host::*;

const SEQ_ACCESS_LEN: usize = 0x10000;

macro_rules! def_bemem_read {
    ($prim:ty, $func_name:ident) => {
        #[bench]
        fn $func_name(b: &mut Bencher) {
            let mem = BigEndianMemory::new(SEQ_ACCESS_LEN, None);
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

macro_rules! def_bemem_write {
    ($prim:ty, $func_name:ident, $val:expr) => {
        #[bench]
        fn $func_name(b: &mut Bencher) {
            let mut mem = BigEndianMemory::new(SEQ_ACCESS_LEN, None);
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


macro_rules! def_hostmem_read {
    ($prim:ty, $func_name:ident) => {
        #[bench]
        fn $func_name(b: &mut Bencher) {
            let mut mem = HostMemBacking::<&str>::new("test", SEQ_ACCESS_LEN);
            mem.add_region("foo", SEQ_ACCESS_LEN);
            mem.enable_region("foo", 0x1000_0000);

            let width = std::mem::size_of::<$prim>();
            b.iter(|| {
                let iter = SEQ_ACCESS_LEN / width;
                for i in 0..iter {
                    unsafe { 
                        let _res = std::ptr::read_volatile::<$prim>(
                            (0x1000_0000 + (i * width)) as *const $prim
                        );
                    }
                }
            })
        }
    }
}

macro_rules! def_hostmem_write {
    ($prim:ty, $func_name:ident, $val:expr) => {
        #[bench]
        fn $func_name(b: &mut Bencher) {
            let mut mem = HostMemBacking::<&str>::new("test", SEQ_ACCESS_LEN);
            mem.add_region("foo", SEQ_ACCESS_LEN);
            mem.enable_region("foo", 0x1000_0000);

            let width = std::mem::size_of::<$prim>();
            b.iter(|| {
                let iter = SEQ_ACCESS_LEN / width;
                for i in 0..iter {
                    unsafe { 
                        let _res = std::ptr::write_volatile::<$prim>(
                            (0x1000_0000 + (i * width)) as *mut $prim, $val
                        );
                    }
                }
            })
        }
    }
}




def_hostmem_read!(u8, seq_read_hostmem_u8);
def_hostmem_read!(u16, seq_read_hostmem_u16);
def_hostmem_read!(u32, seq_read_hostmem_u32);
def_hostmem_read!(u64, seq_read_hostmem_u64);

def_hostmem_write!(u8, seq_write_hostmem_u8, 0xde);
def_hostmem_write!(u16, seq_write_hostmem_u16, 0xdead);
def_hostmem_write!(u32, seq_write_hostmem_u32, 0xdead_cafe);
def_hostmem_write!(u64, seq_write_hostmem_u64, 0xdead_cafe_dead_beef);



def_bemem_read!(u8, seq_read_bemem_u8);
def_bemem_read!(u16, seq_read_bemem_u16);
def_bemem_read!(u32, seq_read_bemem_u32);
def_bemem_read!(u64, seq_read_bemem_u64);

def_bemem_write!(u8, seq_write_bemem_u8, 0xde);
def_bemem_write!(u16, seq_write_bemem_u16, 0xdead);
def_bemem_write!(u32, seq_write_bemem_u32, 0xdead_cafe);
def_bemem_write!(u64, seq_write_bemem_u64, 0xdead_cafe_dead_beef);


