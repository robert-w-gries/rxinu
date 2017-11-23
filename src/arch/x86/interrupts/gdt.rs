use core::mem;
use spin::Once;
use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::segmentation::{Type, CODE_READ, DATA_WRITE};
use x86::shared::task::load_tr;

#[cfg(target_arch = "x86")]
use x86::bits32::task::TaskStateSegment;
#[cfg(target_arch = "x86")]
use x86::bits32::segmentation::{self, SegmentDescriptor};

#[cfg(target_arch = "x86_64")]
use x86::bits64::task::TaskStateSegment;
#[cfg(target_arch = "x86_64")]
use x86::bits64::segmentation::{self, SegmentBitness, SegmentDescriptor};

// Segment Selector Index
const GDT_KERNEL_CODE: usize = 1;
const GDT_KERNEL_DATA: usize = 2;
#[allow(dead_code)]
const GDT_USER_CODE: usize = 3;
#[allow(dead_code)]
const GDT_USER_DATA: usize = 4;
const GDT_TSS: usize = 5;

#[cfg(target_arch = "x86")]
const GDT_SIZE: usize = GDT_TSS + 1;
#[cfg(target_arch = "x86_64")]
const GDT_SIZE: usize = GDT_TSS + 2;

type GdtArray = [SegmentDescriptor; GDT_SIZE];
static GDT: Once<GdtArray> = Once::new();

pub fn init(tss: &TaskStateSegment) {
    let gdt: &GdtArray = create_gdt(&tss);

    let gdtr: DescriptorTablePointer<SegmentDescriptor> = DescriptorTablePointer {
        base: gdt.as_ptr(),
        limit: (gdt.len() * mem::size_of::<SegmentDescriptor>() - 1) as u16,
    };

    unsafe {
        use x86::shared::segmentation::*;

        dtables::lgdt(&gdtr);

        segmentation::set_cs(SegmentSelector::new(
            GDT_KERNEL_CODE as u16,
            PrivilegeLevel::Ring0,
        ));
        load_ds(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));
        load_es(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));
        load_fs(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));
        load_gs(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));
        load_ss(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));

        load_tr(SegmentSelector::new(GDT_TSS as u16, PrivilegeLevel::Ring3));
    }
}

#[cfg(target_arch = "x86")]
fn create_gdt(&tss: &TaskStateSegment) -> &'static GdtArray {
    GDT.call_once(|| {
        [
            SegmentDescriptor::NULL,
            // Kernel Code
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Code(CODE_READ),
                false,
                PrivilegeLevel::Ring0
            ),
            // Kernel Data
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Data(DATA_WRITE),
                false,
                PrivilegeLevel::Ring0
            ),
            // User Code
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Code(CODE_READ),
                false,
                PrivilegeLevel::Ring3
            ),
            // User Data
            SegmentDescriptor::new_memory(
                0,
                0,
                Type::Data(DATA_WRITE),
                false,
                PrivilegeLevel::Ring3
            ),
            SegmentDescriptor::new_tss(&tss, PrivilegeLevel::Ring3),
        ]
    })
}

#[cfg(target_arch = "x86_64")]
fn create_gdt(tss: &TaskStateSegment) -> &'static GdtArray {
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
