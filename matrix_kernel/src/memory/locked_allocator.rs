use core::marker::PhantomData;

use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, PageSize};

pub struct LockedAllocator<Allocator: FrameAllocator<Size> + FrameDeallocator<Size>, Size: PageSize>
{
    allocator: Option<Allocator>,
    _marker: PhantomData<Size>,
}

impl<Allocator: FrameAllocator<Size> + FrameDeallocator<Size>, Size: PageSize>
    LockedAllocator<Allocator, Size>
{
    pub const fn new() -> Self {
        Self {
            allocator: None,
            _marker: PhantomData,
        }
    }

    pub fn init(&mut self, allocator: Allocator) {
        self.allocator = Some(allocator);
    }
}

unsafe impl<Allocator: FrameAllocator<Size> + FrameDeallocator<Size>, Size: PageSize>
    FrameAllocator<Size> for LockedAllocator<Allocator, Size>
{
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size>> {
        self.allocator
            .as_mut()
            .expect("empty locked frame allocator")
            .allocate_frame()
    }
}

impl<Allocator: FrameAllocator<Size> + FrameDeallocator<Size>, Size: PageSize>
    FrameDeallocator<Size> for LockedAllocator<Allocator, Size>
{
    unsafe fn deallocate_frame(&mut self, frame: x86_64::structures::paging::PhysFrame<Size>) {
        unsafe {
            self.allocator
                .as_mut()
                .expect("empty locked frame allocator")
                .deallocate_frame(frame)
        }
    }
}
