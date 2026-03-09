use matrix_common::boot_info::memory_map::MatrixMemoryMap;
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};

pub struct SillyMemoryMapFrameAllocator<'a> {
    map: &'a MatrixMemoryMap,
    next: usize,
}

impl<'a> SillyMemoryMapFrameAllocator<'a> {
    pub fn new(map: &'a MatrixMemoryMap) -> Self {
        Self { map, next: 0 }
    }

    pub fn get_next_index(&self) -> usize {
        self.next
    }
}

unsafe impl FrameAllocator<Size4KiB> for SillyMemoryMapFrameAllocator<'_> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let res = self.map.frame_iter().nth(self.next);
        self.next += 1;

        res
    }
}
