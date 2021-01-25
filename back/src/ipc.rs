//! Types for emulating inter-processor communication.

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpenArg {
    pub name: u32, //ptr
    pub mode: u32,
    pub pad2: u32,
    pub pad3: u32,
    pub pad4: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CloseArg {
    pub pad0: u32, 
    pub pad1: u32,
    pub pad2: u32,
    pub pad3: u32,
    pub pad4: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReadArg {
    pub addr: u32, 
    pub len: u32,
    pub pad2: u32,
    pub pad3: u32,
    pub pad4: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteArg {
    pub addr: u32, 
    pub len: u32,
    pub pad2: u32,
    pub pad3: u32,
    pub pad4: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeekArg {
    pub offset: u32, 
    pub whence: u32,
    pub pad2: u32,
    pub pad3: u32,
    pub pad4: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IoctlArg {
    pub cmd: u32, 
    pub input_addr: u32,
    pub input_len: u32,
    pub output_addr: u32,
    pub output_len: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IoctlvArg {
    pub cmd: u32, 
    pub input: u32,
    pub output: u32,
    pub argv: u32, // ptr
    pub pad4: u32,
}

#[repr(C)]
pub struct IoctlvArgEntry {
    pub data: u32, // ptr
    pub len: u32,
}


/// Commands used in IPC requests to IOS58.
#[repr(u32)]
pub enum IosCmd {
    Open    = 0x0000_0001,
    Close   = 0x0000_0002,
    Read    = 0x0000_0003,
    Write   = 0x0000_0004,
    Seek    = 0x0000_0005,
    Ioctl   = 0x0000_0006,
    Ioctlv  = 0x0000_0007,
}

/// Command arguments used in IPC requests to IOS58.
#[repr(C)]
pub union IosArg {
    open: OpenArg,
    close: CloseArg,
    read: ReadArg,
    write: WriteArg,
    seek: SeekArg,
    ioctl: IoctlArg,
    ioctlv: IoctlvArg,
}

/// An IOS58 IPC request.
#[repr(C)]
pub struct IosReq {
    pub cmd: IosCmd,
    pub res: u32,
    pub fd: u32,
    pub arg: IosArg,
}
