[![Build Status](https://travis-ci.org/robert-w-gries/rxinu.svg?branch=master)](https://travis-ci.org/robert-w-gries/rxinu)

# rxinu
Rust implementation of [Xinu](https://github.com/xinu-os/xinu)

## Dependencies
  
### Quick Start
```bash
sudo apt-get install binutils clang curl grub nasm qemu xorriso -y
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
# add $HOME/.cargo/bin to PATH env variable
rustup install nightly
rustup default nightly
cargo install xargo
rustup component add rust-src
```

* Note: installing the `gcc-devel` apt repository was required to run `cargo install xargo`

### Required

* cargo
  * Rust package tool
* rustup
  * Rust toolchain manager
  * Used for managing nightly rust
* nasm
  * Least painful assembler available. Supports 32 bit and 64 bit output

### Optional

* binutils
* lld
  * [`lld`](http://lld.llvm.org/) can replace `ld` if desired
  * As of `4.0`, lld does not seem to support the `--nmagic` flag
    * TODO: Get `lld` to link the kernel
* qemu
  * Used in Makefile for testing the kernel
* grub
  * Used to build iso file, which is necessary to test x86_64 kernel with `qemu`
  * `xorriso` package is required dependency for building iso file
    * Note: `xorriso` is only available in `libisoburn` for some distros
  * Note: some distributions, such as OpenSUSE, require `grub2-mkrescue` instead of `grub-mkrescue`

## Compilation

```bash
make # build target=x86_64 by default
make target=i686
```

## Testing

```bash
make run # run target=x86_64 by default
make run target=i686
```

* You can also use real hardware

## Debugging

See [Phillipp Oppermann's blog regarding gdb](https://os.phil-opp.com/set-up-gdb/) for details on how to debug the kernel.

The `rxinu` Makefile has support for debugging built-in already. All that is needed is installing the `gdb` fork and pointing the Makefile's environment variable to the forked `gdb`
