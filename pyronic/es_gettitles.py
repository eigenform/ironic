#!/usr/bin/python3

from struct import pack, unpack
from hexdump import hexdump
from pyronic.client import *

ipc = IPCClient()

esfd = ipc.IOSOpen("/dev/es")
print("fd={}".format(esfd))

buf = ipc.alloc_raw(4)
res = ipc.IOSIoctlv(esfd, 0x0e, ":d", buf)
if res < 0:
    print("ES_GetTitleCount() returned {}".format(res))
    ipc.IOSClose(esfd)
    ipc.shutdown()
    exit(0)

num_titles = unpack(">I", buf.read())[0]
print("num_titles={}".format(num_titles))

buf = ipc.alloc_raw(8 * num_titles)
res = ipc.IOSIoctlv(esfd, 0x0f, "i:d", num_titles, buf)
if res < 0:
    print("ES_GetTitles() returned {}".format(res))
    ipc.IOSClose(esfd)
    ipc.shutdown()
    exit(0)

titles = list(unpack(">{}Q".format(num_titles), buf.read()))
print("Found titles:")
for t in titles:
    print("\t{:016x}".format(t))

ipc.IOSClose(esfd)
ipc.shutdown()

