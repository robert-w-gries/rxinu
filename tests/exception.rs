#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate lazy_static;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use rxinu::serial_println;
use rxinu::test::{exit_qemu, QemuExitCode};

static BREAKPOINT_HANDLER_CALLED: AtomicUsize = AtomicUsize::new(0);

entry_point!(main);

fn main(_boot_info: &'static BootInfo) -> ! {
    // idt::init() loads both GDT and IDT
    rxinu::arch::idt::init();
    test_main();
    loop {}
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rxinu::test::test_panic_handler(info);
}

#[test_case]
fn breakpoint() {
    BREAKPOINT_IDT.load();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    match BREAKPOINT_HANDLER_CALLED.load(Ordering::SeqCst) {
        1 => (),
        0 => {
            serial_println!("Breakpoint handler was not called.");
            exit_qemu(QemuExitCode::Failed);
        }
        other => {
            serial_println!("Breakpoint handler was called {} times", other);
            exit_qemu(QemuExitCode::Failed);
        }
    }
}

#[test_case]
fn stack_overflow() {
    // Load new IDT so exception returns to test code
    DOUBLE_FAULT_IDT.load();

    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    serial_println!("Double fault handler was not called");
    exit_qemu(QemuExitCode::Failed);
}

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref BREAKPOINT_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
    static ref DOUBLE_FAULT_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(rxinu::arch::idt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: &mut InterruptStackFrame) {
    BREAKPOINT_HANDLER_CALLED.fetch_add(1, Ordering::SeqCst);
}

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
