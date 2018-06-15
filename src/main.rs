#![feature(
    abi_x86_interrupt, alloc, allocator_api, asm, const_fn, const_max_value,
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
pub mod ipc;
pub mod sync;
pub mod syscall;
pub mod task;

use alloc::String;
use arch::memory::heap::{HEAP_SIZE, HEAP_START};
use core::panic::PanicInfo;
use sync::{IrqLock, Semaphore};

lazy_static! {
    static ref SEM: IrqLock<Semaphore> = IrqLock::new(Semaphore::new(2));
    static ref COMPLETED_TEST: IrqLock<Semaphore> = IrqLock::new(Semaphore::new(0));
    static ref BUF: IrqLock<BoundedBuffer<char>> = IrqLock::new(BoundedBuffer::new(100));
}

use ipc::bounded_buffer::BoundedBuffer;

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

    syscall::create(String::from("rxinu_main"), 0, rxinu_main).unwrap();

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

    for i in b"Hello World!" {
        BUF.lock().push(*i as char).unwrap();
    }

    let process_a = syscall::create(String::from("process a"), 25, process_a).unwrap();
    let process_b = syscall::create(String::from("process b"), 25, process_b).unwrap();

    // Kill process before it can run
    let kill_process = syscall::create(String::from("kill_process"), 0, kill_process).unwrap();
    syscall::kill(kill_process).unwrap();

    // Suspend process before it can run
    let test_process = syscall::create(String::from("test_process"), 0, test_process).unwrap();
    syscall::suspend(test_process).unwrap();

    // Both process A and B should run again
    SEM.lock().signaln(2).unwrap();

    syscall::resume(test_process).unwrap();

    // Assertions: Process waits until signal from test_process
    COMPLETED_TEST.lock().wait().unwrap();
    unsafe {
        global_sched().resched().unwrap();
    }

    kprintln!("\nTesting scheduler state...");

    let check_state = |p: ProcessId, s: State| global_sched().get_process(p).unwrap().state() == s;

    // kill_process should be removed from process list by now
    assert!(global_sched().get_process(kill_process).is_err());
    assert!(global_sched().get_process(test_process).is_err());

    assert!(check_state(process_a, State::Wait));
    assert!(check_state(process_b, State::Wait));

    kprintln!("Scheduling tests passed!\n");
}

pub extern "C" fn test_process() {
    kprint!("\nIn test process!\nBuffer = ");
    let len = BUF.lock().len();
    for _ in 0..len {
        kprint!("{}", BUF.lock().pop().unwrap());
    }
    kprintln!();
    COMPLETED_TEST.lock().signal().unwrap();
}

pub extern "C" fn process_a() {
    kprintln!("\nIn process_a!");
    loop {
        SEM.lock().wait().unwrap();
        kprintln!("ProcessA Waited!");

        syscall::yield_cpu().unwrap();
        arch::interrupts::pause();
    }
}

pub extern "C" fn process_b() {
    kprintln!("\nIn process_b!");
    loop {
        SEM.lock().wait().unwrap();
        kprintln!("ProcessB Waited!");

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
    arch::interrupts::disable_then_execute(|| {
        kprintln!("\n\nPANIC");

        if let Some(location) = info.location() {
            kprint!("in {} at line {}", location.file(), location.line());
        }

        if let Some(message) = info.message() {
            kprintln!("\n    {:?}", message);
        }
    });

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
