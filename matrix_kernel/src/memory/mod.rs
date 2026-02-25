use matrix_boot_args::memory_map::MatrixMemoryMap;
use x86_64::{VirtAddr, structures::paging::OffsetPageTable};

use crate::memory::paging::get_page_table;

pub mod allocator;
pub mod paging;
pub mod simple_allocator;


/// # Safety
/// 
/// creates heap and stuff. should be called once in the kenrel boot
/// 
pub unsafe fn init_memory(
    physical_memory_offset: VirtAddr,
    _boot_info: &MatrixMemoryMap,
) -> OffsetPageTable<'static> {
    let page_table = unsafe {
        let level_4_table = get_page_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    };

    // let frame_allocator = Boot

    // init_heap(mapper, frame_allocator);

    page_table
}
