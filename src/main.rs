#![allow(unused_must_use)]
#![feature(abi_x86_interrupt)]
#![feature(alloc, allocator_api, global_allocator)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_max_value)]
#![feature(const_unique_new, const_atomic_usize_new)]
#![feature(const_fn)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![feature(ptr_internals)]
#![feature(unique)]
#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate once;

extern crate bit_field;
extern crate linked_list_allocator;
extern crate os_bootinfo;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86_64;

#[macro_use]
pub mod arch;
pub mod device;
pub mod sync;
pub mod syscall;
pub mod task;

use alloc::String;
use arch::memory::heap::{HEAP_SIZE, HEAP_START};
use core::panic::PanicInfo;

#[no_mangle]
/// Entry point for rust code
pub extern "C" fn _start(boot_info_address: usize) -> ! {
    arch::init(boot_info_address);

    unsafe {
        task::scheduler::init();
        arch::interrupts::clear_mask();
    }


    kprintln!("\nHEAP START = 0x{:x}", HEAP_START);
    kprintln!("HEAP END = 0x{:x}\n", HEAP_START + HEAP_SIZE);

    syscall::create(String::from("rxinu_main"), 0, rxinu_main);

    loop {
        #[cfg(feature = "serial")]
        {
            use device::uart_16550 as uart;
            uart::read(1024);
        }

        #[cfg(feature = "vga")]
        {
            use device::keyboard::ps2 as kbd;
            kbd::read(1024);
        }

        // Save cycles by pausing until next interrupt
        arch::interrupts::pause();
    }
}

/// Main initialization process for rxinu
pub extern "C" fn rxinu_main() {
    arch::console::clear_screen();
    kprintln!("In main process!\n");

    syscall::create(String::from("process a"), 25, process_a);
    syscall::create(String::from("process b"), 25, process_b).unwrap();

    let pid_kill = syscall::create(String::from("kill_process"), 40, kill_process).unwrap();

    syscall::create(String::from("test_process"), 0, test_process);
    syscall::kill(pid_kill);
}

pub extern "C" fn test_process() {
    kprintln!("\nIn test process!");
    kprintln!("\nYou can now type...\n");
}

pub extern "C" fn process_a() {
    kprintln!("\nIn process_a!");
    loop {
        arch::interrupts::pause();
    }
}

pub extern "C" fn process_b() {
    kprintln!("\nIn process_b!");
    loop {
        arch::interrupts::pause();
    }
}

pub extern "C" fn kill_process() {
    kprint!("\nIn kill_process");
    loop {
        kprint!(".");
        unsafe {
            arch::interrupts::halt();
        }
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_impl"]
#[no_mangle]
pub extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    kprintln!("\n\nPANIC");

    if let Some(location) = info.location() {
        kprint!("in {} at line {}", location.file(), location.line());
    }

    if let Some(message) = info.message() {
        kprintln!("\n    {:?}", message);
    }

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom() -> ! {
    panic!("Out of memory");
}

use arch::memory::heap::HeapAllocator;

#[global_allocator]
static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();
