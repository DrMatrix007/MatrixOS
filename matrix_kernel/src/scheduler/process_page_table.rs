use core::ops::{Deref, DerefMut};

use alloc::format;
use anyhow::{Context, Result, anyhow};
use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, FrameDeallocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags,
        PhysFrame, Size4KiB,
    },
};

use crate::{
    memory::{allocator::KernelFrameAllocator, paging::KernelPageTable},
    memory_locations::PROCESS_CREATION_PAGE_MAP_BASE,
};

pub struct ProcessPageTable {
    page_table_frame: PhysFrame<Size4KiB>,
}

impl ProcessPageTable {
    pub fn new(current_mapper: &mut KernelPageTable) -> Result<Self> {
        let mut frame_allocator = KernelFrameAllocator;

        let new_page_table_frame = frame_allocator
            .allocate_frame()
            .context("allocating frame for the new process's pagetable")?;

        let this = Self {
            page_table_frame: new_page_table_frame,
        };
        {
            let phys_offset = current_mapper.phys_offset();
            let mut page_table = this.map_self(current_mapper, &mut frame_allocator)?;

            let _page_table = unsafe { OffsetPageTable::new(&mut *page_table, phys_offset) };
        }

        Ok(this)
    }
}

impl Drop for ProcessPageTable {
    fn drop(&mut self) {
        unsafe { KernelFrameAllocator.deallocate_frame(self.page_table_frame) };
    }
}

impl ProcessPageTable {
    pub fn map_self<'a, 'b>(
        &'b self,
        m: &'a mut impl Mapper<Size4KiB>,
        frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ) -> Result<ScopedPageTableMapping<'a, 'b>> {
        let new_page_table_page =
            Page::<Size4KiB>::containing_address(PROCESS_CREATION_PAGE_MAP_BASE);

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            m.map_to(
                new_page_table_page,
                self.page_table_frame,
                flags,
                frame_allocator,
            )
            .map_err(|x| anyhow!("{:?}", x))
            .with_context(|| {
                format!(
                    "mapping process page {:?} table to frame {:?}",
                    new_page_table_page, self.page_table_frame
                )
            })?
            .flush();
        };

        let page_table = unsafe { &mut *new_page_table_page.start_address().as_mut_ptr() };

        Ok(ScopedPageTableMapping::new(m, page_table))
    }
}

pub struct ScopedPageTableMapping<'a, 'b> {
    mapper: &'a mut dyn Mapper<Size4KiB>,
    page_table: &'b mut PageTable,
}

impl<'a, 'b> Drop for ScopedPageTableMapping<'a, 'b> {
    fn drop(&mut self) {
        let page_table_page = Page::<Size4KiB>::containing_address(VirtAddr::new(
            self.page_table as *mut PageTable as u64,
        ));

        let res = self.mapper.unmap(page_table_page);

        if let Err(err) = res {
            panic!("fails to unmap page! {:?}", err);
        }
    }
}

impl<'a, 'b> ScopedPageTableMapping<'a, 'b> {
    pub fn new(mapper: &'a mut dyn Mapper<Size4KiB>, page_table: &'b mut PageTable) -> Self {
        Self { mapper, page_table }
    }
}

impl<'a, 'b> Deref for ScopedPageTableMapping<'a, 'b> {
    type Target = PageTable;

    fn deref(&self) -> &Self::Target {
        self.page_table
    }
}

impl<'a, 'b> DerefMut for ScopedPageTableMapping<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.page_table
    }
}
