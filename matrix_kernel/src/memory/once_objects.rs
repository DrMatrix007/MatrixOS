use core::marker::PhantomData;

use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, Mapper, PageSize};

pub struct OnceAllocator<Allocator: FrameAllocator<Size> + FrameDeallocator<Size>, Size: PageSize> {
    allocator: Option<Allocator>,
    _marker: PhantomData<Size>,
}

impl<Allocator: FrameAllocator<Size> + FrameDeallocator<Size>, Size: PageSize>
    OnceAllocator<Allocator, Size>
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
    FrameAllocator<Size> for OnceAllocator<Allocator, Size>
{
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size>> {
        self.allocator
            .as_mut()
            .expect("empty locked frame allocator")
            .allocate_frame()
    }
}

impl<Allocator: FrameAllocator<Size> + FrameDeallocator<Size>, Size: PageSize>
    FrameDeallocator<Size> for OnceAllocator<Allocator, Size>
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

pub struct OnceMapper<M> {
    mapper: Option<M>,
}

impl<M> OnceMapper<M> {
    pub const fn new() -> Self {
        Self { mapper: None }
    }

    pub fn init(&mut self, mapper: M) {
        self.mapper = Some(mapper)
    }

    pub fn inner(&self) -> &M {
        self.mapper.as_ref().expect("empty mapper for `inner`")
    }
}

impl<M: Mapper<Size>, Size: PageSize> Mapper<Size> for OnceMapper<M> {
    unsafe fn map_to_with_table_flags<A>(
        &mut self,
        page: x86_64::structures::paging::Page<Size>,
        frame: x86_64::structures::paging::PhysFrame<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
        parent_table_flags: x86_64::structures::paging::PageTableFlags,
        frame_allocator: &mut A,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlush<Size>,
        x86_64::structures::paging::mapper::MapToError<Size>,
    >
    where
        Self: Sized,
        A: FrameAllocator<x86_64::structures::paging::Size4KiB> + ?Sized,
    {
        unsafe {
            self.mapper
                .as_mut()
                .expect("missing mapper")
                .map_to_with_table_flags(page, frame, flags, parent_table_flags, frame_allocator)
        }
    }

    fn unmap(
        &mut self,
        page: x86_64::structures::paging::Page<Size>,
    ) -> Result<
        (
            x86_64::structures::paging::PhysFrame<Size>,
            x86_64::structures::paging::mapper::MapperFlush<Size>,
        ),
        x86_64::structures::paging::mapper::UnmapError,
    > {
        self.mapper.as_mut().expect("missing mapper").unmap(page)
    }

    unsafe fn update_flags(
        &mut self,
        page: x86_64::structures::paging::Page<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlush<Size>,
        x86_64::structures::paging::mapper::FlagUpdateError,
    > {
        unsafe {
            self.mapper
                .as_mut()
                .expect("missing mapper")
                .update_flags(page, flags)
        }
    }

    unsafe fn set_flags_p4_entry(
        &mut self,
        page: x86_64::structures::paging::Page<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlushAll,
        x86_64::structures::paging::mapper::FlagUpdateError,
    > {
        unsafe {
            self.mapper
                .as_mut()
                .expect("missing mapper")
                .set_flags_p4_entry(page, flags)
        }
    }

    unsafe fn set_flags_p3_entry(
        &mut self,
        page: x86_64::structures::paging::Page<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlushAll,
        x86_64::structures::paging::mapper::FlagUpdateError,
    > {
        unsafe {
            self.mapper
                .as_mut()
                .expect("missing mapper")
                .set_flags_p3_entry(page, flags)
        }
    }

    unsafe fn set_flags_p2_entry(
        &mut self,
        page: x86_64::structures::paging::Page<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlushAll,
        x86_64::structures::paging::mapper::FlagUpdateError,
    > {
        unsafe {
            self.mapper
                .as_mut()
                .expect("missing mapper")
                .set_flags_p2_entry(page, flags)
        }
    }

    fn translate_page(
        &self,
        page: x86_64::structures::paging::Page<Size>,
    ) -> Result<
        x86_64::structures::paging::PhysFrame<Size>,
        x86_64::structures::paging::mapper::TranslateError,
    > {
        self.mapper
            .as_ref()
            .expect("missing mapper")
            .translate_page(page)
    }
}
