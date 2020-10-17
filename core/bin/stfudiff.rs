
use std::env;
use std::fs::File;
use std::io::Read;

use ironic_core::cpu::reg::*;

unsafe fn to_mut_slice<T: Sized>(x: &mut T) -> &mut [u8] {
    std::slice::from_raw_parts_mut(
        (x as *mut T) as *mut u8, std::mem::size_of::<T>()
    )
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: stfudiff <input file A> <input file B>");
        return;
    }

    let mut fd0 = File::open(args[1].as_str()).unwrap_or_else(|e| 
        panic!("Couldn't open {:?}: {:?}", args[1], e));
    let mut fd1 = File::open(args[2].as_str()).unwrap_or_else(|e|
        panic!("Couldn't open {:?}: {:?}", args[2], e));

    let mut s0 = RegisterFile { r: [0; 15], pc: 0, cpsr: Psr(0) };
    let mut s1 = RegisterFile { r: [0; 15], pc: 0, cpsr: Psr(0) };
    let mut num_steps = 0;
    let mut diffs = 0;

    loop { 
        let mut s0_slice = unsafe { to_mut_slice(&mut s0) };
        let mut s1_slice = unsafe { to_mut_slice(&mut s1) };
        fd0.read_exact(&mut s0_slice).unwrap();
        fd1.read_exact(&mut s1_slice).unwrap();

        println!("({}) A: {:x?}", num_steps, s0);
        println!("({}) B: {:x?}", num_steps, s1);
        println!("");

        if s0 != s1 {
            diffs += 1;
            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            println!("[!] State diverges at step {}", num_steps);
            if s0.pc != s1.pc {
                println!("{:4} {:08x} {:08x}", "r15", s0.pc, s1.pc);
            }
            if s0.cpsr != s1.cpsr {
                println!("{:4} {:08x} {:08x}", "cpsr", s0.cpsr.0, s1.cpsr.0);
            }
            for i in 0..15 {
                if s0.r[i] != s1.r[i] {
                    println!("r{} {:08x} {:08x}", i, s0.r[i], s1.r[i]); 
                }
            }
            if diffs >= 30 {
                break;
            }
        }
        num_steps += 1;
    }
}
