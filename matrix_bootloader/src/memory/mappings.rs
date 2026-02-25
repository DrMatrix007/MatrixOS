use core::fmt::Debug;

use anyhow::{Result, anyhow};
use uefi::boot::{self, PAGE_SIZE};
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags,
        PhysFrame, Size4KiB,
    },
};

static UEFI_PHYS_OFFSET: u64 = 0;

struct UefiPageAllocator;

unsafe impl<Size: PageSize> FrameAllocator<Size> for UefiPageAllocator {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size>> {
        boot::allocate_pages(
            boot::AllocateType::AnyPages,
            boot::MemoryType::LOADER_DATA,
            Size::SIZE as usize / PAGE_SIZE,
        )
        .ok()
        .map(|ptr| PhysFrame::containing_address(PhysAddr::new(ptr.as_ptr() as u64)))
    }
}

pub(crate) fn map_kernel<Size: PageSize + Debug, M: Mapper<Size>>(
    mapper: &mut M,
    kernel_phys_start: u64,
    kernel_size: u64,
    new_kernel_base: u64,
) -> Result<()> {
    let phys_offset = kernel_phys_start % Size::SIZE;
    let new_offset = new_kernel_base % Size::SIZE;

    if kernel_phys_start % Size::SIZE != new_kernel_base % Size::SIZE {
        return Err(anyhow!(
            "Page offsets do not match: kernel_phys_start % SIZE = {}, new_kernel_base % SIZE = {}",
            phys_offset,
            new_offset
        ));
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
                .map_to(page, frame, flags, &mut UefiPageAllocator)
                .expect("kernel mapping failed")
                .flush();
        }

        offset += Size::SIZE;
    }

    Ok(())
}

pub(crate) fn map_physical_memory_offset<Size: PageSize + Debug>(
    mapper: &mut impl Mapper<Size>,
    phys_end: u64,
    phys_offset: u64,
) {
    let mut addr = 0;

    while addr < phys_end {
        let frame = PhysFrame::containing_address(PhysAddr::new(addr));

        let virt = phys_offset + addr;
        let page = Page::containing_address(VirtAddr::new(virt));

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            mapper
                .map_to(page, frame, flags, &mut UefiPageAllocator)
                .expect("mapping failed")
                .flush();
        }

        addr += Size::SIZE;
    }
}

pub fn create_page_table() -> KernelPageTable<'static> {
    let pm4_frame: PhysFrame<Size4KiB> =
        <UefiPageAllocator as FrameAllocator<Size4KiB>>::allocate_frame(&mut UefiPageAllocator)
            .unwrap();

    let phys = pm4_frame.start_address();

    let page_table_virt_ptr = VirtAddr::new(phys.as_u64() + UEFI_PHYS_OFFSET);
    let page_table_ptr: *mut PageTable = page_table_virt_ptr.as_mut_ptr();
    unsafe { page_table_ptr.write(PageTable::new()) };

    KernelPageTable {
        page_table: unsafe {
            OffsetPageTable::new(&mut *page_table_ptr, VirtAddr::new(UEFI_PHYS_OFFSET))
        },
        page_table_frame: pm4_frame,
    }
}

pub struct KernelPageTable<'a> {
    page_table: OffsetPageTable<'a>,
    page_table_frame: PhysFrame<Size4KiB>,
}

impl<'a> KernelPageTable<'a> {
    /// # Safety
    ///
    /// changes the current page table
    /// if the current code is not mapped, the cpu tries to execute unmapped pages
    ///
    pub unsafe fn apply(&self) {
        unsafe {
            use x86_64::registers::control::{Cr3, Cr3Flags};
            Cr3::write(self.page_table_frame, Cr3Flags::empty());
        }
    }

    pub fn page_table(&self) -> &OffsetPageTable<'a> {
        &self.page_table
    }

    pub fn page_table_mut(&mut self) -> &mut OffsetPageTable<'a> {
        &mut self.page_table
    }
}
