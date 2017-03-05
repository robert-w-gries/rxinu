#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

#![feature(compiler_builtins_lib)]
extern crate compiler_builtins;

extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main() {
    vga_buffer::clear_screen();
    for i in 0..100 {
        println!("Hello World{} {}", "!", i);
    }
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
