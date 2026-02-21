use x86_64::{VirtAddr, structures::paging::OffsetPageTable};

use crate::memory::{allocator::init_heap, paging::get_page_table};

pub mod allocator;
pub mod paging;
pub mod simple_allocator;

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let page_table = unsafe {
        let level_4_table = get_page_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    };

    let frame_allocator = Boot

    init_heap(mapper, frame_allocator);

    page_table
}
