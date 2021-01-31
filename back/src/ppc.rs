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
pub enum Command { HostWrite, HostRead, Message, Ack, Unimpl }
impl Command {
    fn from_u32(x: u32) -> Self {
        match x {
            1 => Self::HostRead,
            2 => Self::HostWrite,
            3 => Self::Message,
            4 => Self::Ack,
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

    fn recv(&mut self, client: &mut UnixStream) -> Option<usize> {
        let res = client.read(&mut self.ibuf);
        match res {
            Ok(len) => if len == 0 { None } else { Some(len) },
            Err(_) => None
        }
    }
}


impl PpcBackend {

    /// Handle clients connected to the socket.
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

            'handle_client: loop {
                println!("[PPC] waiting for command ...");

                let res = self.wait_for_request(&mut client);
                let req = if res.is_none() { break; } else { res.unwrap() };
                match req.cmd {
                    Command::Ack => self.handle_ack(req),
                    Command::HostRead => self.handle_read(&mut client, req),
                    Command::HostWrite => self.handle_write(&mut client, req),
                    Command::Message => {
                        self.handle_message(&mut client, req);
                        if self.wait_for_ack() {
                            let armmsg = self.bus.read().unwrap().hlwd.ipc.arm_msg;
                            client.write(&u32::to_le_bytes(armmsg)).unwrap();
                        }
                    },
                    Command::Unimpl => break,
                }
            }
            client.shutdown(Shutdown::Both).unwrap();
        }
    }

    /// Block until we get a response from ARM-world.
    fn wait_for_ack(&mut self) -> bool {
        println!("[PPC] waiting for ACK ...");
        loop {
            if self.bus.read().unwrap().hlwd.irq.ppc_irq_output {
                println!("[PPC] got irq");
                let mut res = false;
                let mut bus = self.bus.write().unwrap();

                if bus.hlwd.ipc.state.ppc_ack {
                    bus.hlwd.ipc.state.ppc_ack = false;
                    println!("[PPC] got extra ACK");
                }
                if bus.hlwd.ipc.state.ppc_req {
                    let armmsg = bus.hlwd.ipc.arm_msg;
                    println!("[PPC] Got message from ARM {:08x}", armmsg);
                    bus.hlwd.ipc.state.ppc_req = false;
                    bus.hlwd.ipc.state.arm_ack = true;
                    res = true;
                }

                println!("[PPC] cleared irq");
                bus.hlwd.irq.ppc_irq_status.unset(HollywoodIrq::PpcIpc);
                return res;
            } else {
                thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }

    /// Block until we receive some command message from a client.
    fn wait_for_request(&mut self, client: &mut UnixStream) -> Option<SocketReq> {
        let res = self.recv(client);
        if res.is_none() {
            return None;
        }
        let req = SocketReq::from_buf(
            &self.ibuf[0..0xc].try_into().unwrap()
        );
        if req.len as usize > BUF_LEN - 0xc {
            return None;
        }
        Some(req)
    }

    /// Read from physical memory.
    pub fn handle_read(&mut self, client: &mut UnixStream, req: SocketReq) {
        println!("[PPC] read {:x} bytes at {:08x}", req.len, req.addr);
        self.bus.write().unwrap().dma_read(req.addr, 
            &mut self.obuf[0..req.len as usize]);
        client.write(&self.obuf[0..req.len as usize]).unwrap();
    }

    /// Write to physical memory.
    pub fn handle_write(&mut self, client: &mut UnixStream, req: SocketReq) {
        println!("[PPC] write {:x} bytes at {:08x}", req.len, req.addr);
        let data = &self.ibuf[0xc..(0xc + req.len as usize)];
        self.bus.write().unwrap().dma_write(req.addr, data);
        client.write("OK".as_bytes()).unwrap();
    }

    /// Tell ARM-world that an IPC request is ready at the location indicated
    /// by the pointer in PPC_MSG.
    pub fn handle_message(&mut self, client: &mut UnixStream, req: SocketReq) {
        let mut bus = self.bus.write().unwrap();
        bus.hlwd.ipc.ppc_msg = req.addr;
        bus.hlwd.ipc.state.arm_req = true;
        bus.hlwd.ipc.state.arm_ack = true;
        client.write("OK".as_bytes()).unwrap();
    }

    pub fn handle_ack(&mut self, req: SocketReq) {
        let mut bus = self.bus.write().unwrap();
        let ppc_ctrl = bus.hlwd.ipc.read_handler(4) & 0x3c;
        bus.hlwd.ipc.write_handler(4, ppc_ctrl | 0x8);
    }

}


impl Backend for PpcBackend {
    fn run(&mut self) {
        println!("[PPC] PPC backend thread started");
        self.bus.write().unwrap().hlwd.ipc.state.ppc_ctrl_write(0x36);

        'wait_for_broadway: loop { 
            if self.bus.read().unwrap().hlwd.ppc_on {
                println!("[PPC] Broadway came online");
                break 'wait_for_broadway;
            } else {
                thread::sleep(std::time::Duration::from_millis(500));
            }
        }

        // Block until we get an IRQ with an ACK/MSG
        self.wait_for_ack();

        // Send an extra ACK
        self.bus.write().unwrap().hlwd.ipc.state.arm_ack = true;
        thread::sleep(std::time::Duration::from_millis(100));

        // Try binding to the socket
        let res = std::fs::remove_file(IPC_SOCK);
        match res {
            Ok(_) => {},
            Err(e) => {},
        }
        let res = UnixListener::bind(IPC_SOCK);
        let sock = match res {
            Ok(sock) => Some(sock),
            Err(e) => {
                println!("[PPC] Couldn't bind to {},\n{:?}", IPC_SOCK, e);
                None
            }
        };

        // If we successfully bind, run the server until it exits
        if sock.is_some() {
            self.server_loop(sock.unwrap());
        }
        println!("[PPC] thread exited");
    }
}


