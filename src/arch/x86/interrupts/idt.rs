use arch::x86::interrupts::{DOUBLE_FAULT_IST_INDEX, exception, irq};
use core::fmt;
use core::mem;
use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::paging::VAddr;
use x86::shared::segmentation::{self, SegmentSelector};

#[cfg(target_arch = "x86")] use x86::bits32::irq::{IdtEntry, Type};
#[cfg(target_arch = "x86_64")] use x86::bits64::irq::{IdtEntry, Type};

const IRQ_OFFSET: usize = 32;
const KERNEL_CODE_SELECTOR: SegmentSelector = SegmentSelector::new(1, PrivilegeLevel::Ring0);

static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

static mut IDTR: DescriptorTablePointer<IdtEntry> = DescriptorTablePointer {
    limit: 0,
    base: 0 as * const _,
};

pub unsafe fn init() {
    IDTR.limit = (IDT.len() * mem::size_of::<IdtEntry>() - 1) as u16;
    IDTR.base = IDT.as_ptr();

    IDT[0] = fn_handler_entry(exception::divide_by_zero as usize);
    IDT[1] = fn_handler_entry(exception::debug as usize);
    IDT[2] = fn_handler_entry(exception::non_maskable as usize);
    IDT[3] = fn_handler_entry(exception::breakpoint as usize);
    IDT[4] = fn_handler_entry(exception::overflow as usize);
    IDT[5] = fn_handler_entry(exception::bound_range as usize);
    IDT[6] = fn_handler_entry(exception::invalid_opcode as usize);
    IDT[7] = fn_handler_entry(exception::device_not_available as usize);
    IDT[8] = double_fault_handler_entry(exception::double_fault as usize,
                                        DOUBLE_FAULT_IST_INDEX as u8);
    // 9 no longer available
    IDT[10] = fn_handler_entry(exception::invalid_tss as usize);
    IDT[11] = fn_handler_entry(exception::segment_not_present as usize);
    IDT[12] = fn_handler_entry(exception::stack_segment as usize);
    IDT[13] = fn_handler_entry(exception::protection as usize);
    IDT[14] = fn_handler_entry(exception::page_fault as usize);
    // 15 reserved
    IDT[16] = fn_handler_entry(exception::fpu as usize);
    IDT[17] = fn_handler_entry(exception::alignment_check as usize);
    IDT[18] = fn_handler_entry(exception::machine_check as usize);
    IDT[19] = fn_handler_entry(exception::simd as usize);
    IDT[20] = fn_handler_entry(exception::virtualization as usize);
    // 21 through 29 reserved
    IDT[30] = fn_handler_entry(exception::security as usize);
    // 31 reserved

    IDT[IRQ_OFFSET+0] = fn_handler_entry(irq::timer as usize);
    IDT[IRQ_OFFSET+1] = fn_handler_entry(irq::keyboard as usize);
    IDT[IRQ_OFFSET+2] = fn_handler_entry(irq::cascade as usize);
    IDT[IRQ_OFFSET+3] = fn_handler_entry(irq::com2 as usize);
    IDT[IRQ_OFFSET+4] = fn_handler_entry(irq::com1 as usize);

    dtables::lidt(&IDTR);
}

fn fn_handler_entry(ptr: usize) -> IdtEntry {
    IdtEntry::new(VAddr::from_usize(ptr), KERNEL_CODE_SELECTOR,
                  PrivilegeLevel::Ring0, Type::InterruptGate, 0)
}

#[cfg(target_arch = "x86")]
fn double_fault_handler_entry(ptr: usize, index: u8) -> IdtEntry {
    fn_handler_entry(ptr)
}

#[cfg(target_arch = "x86_64")]
fn double_fault_handler_entry(ptr: usize, index: u8) -> IdtEntry {
    let mut i = fn_handler_entry(ptr);
    i.ist_index = DOUBLE_FAULT_IST_INDEX as u8;
    i
}

bitflags! {
    /// Describes an page fault error code.
    pub struct PageFaultErrorCode: u64 {
        /// If this flag is set, the page fault was caused by a page-protection violation,
        /// else the page fault was caused by a not-present page.
        const PROTECTION_VIOLATION = 1 << 0;

        /// If this flag is set, the memory access that caused the page fault was a write.
        /// Else the access that caused the page fault is a memory read. This bit does not
        /// necessarily indicate the cause of the page fault was a read or write violation.
        const CAUSED_BY_WRITE = 1 << 1;

        /// If this flag is set, an access in user mode (CPL=3) caused the page fault. Else
        /// an access in supervisor mode (CPL=0, 1, or 2) caused the page fault. This bit
        /// does not necessarily indicate the cause of the page fault was a privilege violation.
        const USER_MODE = 1 << 2;

        /// If this flag is set, the page fault is a result of the processor reading a 1 from
        /// a reserved field within a page-translation-table entry.
        const MALFORMED_TABLE = 1 << 3;

        /// If this flag is set, it indicates that the access that caused the page fault was an
        /// instruction fetch.
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

/// Represents the exception stack frame pushed by the CPU on exception entry.
#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
    pub stack_pointer: u32,
    pub stack_segment: u32,
}

#[cfg(target_arch = "x86_64")]
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
        #[cfg(target_arch = "x86")]
        struct Hex(u32);
        #[cfg(target_arch = "x86_64")]
        struct Hex(u64);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("ExceptionStackFrame");
        s.field("instruction_pointer", &Hex(self.instruction_pointer));
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &Hex(self.cpu_flags));
        s.field("stack_pointer", &Hex(self.stack_pointer));
        s.field("stack_segment", &self.stack_segment);
        s.finish()
    }
}
