//! Backend for handling PowerPC HLE.

use ironic_core::bus::*;
use crate::back::*;

use std::sync::{Arc, RwLock};
use std::os::unix::net::{UnixStream, UnixListener};
use std::net::Shutdown;
use std::io::{Read, Write};
use std::boxed::Box;
use std::convert::TryInto;

extern crate pretty_hex;
use pretty_hex::*;

#[derive(Debug)]
#[repr(u32)]
pub enum Command { Write, Read, }
impl Command {
    fn from_u32(x: u32) -> Option<Self> {
        match x {
            1 => Some(Self::Read),
            2 => Some(Self::Write),
            _ => None,
        }
    }
}

/// A request packet from the socket.
#[repr(C)]
pub struct SocketReq {
    pub cmd: Command,
    pub addr: u32,
    pub buf: [u8; 0x10],
}

/// A response packet to the socket.
#[repr(C)]
pub struct SocketRep {
}



pub const IPC_SOCK: &str = "/tmp/ironic.sock";

pub struct PpcBackend {
    pub bus: Arc<RwLock<Bus>>,
    pub ibuf: [u8; 0x10000],
    pub obuf: [u8; 0x10000],
}
impl PpcBackend {
    pub fn new(bus: Arc<RwLock<Bus>>) -> Self {
        PpcBackend {
            bus,
            ibuf: [0; 0x10000],
            obuf: [0; 0x10000],
        }
    }

    pub fn server_loop(&mut self, sock: UnixListener) {
        loop {

            // Block for an incoming connection
            println!("[PPC] server listening on {}", IPC_SOCK);
            let res = sock.accept();
            let mut client = match res {
                Ok((stream, _)) => {
                    println!("[PPC] client connected");
                    stream
                }
                Err(e) => {
                    println!("[PPC] error accepting client {:?}", e);
                    break;
                }
            };

            // Handle this client until they hang up
            self.client_session_loop(&mut client);
        }
    }

    pub fn client_session_loop(&mut self, client: &mut UnixStream) {
        loop {
            // Block for a message from the client; if we recieve an empty 
            // message (of length zero), assume that the client hung up
            let res = client.read(&mut self.ibuf);
            let len = match res {
                Ok(len) => {
                    if len == 0 { println!("[PPC] client HUP"); break; }
                    len
                },
                Err(e) => {
                    println!("[PPC] IO error {:?}", e);
                    break;
                }
            };

            // Handle a command from the client
            let cmd = Command::from_u32(
                u32::from_le_bytes(self.ibuf[0..4].try_into().unwrap())
            );
            if cmd.is_none() { 
                println!("[PPC] invalid command {:08x} from client", 
                    u32::from_le_bytes(self.ibuf[0..4].try_into().unwrap()));
                client.shutdown(Shutdown::Both).unwrap();
                println!("[PPC] server terminated client connection");
                break;
            } else {
                match cmd.unwrap() {
                    Command::Read => self.handle_read(client),
                    Command::Write => self.handle_write(client),
                }
            }
        }
    }

    pub fn handle_read(&mut self, client: &mut UnixStream) {
        let paddr = u32::from_le_bytes(self.ibuf[0x4..0x8]
            .try_into().unwrap());
        let len = u32::from_le_bytes(self.ibuf[0x8..0xc]
            .try_into().unwrap());

        println!("[PPC] read {:x} bytes at {:08x}", len, paddr);
        self.bus.write().unwrap().dma_read(paddr, 
            &mut self.obuf[0..len as usize]);
        client.write(&self.obuf[0..len as usize]).unwrap();
    }

    pub fn handle_write(&mut self, client: &mut UnixStream) {
        let paddr = u32::from_le_bytes(self.ibuf[0x4..0x8]
            .try_into().unwrap());
        let len = u32::from_le_bytes(self.ibuf[0x8..0xc]
            .try_into().unwrap());
        let data = &self.ibuf[0xc..(0xc + len as usize)];

        println!("[PPC] write {:x} bytes at {:08x}", len, paddr);
        println!("{:?}", data.hex_dump());
        self.bus.write().unwrap().dma_write(paddr, data);
        client.write("OK".as_bytes()).unwrap();
    }

}

impl Backend for PpcBackend {
    fn run(&mut self) {
        println!("[PPC] thread started");

        // Try binding to a UNIX socket
        std::fs::remove_file(IPC_SOCK).unwrap();
        let res = UnixListener::bind(IPC_SOCK);
        let sock = match res {
            Ok(sock) => Some(sock),
            Err(e) => {
                println!("[PPC] Couldn't bind to {},\n{:?}", IPC_SOCK, e);
                None
            }
        };

        // If we successfully bind, run the server
        if sock.is_some() {
            self.server_loop(sock.unwrap());
        }


        println!("[PPC] thread died");
    }
}
