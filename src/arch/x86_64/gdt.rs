use arch::x86_64::interrupts::DOUBLE_FAULT_IST_INDEX;
use arch::x86_64::memory::MemoryController;
use core::mem;
use spin::Once;
use x86::current::segmentation::{SegmentBitness, SegmentDescriptor};
use x86::current::task::TaskStateSegment;
use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::segmentation::{Type, CODE_READ, DATA_WRITE};

pub const GDT_SIZE: usize = GDT_TSS + 2;

// Segment Selector Index
pub const GDT_KERNEL_CODE: usize = 1;
pub const GDT_KERNEL_DATA: usize = 2;
pub const GDT_USER_CODE: usize = 3;
pub const GDT_USER_DATA: usize = 4;
pub const GDT_TSS: usize = 5;

pub type GdtArray = [SegmentDescriptor; GDT_SIZE];
pub static GDT: Once<GdtArray> = Once::new();

pub fn init(memory_controller: &mut MemoryController) {
    let tss = tss(memory_controller);
    let gdt: &GdtArray = create_gdt(&tss);

    let gdtr: DescriptorTablePointer<SegmentDescriptor> = DescriptorTablePointer {
        base: gdt.as_ptr(),
        limit: (gdt.len() * mem::size_of::<SegmentDescriptor>() - 1) as u16,
    };

    unsafe {
        use x86::current::segmentation::set_cs;
        use x86::shared::segmentation::{load_ss, SegmentSelector};
        use x86::shared::task::load_tr;

        dtables::lgdt(&gdtr);

        set_cs(SegmentSelector::new(
            GDT_KERNEL_CODE as u16,
            PrivilegeLevel::Ring0,
        ));

        load_selectors(GDT_KERNEL_DATA, PrivilegeLevel::Ring0);
        load_ss(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));

        load_tr(SegmentSelector::new(GDT_TSS as u16, PrivilegeLevel::Ring3));
    }
}

/// Load all selectors except for Stack Segment
/// Stack Segment cannot be loaded as PrivilegeLevel3 and it is not usually loaded anyway
/// [unsafe]
/// This function is purely assembly and is inherently unsafe
pub unsafe fn load_selectors(selector_index: usize, privilege: PrivilegeLevel) {
    use x86::shared::segmentation::{load_ds, load_es, load_fs, load_gs, SegmentSelector};

    load_ds(SegmentSelector::new(selector_index as u16, privilege));
    load_es(SegmentSelector::new(selector_index as u16, privilege));
    load_fs(SegmentSelector::new(selector_index as u16, privilege));
    load_gs(SegmentSelector::new(selector_index as u16, privilege));
}
pub fn tss(memory_controller: &mut MemoryController) -> TaskStateSegment {
    let mut tss = TaskStateSegment::new();

    let double_fault_stack = memory_controller
        .alloc_stack(1)
        .expect("could not allocate double fault stack");
    tss.ist[DOUBLE_FAULT_IST_INDEX] = double_fault_stack.top() as u64;

    // Privilege Level stacks
    for i in 0..3 {
        tss.rsp[i] = memory_controller
            .alloc_stack(1)
            .expect("Could not allocate privilege level stack")
            .top() as u64;
    }

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
