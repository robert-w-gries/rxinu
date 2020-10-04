#![feature(
    abi_x86_interrupt,
    allocator_api,
    alloc_error_handler,
    const_fn,
    const_in_array_repeat_expressions,
    const_mut_refs,
    global_asm,
    lang_items,
    llvm_asm,
    naked_functions,
    ptr_internals
)]
#![no_std]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

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

#[alloc_error_handler]
pub fn rust_oom(info: core::alloc::Layout) -> ! {
    panic!("{:?}", info);
}