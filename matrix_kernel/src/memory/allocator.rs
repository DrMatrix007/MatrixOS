use linked_list_allocator::LockedHeap;
use log::info;
use matrix_common::boot_info::memory_map::MatrixMemoryMap;
use spin::mutex::SpinMutex;
use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, FrameDeallocator, Mapper, Page, PageTableFlags, Size4KiB,
        mapper::MapToError, page::PageRangeInclusive,
    },
};

use crate::{
    memory::{memory_map_frame_allocator::MemoryMapPageAllocator, once_objects::OnceAllocator},
    memory_locations::{HEAP_SIZE, HEAP_START},
};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

static FRAME_ALLOCATOR: SpinMutex<OnceAllocator<MemoryMapPageAllocator, Size4KiB>> =
    SpinMutex::new(OnceAllocator::new());

pub(super) fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    memory_map: &'static MatrixMemoryMap,
) -> Result<(), MapToError<Size4KiB>> {
    let mut frame_allocator = FRAME_ALLOCATOR.lock();

    frame_allocator.init(MemoryMapPageAllocator::new(memory_map, mapper));

    info!("mapping heap");

    let page_range: PageRangeInclusive<Size4KiB> = {
        let heap_start = VirtAddr::new(HEAP_START);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            mapper
                .map_to(page, frame, flags, &mut *frame_allocator)?
                .flush()
        };
    }

    unsafe { ALLOCATOR.lock().init(HEAP_START as _, HEAP_SIZE as _) };

    Ok(())
}

pub struct KernelFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for KernelFrameAllocator {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        FRAME_ALLOCATOR.lock().allocate_frame()
    }
}

impl FrameDeallocator<Size4KiB> for KernelFrameAllocator {
    unsafe fn deallocate_frame(&mut self, frame: x86_64::structures::paging::PhysFrame<Size4KiB>) {
        unsafe { FRAME_ALLOCATOR.lock().deallocate_frame(frame) }
    }
}
