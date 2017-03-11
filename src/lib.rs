#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

#![feature(compiler_builtins_lib)]
extern crate compiler_builtins;

#[macro_use]
extern crate bitflags;

extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86;

#[macro_use]
mod vga_buffer;

pub mod memory;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    let kernel_start = elf_sections_tag.sections()
        .map(|s| s.addr)
        .min()
        .unwrap();
    let kernel_end = elf_sections_tag.sections()
        .map(|s| s.addr + s.size)
        .max()
        .unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel_start = 0x{:x}, kernel_end = 0x{:x}",
             kernel_start,
             kernel_end);
    println!("multiboot_start = 0x{:x}, multiboot_end = 0x{:x}",
             multiboot_start,
             multiboot_end);

    enable_nxe_bit();
    enable_write_protect_bit();

    let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_start as usize,
                                                              kernel_end as usize,
                                                              multiboot_start,
                                                              multiboot_end,
                                                              memory_map_tag.memory_areas());

    memory::remap_the_kernel(&mut frame_allocator, boot_info);

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

fn enable_nxe_bit() {
    use x86::shared::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::shared::control_regs::{cr0, cr0_write, CR0_WRITE_PROTECT};

    unsafe { cr0_write(cr0() | CR0_WRITE_PROTECT) };
}
