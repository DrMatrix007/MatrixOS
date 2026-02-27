use log::info;
use matrix_boot_common::boot_info::memory_map::MatrixMemoryMap;
use x86_64::{VirtAddr, structures::paging::OffsetPageTable};

use crate::memory::{allocator::init_heap, paging::get_page_table};

pub mod allocator;
pub mod paging;
pub mod simple_allocator;

/// # Safety
///
/// creates heap and stuff. should be called once in the kenrel boot
///
pub unsafe fn init_memory(
    physical_memory_offset: VirtAddr,
    memory_map: &'static MatrixMemoryMap,
) -> OffsetPageTable<'static> {
    let mut page_table = unsafe {
        let level_4_table = get_page_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    };

    info!("before init heap");

    init_heap(&mut page_table, memory_map).unwrap();

    page_table
}
