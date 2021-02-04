#!/usr/bin/python3
""" kernel_boot.py
A bootloader for foreign (non-IOS) kernels, via IOSv58.
For more details, see the repository at https://github.com/eigenform/mana
"""

from sys import argv
from struct import pack, unpack
from hexdump import hexdump
from pyronic.client import *
from pyronic.ios import *

BAD_POINTER     = 0x201125b0
NUM_ENTRIES     = 0x00000000

if len(argv) < 3:
    print("usage: {} <stub binary> <kernel binary>".format(argv[0]))
    exit(0)

with open(argv[1], "rb") as f:
    stub_data = f.read()
with open(argv[2], "rb") as f:
    kernel_data = f.read()


ipc = IPCClient()

# Write the stub loader to 0x00010000
stub_target = ipc.alloc_buf(stub_data, paddr=0x00010000)

# Write the kernel binary to 0x10100000
kern_target = ipc.alloc_buf(kernel_data, paddr=0x10100000)

# Jump to the stub loader by invoking the ES_GetTitles() ioctlv.
# We aren't using ipc.IOSIoctlv() here because we don't expect IOS to continue
# running after the ioctl is send (and so, there's no reason waiting for a 
# response from the IOS kernel).
#
# Instead, the PPC HLE server implements a special "IPC_MSG_NORETURN" command 
# which indicates that ironic should not wait for a response.

fd = ipc.IOSOpen("/dev/es")

bad_buffer = ipc.alloc_raw(0, paddr=BAD_POINTER)
num_entries = ipc.alloc_raw(4)
num_entries.write(pack(">L", 0x00000000))

ioctlvbuf = ipc.alloc_buf(pack(">LLLL", num_entries.paddr, 
    num_entries.size, bad_buffer.paddr, bad_buffer.size))
msg = IPCMsg(IPCClient.IPC_IOCTLV, fd=fd, args=[0x0f, 1, 1, ioctlvbuf.paddr])
ipc_buf = ipc.alloc_buf(msg.to_buffer())
ipc.sock.send_ipcmsg_noret(ipc_buf.paddr)

ipc.shutdown()

