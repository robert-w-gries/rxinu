[![Build Status](https://travis-ci.org/robert-w-gries/rxinu.svg?branch=master)](https://travis-ci.org/robert-w-gries/rxinu)

# rxinu
Rust implementation of [Xinu](https://github.com/xinu-os/xinu)

## Dependencies
  
### Quick Start
```bash
sudo apt-get install binutils clang curl grub qemu xorriso -y
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
# add $HOME/.cargo/bin to PATH env variable
rustup install nightly
rustup default nightly
cargo install xargo
rustup component add rust-src
```

### Required

* cargo
  * Rust package tool
* rustup
  * Rust toolchain manager
  * Used for managing nightly rust


### Optional

* binutils
  * [`lld`](http://lld.llvm.org/) can replace `ld` if desired
* clang
  * Recommended for easy cross-compilation
  * Required version >= 3.5
* qemu
  * Used in Makefile for testing the kernel
* grub
  * Used to build iso file, which is necessary to test x86_64 kernel with `qemu`
  * `xorriso` package is required dependency for building iso file

## Compilation

```bash
make # build 32-bit
make target=x86_64
```

* The makefile uses clang right now but you can change it to gcc if you have a cross-compiler toolchain.

## Testing

```bash
make run # run 32-bit
make run target=x86_64
```

* You can also use real hardware
