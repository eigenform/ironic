from struct import pack, unpack

from pyronic.socket import *
from pyronic.ios import *

class PPCMemory(object):
    """ Simple book-keeping for guest memory usage """
    def __init__(self, head=0x01000000, tail=0x01780000):
        self.head = head
        self.tail = tail
        self.cursor = head
        self.block_len = 0x40
        self.allocations = []

    def alloc(self, rsize):
        assert rsize != 0
        if (rsize % self.block_len) == 0: 
            size = rsize
        else: 
            size = (rsize & ~(self.block_len-1)) + self.block_len
        assert size < self.tail - self.head
        if ((self.cursor + size) >= self.tail):
            self.cursor = self.head
            self.allocations = []
        addr = self.cursor
        self.cursor += size
        self.allocations.append((addr, size))
        return addr


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
        self.mem = PPCMemory()

    def shutdown(self):
        """ Kill our connection to the server """
        self.sock.close()

    def read(self, paddr, size): 
        """ Read some data from guest physical memory """
        return self.sock.send_guestread(paddr, size)

    def write(self, paddr, buf):
        """ Write some data to guest physical memory """
        self.sock.send_guestwrite(paddr, buf)

    def ipc_request(self, ipcmsg: IPCMsg):
        """ Send some IOS IPC request to ARM-world and wait for the response.
        Returns the entire response buffer (big-endian binary representation).
        """
        req_buf = ipcmsg.to_buffer()
        req_ptr = self.mem.alloc(len(req_buf))
        self.write(req_ptr, req_buf)

        self.sock.send_ipcmsg(req_ptr)
        res_ptr = self.sock.recv_ipcmsg()
        res_buf = self.read(res_ptr, len(req_buf))
        return res_buf

    def IOSOpen(self, inpath, mode=0):
        path_buf = inpath.encode('utf-8') + b'\x00'
        path_buf_ptr = self.mem.alloc(len(path_buf))
        self.write(path_buf_ptr, path_buf)

        msg = IPCMsg(self.IPC_OPEN, fd=0, args=[path_buf_ptr, mode])
        res_buf = self.ipc_request(msg)
        return unpack(">i", res_buf[4:8])[0]

    def IOSClose(self, fd):
        msg = IPCMsg(self.IPC_CLOSE, fd=fd)
        res_buf = self.ipc_request(msg)
        return unpack(">i", res_buf[4:8])[0]

    def IOSRead(self, fd, size, dst=None):
        if dst == None:
            data_buf_ptr = self.mem.alloc(size)
        else:
            data_buf_ptr = dst

        msg = IPCMsg(self.IPC_READ, fd=fd, args=[data_buf_ptr, size])
        res_buf = self.ipc_request(msg)
        return unpack(">i", res_buf[4:8])[0]

    def IOSWrite(self, fd, dst, size=None):
        assert (type(dst) == int) or (type(dst) == bytes) or (type(dst) == bytearray)
        if type(dst) == int:
            assert size != None
            data_ptr = dst
            data_len = size
        elif (type(dst) == bytes) or (type(dst) == bytearray):
            data_ptr = self.mem.alloc(len(dst))
            data_len = len(dst)
            self.write(data_ptr, dst)

        msg = IPCMsg(self.IPC_WRITE, fd=fd, args=[data_ptr, data_len])
        res_buf = self.ipc_request(msg)
        return unpack(">i", res_buf[4:8])[0]

    def IOSSeek(self, fd, where, whence):
        msg = IPCMsg(self.IPC_SEEK, fd=fd, args=[where, whence])
        res_buf = self.ipc_request(msg)
        return unpack(">i", res_buf[4:8])[0]

    #def IOSIoctlv(self, fd, cmd, fmt, *args):



