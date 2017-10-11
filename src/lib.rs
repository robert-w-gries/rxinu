#![feature(alloc)]
#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]
#![feature(compiler_builtins_lib)]
extern crate compiler_builtins;

#[cfg(target_arch = "x86")]
#[macro_use]
extern crate arch_i686 as arch;

#[cfg(target_arch = "x86_64")]
#[macro_use]
extern crate arch_x86_64 as arch;

#[macro_use]
extern crate alloc;
extern crate multiboot2;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    arch::device::init();
    arch::console::init();

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };

    let mut memory_controller = arch::memory::init(boot_info);

    #[cfg(target_arch = "x86_64")]
    arch::interrupts::init(&mut memory_controller);

    println!("\nIt did not crash!");

    loop {}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
