#![feature(
    abi_x86_interrupt, alloc, allocator_api, global_allocator, asm, const_fn, const_max_value,
    const_unique_new, const_atomic_usize_new, const_fn, global_asm, lang_items, naked_functions,
    panic_info_message, ptr_internals, unique
)]
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
use sync::{IrqLock, Semaphore};

lazy_static! {
    static ref SEM: IrqLock<Semaphore> = IrqLock::new(Semaphore::new(2));
}

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

    syscall::create(String::from("rxinu_main"), 10, rxinu_main).unwrap();

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
    use task::{global_sched, ProcessId, State, Scheduling};

    arch::console::clear_screen();
    kprintln!("In main process!\n");

    let process_a = syscall::create(String::from("process a"), 25, process_a).unwrap();
    let process_b = syscall::create(String::from("process b"), 25, process_b).unwrap();

    // Kill process before it can run
    let pid_kill = syscall::create(String::from("kill_process"), 0, kill_process).unwrap();
    syscall::kill(pid_kill).unwrap();

    // Suspend process before it can run
    let pid = syscall::create(String::from("test_process"), 0, test_process).unwrap();
    syscall::suspend(pid).unwrap();

    // Both process A and B should run again
    SEM.lock().signaln(2).unwrap();

    syscall::resume(pid).unwrap();

    let check_state = |p: ProcessId, s: State| {
        let proc_ref = global_sched().get_process(p).unwrap();
        let state = proc_ref.read().state;
        state == s
    };

    assert!(check_state(pid, State::Ready));
    assert!(check_state(process_a, State::Wait));
    assert!(check_state(process_b, State::Wait));
    assert!(global_sched().get_process(pid_kill).is_err());
}

pub extern "C" fn test_process() {
    kprintln!("\nIn test process!");
    kprintln!("\nYou can now type...\n");
}

pub extern "C" fn process_a() {
    kprintln!("\nIn process_a!");
    loop {
        SEM.lock().wait().unwrap();

        syscall::yield_cpu().unwrap();
        arch::interrupts::pause();
    }
}

pub extern "C" fn process_b() {
    kprintln!("\nIn process_b!");
    loop {
        SEM.lock().wait().unwrap();

        syscall::yield_cpu().unwrap();
        arch::interrupts::pause();
    }
}

pub extern "C" fn kill_process() {
    kprint!("\nIn kill_process");
    loop {
        kprint!(".");
        syscall::yield_cpu().unwrap();
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

    loop {
        unsafe {
            arch::interrupts::halt();
        }
    }
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
