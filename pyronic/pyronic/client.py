from struct import pack, unpack

from pyronic.socket import *
from pyronic.ios import *
from hexdump import hexdump

class MemHandle(object):
    """ A handle to some piece of guest memory """
    def __init__(self, sock, paddr, size):
        self.__sock = sock
        self.paddr = paddr
        self.size = size
    def read(self, off=0, size=None):
        return self.__sock.send_guestread(self.paddr, self.size)
    def write(self, buf, off=0):
        assert len(buf) <= self.size
        self.__sock.send_guestwrite(self.paddr, buf)
        self.data_size = len(buf)


class PPCMemory(object):
    """ The fabled "bump allocator," notoriously difficult to implement """
    def __init__(self, sock, head=0x01000000, tail=0x01780000):
        self.__sock = sock
        self.head = head
        self.tail = tail
        self.cursor = head
        self.block_len = 0x40

    def free(self, paddr): raise ValueError("nah")
    def alloc(self, rsize):
        # Round up the size of an allocation (to the block length)
        if (rsize % self.block_len) == 0: 
            size = rsize
        else: 
            size = (rsize & ~(self.block_len-1)) + self.block_len

        if (size > (self.tail - self.head)):
            raise ValueError("Allocation too large for bounds")

        # If we run out of room, just go back to the beginning
        if ((self.cursor + size) >= self.tail):
            self.cursor = self.head

        addr = self.cursor
        self.cursor += size
        return MemHandle(self.__sock, addr, rsize)


class IPCClient(object):
    """ Client used to interact with the PPC HLE server """
    IPC_OPEN    = 1
    IPC_CLOSE   = 2
    IPC_READ    = 3
    IPC_WRITE   = 4
    IPC_SEEK    = 5
    IPC_IOCTL   = 6
    IPC_IOCTLV  = 7

    def __init__(self, filename="/tmp/ironic.sock"):
        self.sock = IronicSocket()
        self.mem = PPCMemory(self.sock)

    def alloc_buf(self, buf, paddr=None):
        """ Return a handle to a piece of memory initialized with 'buf' """
        if paddr == None:
            hdl = self.mem.alloc(len(buf))
        else:
            hdl = MemHandle(self.sock, paddr, len(buf))
        hdl.write(buf)
        return hdl

    def alloc_raw(self, size, paddr=None):
        """ Return a handle to a piece of uninitialized memory """
        if paddr == None:
            return self.mem.alloc(size)
        else:
            return MemHandle(self.sock, paddr, size)

    def shutdown(self):
        """ Close our connection to the server """
        self.sock.close()

    def guest_read(self, paddr, size): 
        """ Read some data from guest physical memory """
        return self.sock.send_guestread(paddr, size)

    def guest_write(self, paddr, buf):
        """ Write some data to guest physical memory """
        self.sock.send_guestwrite(paddr, buf)

    def guest_ipc(self, ipcmsg: IPCMsg):
        """ Send an IPC request, block, return a handle to the response """
        buf = self.alloc_buf(ipcmsg.to_buffer())
        self.sock.send_ipcmsg(buf.paddr)
        response_ptr = self.sock.recv_ipcmsg()
        return MemHandle(self.sock, response_ptr, 0x20)

    def IOSOpen(self, inpath, mode=0):
        buf = self.alloc_buf(inpath.encode('utf-8') + b'\x00')
        msg = IPCMsg(self.IPC_OPEN, fd=0, args=[buf.paddr, mode])
        res = self.guest_ipc(msg)
        return unpack(">i", res.read()[4:8])[0]

    def IOSClose(self, fd):
        msg = IPCMsg(self.IPC_CLOSE, fd=fd)
        res = self.guest_ipc(msg)
        return unpack(">i", res.read()[4:8])[0]

    def __ioctlv_parse(self, fmt, args):
        """ Parse some ioctlv arguments into a set of memory handles """
        handles = []

        for (c, v) in zip(fmt, args):
            print("{} {}".format(c, v))
            if c == 'b': 
                handles.append(self.alloc_buf(pack(">B", v & 0xff)))
            elif c == 'h': 
                handles.append(self.alloc_buf(pack(">H", v & 0xffff)))
            elif c == 'i': 
                handles.append(self.alloc_buf(pack(">I", v & 0xffffffff)))
            elif c == 'q': 
                handles.append(self.alloc_buf(pack(">Q", v & 0xffffffffffffffff)))
            elif c == 'd': 
                handles.append(v)
            else:
                raise ValueError("Invalid ioctlv format string")
        return handles

    def IOSIoctlv(self, fd, cmd, fmt, *args):
        iargs = []
        oargs = []
        arglist = list(args)

        if ':' not in fmt:
            ifmt = list(fmt)
            ofmt = ""
        else:
            ifmt = list(fmt.split(":")[0])
            ofmt = list(fmt.split(":")[1])

        for c in ifmt: iargs.append(arglist.pop(0))
        for c in ofmt: oargs.append(arglist.pop(0))

        print(ifmt, ofmt)
        print(iargs, oargs)

        ibufs = self.__ioctlv_parse(ifmt, iargs)
        obufs = self.__ioctlv_parse(ofmt, oargs)
        print(ibufs, obufs)

        ioctlvbuf = bytearray()
        for handle in ibufs:
            ioctlvbuf += pack(">LL", handle.paddr, handle.size)
        for handle in obufs:
            ioctlvbuf += pack(">LL", handle.paddr, handle.size)
        buf = self.alloc_buf(ioctlvbuf)
        print(hexdump(buf.read()))

        msg = IPCMsg(self.IPC_IOCTLV, fd=fd, 
                args=[cmd, len(ibufs), len(obufs), buf.paddr])
        print(hexdump(msg.to_buffer()))
        res = self.guest_ipc(msg)
        return unpack(">i", res.read()[4:8])[0]


