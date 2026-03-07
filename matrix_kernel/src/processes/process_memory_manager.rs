use alloc::collections::btree_set::BTreeSet;
use x86_64::structures::paging::{PhysFrame, Size4KiB};

use crate::processes::process_memory_manager::vads::Vad;

pub struct ProcessMemoryManager {
    page_table_frame: PhysFrame<Size4KiB>,
    frames_owned: BTreeSet<Vad>,
}

impl ProcessMemoryManager {
    pub fn new(page_table_frame: PhysFrame<Size4KiB>) -> Self {
        Self {
            page_table_frame,
            frames_owned: Default::default(),
        }
    }
}

mod vads {
    use x86_64::structures::paging::{frame::PhysFrameRangeInclusive, page::PageRangeInclusive};

    pub struct Vad {
        pages: PageRangeInclusive,
        frames: PhysFrameRangeInclusive,
    }
}
