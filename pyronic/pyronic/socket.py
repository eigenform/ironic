import socket
from struct import pack, unpack

class IronicSocket(object):
    """ Representing some connection to the PPC HLE server. """
    IRONIC_READ    = 1
    IRONIC_WRITE   = 2
    IRONIC_MSG     = 3
    IRONIC_ACK     = 4

    def __init__(self, filename="/tmp/ironic.sock"):
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.connect(filename)

    def close(self): 
        self.socket.close()

    def send_guestread(self, paddr, size):
        """ Send a guest read command to the server """
        msg = bytearray()
        msg += pack("<LLL", self.IRONIC_READ, paddr, size)
        self.socket.send(msg)
        resp = self.socket.recv(size)
        assert len(resp) == size
        return resp

    def send_guestwrite(self, paddr, buf):
        """ Send a guest write command to the server """
        msg = bytearray()
        msg += pack("<LLL", self.IRONIC_WRITE, paddr, len(buf))
        msg += buf
        self.socket.send(msg)
        resp = self.socket.recv(2)
        assert resp.decode('utf-8') == "OK"

    def send_ipcmsg(self, ptr):
        """ Send an IPC message command to the server """
        msg = bytearray()
        msg += pack("<LLL", self.IRONIC_MSG, ptr, 4)
        self.socket.send(msg)
        resp = self.socket.recv(2)
        assert resp.decode('utf-8') == "OK"

    def recv_ipcmsg(self):
        """ Wait for the server to respond with a pointer to an IPC message """
        res_buf = self.socket.recv(4)
        res_ptr = unpack("<L", res_buf)[0]
        return res_ptr

    def send_ack(self):
        """ Send an ACK command to the server """
        msg = bytearray()
        msg += pack("<LLL", self.IRONIC_ACK, 0, 0)
        self.socket.send(msg)
        resp = self.socket.recv(2)
        assert resp.decode('utf-8') == "OK"



