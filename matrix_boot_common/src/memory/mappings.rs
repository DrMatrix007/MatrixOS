use core::fmt::Debug;

use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageSize, PageTableFlags, PhysFrame, Size4KiB,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MappingKernelError {
    BadOffsets { phys_offset: u64, new_offset: u64 },
}

pub fn map_kernel_to_mapper<Size: PageSize + Debug>(
    kernel_phys_start: u64,
    kernel_size: u64,
    new_kernel_base: u64,
    mapper: &mut impl Mapper<Size>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MappingKernelError> {
    let phys_offset = kernel_phys_start % Size::SIZE;
    let new_offset = new_kernel_base % Size::SIZE;

    if kernel_phys_start % Size::SIZE != new_kernel_base % Size::SIZE {
        return Err(MappingKernelError::BadOffsets {
            phys_offset,
            new_offset,
        });
    }

    let phys_start = PhysFrame::<Size>::containing_address(PhysAddr::new(kernel_phys_start))
        .start_address()
        .as_u64();

    let phys_end = {
        let frame = PhysFrame::<Size>::containing_address(PhysAddr::new(
            kernel_phys_start + kernel_size - 1,
        ));
        frame.start_address().as_u64() + frame.size()
    };

    let mut offset = 0;

    while phys_start + offset < phys_end {
        let phys = phys_start + offset;
        let virt = new_kernel_base + offset;

        let frame = PhysFrame::containing_address(PhysAddr::new(phys));

        let page = Page::containing_address(VirtAddr::new(virt));

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            mapper
                .map_to(page, frame, flags, frame_allocator)
                .expect("kernel mapping failed")
                .flush();
        }

        offset += Size::SIZE;
    }

    Ok(())
}

pub fn map_physical_memory_offset<Size: PageSize + Debug>(
    phys_end: u64,
    phys_offset: u64,
    mapper: &mut impl Mapper<Size>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let mut addr = 0;

    while addr < phys_end {
        let frame = PhysFrame::containing_address(PhysAddr::new(addr));

        let virt = phys_offset + addr;
        let page = Page::containing_address(VirtAddr::new(virt));

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE;

        unsafe {
            mapper
                .map_to(page, frame, flags, frame_allocator)
                .expect("mapping failed")
                .flush();
        }

        addr += Size::SIZE;
    }
}
