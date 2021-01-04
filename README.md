# ironic
It's an emulator.

If there end up being features specific to Linux platforms, I am not planning 
on Windows/Mac compatibility. It's also probably very slow, sorry.

## Quick Roadmap
- [x] Execution through the boot ROM
- [x] Execution through first-stage bootloader
- [x] Execution through second-stage bootloader
- [x] Execution in the kernel
- [ ] Guest debugging functionality (either via GDB or UI)
- [ ] Go fast (performance optimizations, i.e. a JIT backend, etc)
- [ ] Other related tools?

## Contributing
I am not interested in accepting contributions to this project and I will 
probably work on it alone. When I get around to deciding on a license, it
will probably be as permissive as possible.

## Building
I use Nightly by default, so you may have to do something like this:
```
$ git clone https://github.com/hosaka-corp/ironic && cd ironic/
...
$ rustup toolchain install nightly
$ rustup override set nightly
```

