use core::mem;

#[cfg(target_arch = "x86")] use x86::bits32::task::TaskStateSegment;
#[cfg(target_arch = "x86")] use x86::bits32::segmentation::{self, SegmentBitness, SegmentDescriptor, SegmentSelector};

#[cfg(target_arch = "x86_64")] use x86::bits64::task::TaskStateSegment;
#[cfg(target_arch = "x86_64")] use x86::bits64::segmentation::{self, SegmentBitness, SegmentDescriptor, SegmentSelector};

use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::segmentation::{Type, CODE_CONFORMING, DATA_WRITE};
use x86::shared::task::load_tr;

#[allow(dead_code)]
const GDT_NULL: usize = 0;
const GDT_KERNEL_CODE: usize = 1;
const GDT_KERNEL_DATA: usize = 2;
const GDT_TSS: usize = 3;

pub fn init(tss: &TaskStateSegment) {
    // TODO: Investigate PrivilegeLevel for TSS
    let tss_segs = SegmentDescriptor::new_tss(&tss, PrivilegeLevel::Ring0); 

    let gdt: [SegmentDescriptor; 5] = [
        // Null
        SegmentDescriptor::NULL,
        // Kernel code
        SegmentDescriptor::new_memory(0,
                                      0,
                                      Type::Code(CODE_CONFORMING),
                                      false,
                                      PrivilegeLevel::Ring0,
                                      SegmentBitness::Bits64),
        // Kernel data
        SegmentDescriptor::new_memory(0,
                                      0,
                                      Type::Data(DATA_WRITE),
                                      false,
                                      PrivilegeLevel::Ring0,
                                      SegmentBitness::Bits64),
        // TSS
        tss_segs[0],
        tss_segs[1],
    ];

    let gdtr: DescriptorTablePointer<SegmentDescriptor> = DescriptorTablePointer {
        limit: (gdt.len() * mem::size_of::<SegmentDescriptor>() - 1) as u16,
        base: gdt.as_ptr(),
    };

    unsafe { dtables::lgdt(&gdtr); }

    // TODO: Investigate PrivilegeLevel for segment selectors and TSS load
    unsafe {
        segmentation::set_cs(SegmentSelector::new(GDT_KERNEL_CODE as u16, PrivilegeLevel::Ring0));
        segmentation::load_ds(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
        segmentation::load_es(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
        segmentation::load_fs(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
        segmentation::load_gs(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
        segmentation::load_ss(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));

        load_tr(SegmentSelector::new(GDT_TSS as u16, PrivilegeLevel::Ring0));
    }
}
