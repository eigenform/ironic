#!/usr/bin/python3

from hexdump import hexdump
from pyronic.client import *

c = IPCClient()
fd = c.IOSOpen("/dev/es")
fd1 = c.IOSOpen("/dev/fs")
print("{} {}".format(fd, fd1))
c.shutdown()

