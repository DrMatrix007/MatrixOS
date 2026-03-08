use log::info;
use matrix_boot_common::boot_info::memory_map::MatrixMemoryMap;
use spin::Mutex;
use x86_64::{VirtAddr, structures::paging::OffsetPageTable};

use crate::memory::{allocator::init_heap, once_objects::OnceMapper, paging::get_page_table};

pub mod allocator;
pub mod memory_map_frame_allocator;
pub mod once_objects;
pub mod paging;
pub mod silly_memory_map_frame_allocator;
pub mod apic_mapping;

pub static PAGE_TABLE: Mutex<OnceMapper<OffsetPageTable>> = Mutex::new(OnceMapper::new());

/// # Safety
///
/// creates heap and stuff. should be called once in the kenrel boot
///
pub unsafe fn init_memory(physical_memory_offset: VirtAddr, memory_map: &'static MatrixMemoryMap) {
    let mut page_table = unsafe {
        let level_4_table = get_page_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    };

    info!("before init heap");

    init_heap(&mut page_table, memory_map).unwrap();

    PAGE_TABLE.lock().init(page_table);
}
