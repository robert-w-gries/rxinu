[![Build Status](https://travis-ci.org/robert-w-gries/rxinu.svg?branch=master)](https://travis-ci.org/robert-w-gries/rxinu)

# rxinu
Rust implementation of [Xinu](https://github.com/xinu-os/xinu), based on the [excellent blog written by Philipp Oppermann](https://os.phil-opp.com/)

```bash
sudo apt-get install gcc qemu -y
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="${PATH}:$HOME/.cargo/bin"
rustup default nightly
rustup component add rust-src
rustup component add llvm-tools-preview
cargo install bootimage
cargo run # run kernel using qemu
```

## Running

### Docker

```
docker build -t rxinu .
docker run -v ${PWD}:/rxinu -i -t rxinu
```

### QEMU headless mode

There are multiple methods of running QEMU in headless mode. To use the `curses` option, use the following:

```
cargo run -- -curses
```

### Other methods

[See here](https://os.phil-opp.com/minimal-rust-kernel/#virtualbox) for instructions on running the kernel on VirtualBox or on real hardware.

## Debugging

See [Phillipp Oppermann's blog post regarding gdb](https://os.phil-opp.com/set-up-gdb/) to build a gdb binary that can debug x86_64 kernels.

```bash
qemu-system-x86_64 -drive format=raw,file=bootimage.bin -d int -s -S &
rust-gdb target/x86_64-rxinu/debug/rxinu -ex "target remote :1234"
```

## Features

* Architectures
  * x86_64
* MMU
  * Paging
  * Heap Allocation
* Interrupt Handling
  * Exceptions
  * IRQ
* Scheduling
  * Cooperative Scheduler
  * Preemptive Scheduler
* Device Drivers
  * PIC
  * PIT
  * PS/2 Keyboard
  * Serial
  * VGA

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
