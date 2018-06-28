#![feature(
    abi_x86_interrupt, alloc, allocator_api, asm, const_fn, const_max_value,
    const_unique_new, const_atomic_usize_new, const_fn, global_asm, lang_items, naked_functions,
    ptr_internals, unique
)]
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

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

#[macro_use]
pub mod device;

#[macro_use]
pub mod arch;

pub mod sync;
pub mod syscall;
pub mod task;

/// Main initialization process for rxinu
pub extern "C" fn rxinu_main() {
    device::console::clear_screen();
    kprintln!("In main process!\n");
    serial_println!("In main process!\n");
}

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

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
