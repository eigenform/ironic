from enum import Enum
from struct import pack, unpack

class IOSErr(Enum):
    EINVAL      = -4
    EACCESS     = -102
    ENOENT      = -106

class IPCMsg(object):
    """ A structure representing some PPC-to-ARM IPC message. 
    After this is filled out, the user will obtain the raw bytes and write 
    them to physical memory somewhere (aligned to 32-byte boundaries).
    """
    def __init__(self, cmd, fd=0, args=[0,0,0,0,0]):
        self.cmd = cmd
        self.res = 0
        self.fd = fd
        self.args = args

    def to_buffer(self):
        """ Convert to a big-endian binary representation """
        while len(self.args) < 5: 
            self.args.append(0)
        assert len(self.args) == 5
        return pack(">Lii5L", self.cmd, self.res, self.fd, *self.args)


