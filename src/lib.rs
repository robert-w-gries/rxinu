#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

#![feature(compiler_builtins_lib)]
extern crate compiler_builtins;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_use]
extern crate arch_x86 as arch;

extern crate multiboot2;
extern crate x86;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    arch::console::init();

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    arch::memory::init(boot_info);

    println!("It did not crash!");

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
