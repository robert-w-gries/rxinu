use arch::x86::memory::MemoryController;
use arch::x86::interrupts::DOUBLE_FAULT_IST_INDEX;
use x86::current::task::TaskStateSegment;
use x86::current::segmentation::{SegmentBitness, SegmentDescriptor};
use x86::shared::PrivilegeLevel;
use x86::shared::segmentation::{Type, CODE_READ, DATA_WRITE};
use super::{GdtArray, GDT, GDT_TSS};

pub const GDT_SIZE: usize = GDT_TSS + 2;

pub fn tss(memory_controller: &mut MemoryController) -> TaskStateSegment {
    let mut tss = TaskStateSegment::new();

    let double_fault_stack = memory_controller
        .alloc_stack(1)
        .expect("could not allocate double fault stack");
    tss.ist[DOUBLE_FAULT_IST_INDEX] = double_fault_stack.top() as u64;

    tss
}

pub fn create_gdt(tss: &TaskStateSegment) -> &'static GdtArray {
    let tss_segs = SegmentDescriptor::new_tss(&tss, PrivilegeLevel::Ring3);

    GDT.call_once(|| {
        [
            SegmentDescriptor::NULL,
            // Kernel Code
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Code(CODE_READ),
                false,
                PrivilegeLevel::Ring0,
                SegmentBitness::Bits64,
            ),
            // Kernel data
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Data(DATA_WRITE),
                false,
                PrivilegeLevel::Ring0,
                SegmentBitness::Bits64,
            ),
            // User Code
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Data(DATA_WRITE),
                false,
                PrivilegeLevel::Ring3,
                SegmentBitness::Bits64,
            ),
            // User Data
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Data(DATA_WRITE),
                false,
                PrivilegeLevel::Ring3,
                SegmentBitness::Bits64,
            ),
            // TSS
            tss_segs[0],
            tss_segs[1],
        ]
    })
}
