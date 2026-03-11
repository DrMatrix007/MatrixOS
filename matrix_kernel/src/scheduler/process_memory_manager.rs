use alloc::{collections::btree_set::BTreeSet, vec::Vec};
use anyhow::{Context, Result, anyhow};
use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
        mapper::CleanUp,
    },
};

use crate::{
    memory::{PAGE_TABLE, allocator::FRAME_ALLOCATOR},
    memory_locations::PROCESS_CREATION_PAGE_MAP_BASE,
    scheduler::{process_memory_manager::vads::Vad, process_page_table::ProcessPageTable},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllocationError {
    AlreadyTaken,
    CantAllocateFrames,
}

pub struct ProcessMemoryManager {
    page_table: ProcessPageTable,
    frames_owned: BTreeSet<Vad>,
}

impl ProcessMemoryManager {
    pub fn new(page_table_frame: PhysFrame<Size4KiB>) -> Result<Self> {
       
         let new_page_table =
            unsafe { &mut *(new_page_table_page.start_address().as_mut_ptr() as *mut PageTable) };

        *new_page_table = PageTable::new();

        current_page_table
            .unmap(new_page_table_page)
            .map_err(|x| anyhow!("{:?}", x))
            .context("unmapping the temp page table")?
            .1
            .flush();

        unsafe {
            current_page_table
                .inner_mut()
                .clean_up(&mut *frame_allocator)
        };

        Ok(Self {
            page_table_frame,
            frames_owned: Default::default(),
        })
    }

    pub fn allocate_memory(
        &mut self,
        start: VirtAddr,
        size: u64,
        flags: PageTableFlags,
    ) -> Result<&Vad, AllocationError> {
        let end = start + size;

        let mut closest_frames = self.frames_owned.iter().filter(|x| {
            let owned_start = x.start_address();
            let owned_end = owned_start + x.len();

            start < owned_end && owned_start < end
        });

        let chosen_vad = closest_frames.next();
        let overlapping = closest_frames.count();

        assert!(overlapping == 0, "there should not be overlapping vads!");

        if let Some(_) = chosen_vad {
            return Err(AllocationError::AlreadyTaken);
        }

        let mut frame_allocator = FRAME_ALLOCATOR.lock();

        let pages = Page::range_inclusive(
            Page::containing_address(start),
            Page::containing_address(end),
        );

        let frames = core::iter::repeat_with(|| frame_allocator.allocate_frame())
            .take(pages.len() as _)
            .collect::<Option<Vec<_>>>(); // TODO: handle failed allocation

        if let Some(frames) = frames {
            for (page, frame) in core::iter::zip(pages, &frames) {}

            let vad = Vad { pages, frames };
            let key = vad.pages.start;

            self.frames_owned.insert(vad);

            Ok(self.frames_owned.get(&key).expect("just added this"))
        } else {
            return Err(AllocationError::CantAllocateFrames);
        }
    }
}

mod vads {

    use core::{borrow::Borrow, cmp::Ordering};

    use alloc::vec::Vec;
    use x86_64::{
        VirtAddr,
        structures::paging::{Page, PhysFrame, page::PageRangeInclusive},
    };

    #[derive(Debug, Clone)]
    pub struct Vad {
        pub pages: PageRangeInclusive,
        pub frames: Vec<PhysFrame>,
    }

    impl PartialEq for Vad {
        fn eq(&self, other: &Self) -> bool {
            self.pages.start == other.pages.start
        }
    }

    impl Eq for Vad {}

    impl PartialOrd for Vad {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Vad {
        fn cmp(&self, other: &Self) -> Ordering {
            self.pages.start.cmp(&other.pages.start)
        }
    }

    impl Vad {
        pub fn start_address(&self) -> VirtAddr {
            self.pages.start.start_address()
        }

        pub fn len(&self) -> u64 {
            self.pages.len() * self.pages.start.size()
        }
    }

    impl Borrow<Page> for Vad {
        fn borrow(&self) -> &Page {
            &self.pages.start
        }
    }
}
