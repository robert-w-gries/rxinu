use arch::x86_64::memory::map_page;
use x86_64::structures::paging::{
    FrameAllocator, Page, PageRangeInclusive, PageSize, PageTableFlags, RecursivePageTable,
    Size4KiB,
};

pub struct StackAllocator {
    range: PageRangeInclusive,
}

impl StackAllocator {
    pub fn new(page_range: PageRangeInclusive) -> StackAllocator {
        StackAllocator { range: page_range }
    }
}

impl StackAllocator {
    pub fn alloc_stack<FA: FrameAllocator<Size4KiB>>(
        &mut self,
        page_table: &mut RecursivePageTable,
        frame_allocator: &mut FA,
        size_in_pages: usize,
    ) -> Option<Stack> {
        if size_in_pages == 0 {
            return None; // a zero sized stack makes no sense
        }

        // clone the range, since we only want to change it on success
        let mut range = self.range.clone();

        // try to allocate the stack pages and a guard page
        let guard_page = range.next();
        let stack_start = range.next();
        let stack_end = if size_in_pages == 1 {
            stack_start
        } else {
            // choose the (size_in_pages-2)th element, since index
            // starts at 0 and we already allocated the start page
            range.nth(size_in_pages - 2)
        };

        match (guard_page, stack_start, stack_end) {
            (Some(_), Some(start), Some(end)) => {
                // success! write back updated range
                self.range = range;

                // map stack pages to physical frames
                for page in Page::range_inclusive(start, end) {
                    let flags = PageTableFlags::PRESENT
                        | PageTableFlags::WRITABLE
                        | PageTableFlags::NO_EXECUTE;

                    map_page(page, flags, page_table, frame_allocator)
                        .expect("Stack page mapping failed");
                }

                // create a new stack
                let top_of_stack = end.start_address() + Size4KiB::SIZE;
                Some(Stack::new(
                    top_of_stack.as_u64() as usize,
                    start.start_address().as_u64() as usize,
                ))
            }
            _ => None, /* not enough pages */
        }
    }
}

#[derive(Debug)]
pub struct Stack {
    top: usize,
    bottom: usize,
}

impl Stack {
    fn new(top: usize, bottom: usize) -> Stack {
        assert!(top > bottom);
        Stack {
            top: top,
            bottom: bottom,
        }
    }
}

impl Stack {
    #[allow(dead_code)]
    pub fn top(&self) -> usize {
        self.top
    }

    #[allow(dead_code)]
    pub fn bottom(&self) -> usize {
        self.bottom
    }
}
