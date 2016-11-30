#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };

    let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Elf-sections tag required");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
            .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
            .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel_start = {}, kernel_end = {}", kernel_start, kernel_end);
    println!("multiboot_start = {}, multiboot_end = {}", multiboot_start, multiboot_end);

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
        loop {}
}
