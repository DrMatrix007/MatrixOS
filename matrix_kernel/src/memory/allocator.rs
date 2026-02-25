use linked_list_allocator::LockedHeap;
use matrix_boot_args::memory_map::{MatrixMemoryMap, MatrixMemoryRegionKind};
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageSize, PageTableFlags, PhysFrame, Size4KiB,
        mapper::MapToError, page::PageRangeInclusive,
    },
};

struct MemoryMapPageAllocator {
    memory_map: &'static MatrixMemoryMap,
    next: usize,
}

impl MemoryMapPageAllocator {
    fn new(memory_map: &'static MatrixMemoryMap) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for MemoryMapPageAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let phys_addr = self
            .memory_map
            .get_slice()
            .iter()
            .filter(|region| matches!(region.kind, MatrixMemoryRegionKind::Usable))
            .flat_map(|region| {
                (region.phys_start
                    ..(region.phys_start + region.amount_of_4k_pages * Size4KiB::SIZE))
                    .step_by(Size4KiB::SIZE as _)
            })
            .nth(self.next)?;

        self.next += 1;

        Some(PhysFrame::containing_address(PhysAddr::new(phys_addr)))
    }
}

pub const HEAP_START: u64 = 0x_4444_4444_0000;
pub const HEAP_SIZE: u64 = 10 * 4096;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub(super) fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    memory_map: &'static MatrixMemoryMap,
) -> Result<(), MapToError<Size4KiB>> {
    let mut frame_allocator = MemoryMapPageAllocator::new(memory_map);

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
                .map_to(page, frame, flags, &mut frame_allocator)?
                .flush()
        };
    }

    unsafe { ALLOCATOR.lock().init(HEAP_START as _, HEAP_SIZE as _) };

    Ok(())
}
