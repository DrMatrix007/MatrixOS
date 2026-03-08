use alloc::collections::btree_set::BTreeSet;
use x86_64::{
    VirtAddr,
    structures::paging::{PhysFrame, Size4KiB},
};

use crate::processes::process_memory_manager::vads::Vad;

pub enum AllocationError {
    AlreadyTaken,
}

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

    pub fn allocate_fixed_memory(
        &self,
        start: VirtAddr,
        size: u64,
    ) -> Result<&Vad, AllocationError> {
        let start = start;
        let end = start + size;

        let mut closest_frames = self.frames_owned.iter().filter(|x| {
            let owned_start = x.start_address();
            let owned_end = owned_start + x.len();

            start < owned_end && owned_start < end
        });

        let chosen_vad = closest_frames.next();
        let overlapping = closest_frames.count();

        assert!(overlapping != 0, "there should not be overlapping vads!");

        if let Some(_) = chosen_vad {
            return Err(AllocationError::AlreadyTaken);
        }

        loop {}
    }
}

mod vads {

    use alloc::vec::Vec;
    use x86_64::{
        VirtAddr,
        structures::paging::{PhysFrame, page::PageRangeInclusive},
    };

    pub struct Vad {
        pub pages: PageRangeInclusive,
        pub frames: Vec<PhysFrame>,
    }

    impl Vad {
        pub fn start_address(&self) -> VirtAddr {
            self.pages.start.start_address()
        }

        pub fn len(&self) -> u64 {
            self.pages.len() * self.pages.start.size()
        }
    }
}
