use core::fmt;
use core::mem;

#[cfg(target_arch = "x86_64")] use x86::bits64::irq::IdtEntry;

use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::segmentation;

use interrupts::{DOUBLE_FAULT_IST_INDEX, exception, irq};

const IRQ_OFFSET: usize = 32;

static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

static mut IDTR: DescriptorTablePointer<IdtEntry> = DescriptorTablePointer {
    limit: 0,
    base: 0 as * const _,
};

pub unsafe fn init() {
    IDTR.limit = (IDT.len() * mem::size_of::<IdtEntry>() - 1) as u16;
    IDTR.base = IDT.as_ptr();

    set_handler_fn(&mut IDT[0], exception::divide_by_zero);
    set_handler_fn(&mut IDT[1], exception::debug);
    set_handler_fn(&mut IDT[2], exception::non_maskable);
    set_handler_fn(&mut IDT[3], exception::breakpoint);
    set_handler_fn(&mut IDT[4], exception::overflow);
    set_handler_fn(&mut IDT[5], exception::bound_range);
    set_handler_fn(&mut IDT[6], exception::invalid_opcode);
    set_handler_fn(&mut IDT[7], exception::device_not_available);
    set_double_fault_handler_fn(&mut IDT[8],
                                exception::double_fault,
                                DOUBLE_FAULT_IST_INDEX as u8);
    // 9 no longer available
    set_handler_fn(&mut IDT[10], exception::invalid_tss);
    set_handler_fn(&mut IDT[11], exception::segment_not_present);
    set_handler_fn(&mut IDT[12], exception::stack_segment);
    set_handler_fn(&mut IDT[13], exception::protection);
    set_handler_fn(&mut IDT[14], exception::page_fault);
    // 15 reserved
    set_handler_fn(&mut IDT[16], exception::fpu);
    set_handler_fn(&mut IDT[17], exception::alignment_check);
    set_handler_fn(&mut IDT[18], exception::machine_check);
    set_handler_fn(&mut IDT[19], exception::simd);
    set_handler_fn(&mut IDT[20], exception::virtualization);
    // 21 through 29 reserved
    set_handler_fn(&mut IDT[30], exception::security);
    // 31 reserved

    set_handler_fn(&mut IDT[IRQ_OFFSET+2], irq::cascade);
    set_handler_fn(&mut IDT[IRQ_OFFSET+3], irq::com2);
    set_handler_fn(&mut IDT[IRQ_OFFSET+4], irq::com1);

    dtables::lidt(&IDTR);
}

type HandlerFunc = extern "x86-interrupt" fn(&mut ExceptionStackFrame);
fn set_handler_fn(i: &mut IdtEntry, e: HandlerFunc) {
    let ptr = e as usize;
    i.base_lo = ((ptr as u64) & 0xFFFF) as u16;
    i.base_hi = ptr as u64 >> 16;
    i.selector = segmentation::cs();

    use x86::shared::descriptor::*;
    use x86::shared::PrivilegeLevel::Ring0;

    i.flags = Flags::from_priv(Ring0)
                  .const_or(FLAGS_TYPE_SYS_NATIVE_INTERRUPT_GATE)
                  .const_or(FLAGS_PRESENT);
}

fn set_double_fault_handler_fn(mut i: &mut IdtEntry, e: HandlerFunc, index: u8) {
    i.ist_index = index;
    set_handler_fn(&mut i, e);
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
