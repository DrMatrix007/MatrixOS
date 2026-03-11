use alloc::format;
use anyhow::{Context, Result, anyhow};
use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
    },
};

use crate::{
    memory::{PAGE_TABLE, allocator::FRAME_ALLOCATOR},
    memory_locations::PROCESS_CREATION_PAGE_MAP_BASE,
};

pub struct ProcessPageTable {
    page_table_frame: PhysFrame<Size4KiB>,
}

impl ProcessPageTable {
    pub fn new(m: &mut impl Mapper<Size4KiB>) -> Result<Self> {
        let mut frame_allocator = FRAME_ALLOCATOR.lock();
        let mut current_page_table = PAGE_TABLE.lock();

        let new_page_table_frame = frame_allocator
            .allocate_frame()
            .context("allocating frame for the new process's pagetable")?;

        let this = Self {
            page_table_frame: new_page_table_frame,
        };

        let page_table = this.map_self(
            PROCESS_CREATION_PAGE_MAP_BASE,
            &mut *current_page_table,
            &mut *frame_allocator,
        )?;

        *page_table = PageTable::new();

        this.unmap_self(&mut *current_page_table, page_table)?;

        Ok()
    }
}

impl Drop for ProcessPageTable {
    fn drop(&mut self) {
        
    }
}

impl ProcessPageTable {
    pub fn map_self<'a>(
        &'a self,
        virt_address: VirtAddr,
        m: &mut impl Mapper<Size4KiB>,
        frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ) -> Result<&'a mut PageTable> {
        let new_page_table_page = Page::<Size4KiB>::containing_address(virt_address);

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

        Ok(unsafe { &mut *new_page_table_page.start_address().as_mut_ptr() })
    }

    pub fn unmap_self<'a>(
        &'a self,
        mapper: &mut impl Mapper<Size4KiB>,
        page_table: &'a mut PageTable,
    ) -> Result<()> {
        Ok(())
    }
}
