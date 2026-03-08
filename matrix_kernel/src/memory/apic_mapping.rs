use log::info;
use x86_64::{
    VirtAddr,
    structures::paging::{Mapper, OffsetPageTable, PageTableFlags},
};

use crate::{
    memory::{allocator::FRAME_ALLOCATOR, paging::get_page_table},
    memory_locations::{APIC_FRAME, APIC_PAGE},
};

pub fn init_apic_mappings(phys_offset: VirtAddr) {
    let page_table = unsafe { get_page_table(phys_offset) };

    let mut page_table = unsafe { OffsetPageTable::new(page_table, phys_offset) };

    info!("mapping apic");
    
    unsafe {
        page_table
        .map_to(
                APIC_PAGE,
                APIC_FRAME,
                PageTableFlags::PRESENT | PageTableFlags::NO_CACHE | PageTableFlags::WRITABLE,
                &mut *FRAME_ALLOCATOR.lock(),
            )
            .expect("this should map")
            .flush();
    }
}
