//! Backend for handling PowerPC HLE.
//!
//! NOTE: The socket is blocking right now, but I guess ultimately we don't
//! want that. 

use ironic_core::bus::*;
use ironic_core::dev::hlwd::irq::*;
use crate::back::*;

use std::thread;
use std::sync::{Arc, RwLock};
use std::os::unix::net::{UnixStream, UnixListener};
use std::net::Shutdown;
use std::io::{Read, Write};
use std::convert::TryInto;

extern crate pretty_hex;
use pretty_hex::*;

/// A type of command sent over the socket.
#[derive(Debug)]
#[repr(u32)]
pub enum Command { HostWrite, HostRead, Message, Unimpl }
impl Command {
    fn from_u32(x: u32) -> Self {
        match x {
            1 => Self::HostRead,
            2 => Self::HostWrite,
            3 => Self::Message,
            _ => Self::Unimpl,
        }
    }
}

/// A request packet from the socket.
#[repr(C)]
pub struct SocketReq {
    pub cmd: Command,
    pub addr: u32,
    pub len: u32,
}
impl SocketReq {
    pub fn from_buf(s: &[u8; 0xc]) -> Self {
        let cmd = Command::from_u32(
            u32::from_le_bytes(s[0..4].try_into().unwrap())
        );
        let addr = u32::from_le_bytes(s[0x4..0x8].try_into().unwrap());
        let len = u32::from_le_bytes(s[0x8..0xc].try_into().unwrap());
        SocketReq { cmd, addr, len }
    }
}

pub const IPC_SOCK: &str = "/tmp/ironic.sock";
pub const BUF_LEN: usize = 0x10000;

pub struct PpcBackend {
    /// Reference to the system bus.
    pub bus: Arc<RwLock<Bus>>,
    /// Input buffer for the socket.
    pub ibuf: [u8; BUF_LEN],
    /// Output buffer for the socket.
    pub obuf: [u8; BUF_LEN],
}
impl PpcBackend {
    pub fn new(bus: Arc<RwLock<Bus>>) -> Self {
        PpcBackend {
            bus,
            ibuf: [0; BUF_LEN],
            obuf: [0; BUF_LEN],
        }
    }
}


impl PpcBackend {
    fn recv(&mut self, client: &mut UnixStream) -> Option<usize> {
        let res = client.read(&mut self.ibuf);
        match res {
            Ok(len) => {
                if len == 0 { 
                    println!("[PPC] client HUP"); 
                    None
                } else {
                    Some(len)
                }
            },
            Err(e) => {
                println!("[PPC] IO error {:?}", e);
                None
            }
        }
    }
}


impl PpcBackend {
    pub fn server_loop(&mut self, sock: UnixListener) {
        loop {
            let res = sock.accept();
            let mut client = match res {
                Ok((stream, _)) => stream,
                Err(e) => { 
                    println!("[PPC] accept() error {:?}", e);
                    break;
                }
            };

            if self.wait_for_broadway(&mut client) {
                self.handle_client(&mut client);
                client.shutdown(Shutdown::Both).unwrap();
            } else {
                client.shutdown(Shutdown::Both).unwrap();
            }
        }
    }

    fn wait_for_broadway(&mut self, client: &mut UnixStream) -> bool {
        loop {
            if self.bus.read().unwrap().hlwd.ppc_on {
                let res = client.write("READY".as_bytes());
                match res {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            } else {
                thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }

    fn handle_ppc_irq(&mut self) -> Option<u32> {
        let mut bus = self.bus.write().unwrap();
        if bus.hlwd.irq.ppc_irq_pending(HollywoodIrq::PpcIpc) {

            // ARM-world ACK'ed our message, so clear it
            if bus.hlwd.ipc.state.ppc_ack {
                bus.hlwd.ipc.state.ppc_ack = false;
            }

            // ARM-world sent us a message, so ACK it
            if bus.hlwd.ipc.state.ppc_req {
                bus.hlwd.ipc.state.arm_ack = true;

                let msg = bus.hlwd.ipc.arm_msg;
                bus.hlwd.ipc.state.ppc_req = false;

                bus.hlwd.irq.ppc_irq_status.unset(HollywoodIrq::PpcIpc);
                return Some(msg);
            }
        }
        None
    }

    pub fn handle_client(&mut self, client: &mut UnixStream) {
        loop {

            let msg = self.handle_ppc_irq();
            if msg.is_some() {
                println!("[PPC] got message {:08x} from ARM", msg.unwrap());
                panic!("unimpl");
            }

            println!("[PPC] waiting for command ...");
            let res = self.wait_for_command(client);
            let wait_for_int = match res {
                Err(_) => break,
                Ok(block) => block,
            };
            if wait_for_int {
                println!("[PPC] waiting for ARM to ack ...");
                loop {
                    let asserted = self.bus.read().unwrap().hlwd.irq
                        .ppc_irq_pending(HollywoodIrq::PpcIpc);
                    if asserted {
                        let mut bus = self.bus.write().unwrap();
                        if bus.hlwd.ipc.state.ppc_ack {
                            bus.hlwd.ipc.state.ppc_ack = false;
                            let arm_msg = bus.hlwd.ipc.arm_msg;
                            client.write("ACK".as_bytes()).unwrap();
                            println!("[PPC] ARM acked");
                            break;
                        }
                        bus.hlwd.irq.ppc_irq_status.unset(HollywoodIrq::PpcIpc);
                    } else {
                        thread::sleep(std::time::Duration::from_millis(100));
                    }
                }
            }
        }
    }
}

/// Functions for handling particular commands from a client.
impl PpcBackend {
    fn wait_for_command(&mut self, client: &mut UnixStream) -> Result<bool, ()> {
        let res = self.recv(client);
        if res.is_none() {
            return Err(());
        }
        let req = SocketReq::from_buf(
            &self.ibuf[0..0xc].try_into().unwrap()
        );
        if req.len as usize > BUF_LEN - 0xc {
            println!("[PPC] request overflow len={:08x}", req.len);
            return Err(());
        }
        match req.cmd {
            Command::HostRead => {
                self.handle_read(client, req);
                return Ok(false);
            },
            Command::HostWrite => {
                self.handle_write(client, req);
                return Ok(false);
            },
            Command::Message => {
                self.handle_message(req);
                return Ok(true);
            },
            Command::Unimpl => {
                return Err(());
            }
        }
    }

    pub fn handle_read(&mut self, client: &mut UnixStream, req: SocketReq) {
        println!("[PPC] read {:x} bytes at {:08x}", req.len, req.addr);
        self.bus.write().unwrap().dma_read(req.addr, 
            &mut self.obuf[0..req.len as usize]);
        client.write(&self.obuf[0..req.len as usize]).unwrap();
    }

    pub fn handle_write(&mut self, client: &mut UnixStream, req: SocketReq) {
        println!("[PPC] write {:x} bytes at {:08x}", req.len, req.addr);
        let data = &self.ibuf[0xc..(0xc + req.len as usize)];
        self.bus.write().unwrap().dma_write(req.addr, data);
        client.write("OK".as_bytes()).unwrap();
    }
    pub fn handle_message(&mut self, req: SocketReq) {
        let mut bus = self.bus.write().unwrap();
        bus.hlwd.ipc.ppc_msg = req.addr;
        bus.hlwd.ipc.state.arm_req = true;
    }

}

///// Top-level loop for this backend.
//impl Backend for PpcBackend {
//    fn run(&mut self) {
//        println!("[PPC] thread started");
//
//        // Try binding to a UNIX socket
//        let res = std::fs::remove_file(IPC_SOCK);
//        match res {
//            Ok(_) => {},
//            Err(e) => {},
//        }
//        let res = UnixListener::bind(IPC_SOCK);
//        let sock = match res {
//            Ok(sock) => Some(sock),
//            Err(e) => {
//                println!("[PPC] Couldn't bind to {},\n{:?}", IPC_SOCK, e);
//                None
//            }
//        };
//
//        // If we successfully bind, run the server
//        if sock.is_some() {
//            self.server_loop(sock.unwrap());
//        }
//
//        println!("[PPC] thread died");
//    }
//}

// NOTE: Temporary 
impl Backend for PpcBackend {
    fn run(&mut self) {
        println!("[PPC] PPC backend thread started");

        'wait_for_broadway: loop { 
            if self.bus.read().unwrap().hlwd.ppc_on {
                println!("[PPC] Broadway came online");
                break 'wait_for_broadway;
            } else {
                thread::sleep(std::time::Duration::from_millis(50));
            }
        }

        'main_loop: loop {
            if self.bus.read().unwrap().hlwd.irq.ppc_irq_output {
                let sts = self.bus.read().unwrap().hlwd.irq.ppc_irq_status.0;
                let en = self.bus.read().unwrap().hlwd.irq.ppc_irq_enable.0;
                println!("[PPC] irq line high, sts={:08x} en={:08x}", sts, en);
            }
            thread::sleep(std::time::Duration::from_millis(500));
        }

    }
}




