[![Build Status](https://travis-ci.org/robert-w-gries/rxinu.svg?branch=master)](https://travis-ci.org/robert-w-gries/rxinu)

# rxinu
Rust implementation of [Xinu](https://github.com/xinu-os/xinu), based on the [excellent blog written by Philipp Oppermann](https://os.phil-opp.com/)

## Quick Run (Docker)

Clone this repo then run the following:

```bash
sudo apt-get install make
wget -qO- https://get.docker.com/ | sh
# Fedora: sudo systemctl start docker
sudo make docker_build
sudo make docker_run
make run # Inside of docker linux container
```

## Dependencies

### Installation

```bash
sudo apt-get install binutils clang curl grub nasm qemu xorriso -y
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="${PATH}:$HOME/.cargo/bin"
rustup install nightly
rustup default nightly
cargo install xargo
rustup component add rust-src
```

#### Distro Notes

* Fedora
  * `grub2` package was already installed
  * `make run` fails due to wrong GRUB_MKRESCUE
    * Use `make run GRUB_MKRESCUE=grub2-mkrescue` or `export GRUB_MKRESCUE=grub2-mkrescue`
* OpenSUSE
  * installing the `gcc-devel` apt repository was required to run `cargo install xargo`

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

See [Phillipp Oppermann's blog post regarding gdb](https://os.phil-opp.com/set-up-gdb/) for details on how to debug the kernel.

The `rxinu` Makefile has support for debugging built-in already. All that is needed is installing the `gdb` fork and pointing the Makefile's environment variable to the forked `gdb`

## Goals

- [x] Kernel runs rust code
- [x] Simple VGA driver
- [x] Memory Management
  - [x] Setup paging
  - [x] Physical Memory Manager
  - [x] Virtual Memory Allocator
  - [x] Heap Allocator
  - [ ] Switch to [ralloc](https://github.com/redox-os/ralloc)
- [ ] Interrupt handling
  - [x] CPU Exception
  - [ ] IRQ
  - [ ] Syscall
- [ ] Project 1: Synchronous serial driver
  - [x] Serial driver
  - [ ] Keyboard interrupt
  - [ ] `kputc`/`kgetc`/`kungetc`/`kprintf`
- [ ] Higher half kernel
- [ ] Unit tests and integration tests
- [ ] MIPS target
- [ ] Timer
- [ ] Processes
- [ ] Scheduler
  - [ ] Project 2: Multiprocessing and Context Switch
  - [ ] Project 3: Priority and Preemption
- [ ] Project 4: Synchronization and Interprocess Communications
- [ ] Project 5: Sleep and Delta Queues
- [ ] Project 6: File system
- [ ] Project 7: Xinu File Sharing Protocol
- [ ] Project 8: Chat application
- [ ] Permissions for kernel sections
- [ ] Hardware abstraction
- [ ] CI
  - [x] Build all targets
  - [ ] Rustfmt for all crates
  - [ ] Unit tests
  - [ ] Regression tests
  - [ ] Code coverage

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
