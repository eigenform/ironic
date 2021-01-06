# ironic
Research emulator for the ARM9 core in the Nintendo Wii.

Credit is due to the following projects and resources:
- [Team Twiizers' fork of Skyeye](https://github.com/marcan/skyeye-starlet)
- [MINI](https://github.com/fail0verflow/mini)
- [Wiibrew](https://wiibrew.org)
- [archshift/llama](https://github.com/archshift/llama)
- [MerryMage/dynarmic](https://github.com/MerryMage/dynarmic)
- All of the folks who still work on Wii/GC homebrew

## Quick Roadmap
- [x] Execution through the boot ROM
- [x] Execution through first-stage bootloader
- [x] Execution through second-stage bootloader
- [x] Execution in the kernel
- [ ] Broadway/PowerPC-world HLE 
- [ ] Emulated SDHC (SD card) support?
- [ ] Emulated USB support?
- [ ] Emulated WLAN functionality?
- [ ] Guest debugging functionality (perhaps via GDB, or some UI?)
- [ ] Go fast (performance optimizations, i.e. a JIT backend, etc)
- [ ] Tools for fuzzing guest code
- [ ] Other related tools?

If there end up being features specific to Linux platforms, I am not planning 
on Windows/Mac compatibility. It's also probably very slow, sorry.

## Contributing
I am not interested in accepting contributions to this project and I will 
probably work on it alone.  When I get around to deciding on a license, it will 
probably be as permissive as possible.

## Building
I use Nightly by default, so you may have to do something like this:
```
$ git clone https://github.com/hosaka-corp/ironic && cd ironic/
...
$ rustup toolchain install nightly
$ rustup override set nightly
$ cargo build --release
```

## Usage
In order to boot, `ironic` expects the following files to live in the project 
directory:

- `boot0.bin` - Your copy of the Wii boot ROM
- `nand.bin` - The NAND flash data from your console
- `otp.bin` - Your associated OTP/fused memory dump
- `seeprom.bin` - Your associated SEEPROM memory dump

You can run the emulator with the interpreter backend like this:
```
$ cargo run --release --bin ironic-tui interp
```
