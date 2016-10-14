#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

extern crate rlibc;

mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::print_something()
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
        loop {}
}

