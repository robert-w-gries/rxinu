#![feature(alloc, collections)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

#![feature(compiler_builtins_lib)]
extern crate compiler_builtins;

#[cfg(target_arch = "x86")]
#[macro_use]
extern crate arch_i686 as arch;

#[cfg(target_arch = "x86_64")]
#[macro_use]
extern crate arch_x86_64 as arch;

extern crate alloc;
extern crate multiboot2;

#[macro_use]
extern crate collections;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    arch::console::init();

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    arch::memory::init(boot_info);

    use alloc::boxed::Box;
    let mut heap_test = Box::new(42);
    *heap_test -= 15;
    let heap_test2 = Box::new("hello");
    println!("{:?} {:?}", heap_test, heap_test2);

    let mut vec_test = vec![1,2,3,4,5,6,7];
    vec_test[3] = 42;
    for i in &vec_test {
            print!("{} ", i);
    }

    for i in 0..10000 {
            format!("Some String");
    }

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
