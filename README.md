# rxinu
Rust implementation of Xinu

## Dependencies

### Required
```
rustup - for rust nightly compiler
cargo - rust build tool
ld
make
```

### Optional
```
clang - for easy cross-compilation
qemu - for testing the kernel
grub - if you want to build iso
```

## Compilation

```
make
```

* The makefile supports clang right now but you can change it to gcc if you have a cross-compiler toolchain.
* x86 32-bit is the only supported architecture right now

## Testing

* You can use real hardware
* I recommend using `qemu-system-x86_64`
* 64 bit version of kernel would need to be booted through cdrom.
  * `make iso` creates the disk image
  * `qemu-system-x86_64 -cdrom build/kernel*.iso`
* 32 bit is a bit easier
  * `qemu-system-x86_64 -kernel build/kernel*.bin`
