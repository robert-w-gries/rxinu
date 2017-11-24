use arch::x86::memory::MemoryController;
use x86::bits32::task::TaskStateSegment;
use x86::bits32::segmentation::SegmentDescriptor;
use x86::shared::PrivilegeLevel;
use x86::shared::segmentation::{Type, CODE_READ, DATA_WRITE};
use super::{GdtArray, GDT, GDT_TSS};

pub const GDT_SIZE: usize = GDT_TSS + 1;

pub fn tss(_memory_controller: &mut MemoryController) -> TaskStateSegment {
    TaskStateSegment::new()
}

pub fn create_gdt(&tss: &TaskStateSegment) -> &'static GdtArray {
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
            ),
            // Kernel Data
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Data(DATA_WRITE),
                false,
                PrivilegeLevel::Ring0,
            ),
            // User Code
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Code(CODE_READ),
                false,
                PrivilegeLevel::Ring3,
            ),
            // User Data
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Data(DATA_WRITE),
                false,
                PrivilegeLevel::Ring3,
            ),
            SegmentDescriptor::new_tss(&tss, PrivilegeLevel::Ring3),
        ]
    })
}
