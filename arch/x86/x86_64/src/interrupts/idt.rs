use core::fmt;
use core::mem;

#[cfg(target_arch = "x86_64")] use x86::bits64::irq::IdtEntry;

use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::segmentation;

use interrupts::{exception, irq};

const IRQ_OFFSET: usize = 32;

static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

static mut IDTR: DescriptorTablePointer<IdtEntry> = DescriptorTablePointer {
    limit: 0,
    base: 0 as * const _,
};

pub unsafe fn init() {
    IDTR.limit = (IDT.len() * mem::size_of::<IdtEntry>() - 1) as u16;
    IDTR.base = IDT.as_ptr();

    set_handler_fn(&mut IDT[0],  exception::divide_by_zero);
    set_handler_fn(&mut IDT[3],  exception::breakpoint);
    set_handler_fn(&mut IDT[6],  exception::invalid_opcode);
    set_handler_fn(&mut IDT[8],  exception::double_fault);
    set_handler_fn(&mut IDT[14], exception::page_fault);

    // TODO: Implement stack for double fault
    //.set_stack_index(interrupts::DOUBLE_FAULT_IST_INDEX as u16);

    set_handler_fn(&mut IDT[IRQ_OFFSET+0], irq::cascade);
    set_handler_fn(&mut IDT[IRQ_OFFSET+1], irq::com1);
    set_handler_fn(&mut IDT[IRQ_OFFSET+2], irq::com2);

    dtables::lidt(&IDTR);
}

type HandlerFunc = extern "x86-interrupt" fn(&mut ExceptionStackFrame);
fn set_handler_fn(i: &mut IdtEntry, e: HandlerFunc) {
    let ptr = e as usize;
    i.base_lo = ((ptr as u64) & 0xFFFF) as u16;
    i.base_hi = ptr as u64 >> 16;
    i.selector = segmentation::cs().bits();

    use x86::shared::descriptor::*;
    use x86::shared::PrivilegeLevel::Ring0;

    i.flags = Flags::from_priv(Ring0)
                  .const_or(FLAGS_TYPE_SYS_NATIVE_INTERRUPT_GATE)
                  .const_or(FLAGS_PRESENT);
}

/// Represents the exception stack frame pushed by the CPU on exception entry.
#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

impl fmt::Debug for ExceptionStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u64);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("ExceptionStackFrame");
        s.field("instruction_pointer", &self.instruction_pointer);
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &Hex(self.cpu_flags));
        s.field("stack_pointer", &self.stack_pointer);
        s.field("stack_segment", &self.stack_segment);
        s.finish()
    }
}
