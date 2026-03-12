use spin::Mutex;
use x86_64::{
    VirtAddr,
    registers::control::Cr3,
    structures::paging::{Mapper, OffsetPageTable, Page, PageSize, PageTable, PhysFrame, Size4KiB},
};

use crate::memory::once_objects::OnceMapper;

pub static PAGE_TABLE: Mutex<OnceMapper<KernelPageTable>> = Mutex::new(OnceMapper::new());

pub fn init_paging(page_table: KernelPageTable<'static>) {
    PAGE_TABLE.lock().init(page_table);
}

/// # Safety
///
/// the phys_offset should be the physical offset that the physical frames are mapped to
///
pub unsafe fn get_page_table(phys_offset: VirtAddr) -> &'static mut PageTable {
    let (physical_frame, _flags) = Cr3::read();

    let phys = physical_frame.start_address();
    let virt = phys_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub struct KernelPageTable<'a> {
    page_table: OffsetPageTable<'a>,
    phys_frame: PhysFrame<Size4KiB>,
}

impl<'a> KernelPageTable<'a> {
    pub unsafe fn new(page_table: OffsetPageTable<'a>, phys_frame: PhysFrame<Size4KiB>) -> Self {
        Self {
            page_table,
            phys_frame,
        }
    }

    pub fn phys_offset(&self) -> VirtAddr {
        self.page_table.phys_offset()
    }

    fn is_active(&self) -> bool {
        Cr3::read().0 == self.phys_frame
    }
}

impl<'a, Size: PageSize> Mapper<Size> for KernelPageTable<'a>
where
    OffsetPageTable<'a>: Mapper<Size>,
{
    unsafe fn map_to_with_table_flags<A>(
        &mut self,
        page: Page<Size>,
        frame: PhysFrame<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
        parent_table_flags: x86_64::structures::paging::PageTableFlags,
        frame_allocator: &mut A,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlush<Size>,
        x86_64::structures::paging::mapper::MapToError<Size>,
    >
    where
        Self: Sized,
        A: x86_64::structures::paging::FrameAllocator<Size4KiB> + ?Sized,
    {
        unsafe {
            self.page_table.map_to_with_table_flags(
                page,
                frame,
                flags,
                parent_table_flags,
                frame_allocator,
            )
        }
    }

    fn unmap(
        &mut self,
        page: Page<Size>,
    ) -> Result<
        (
            PhysFrame<Size>,
            x86_64::structures::paging::mapper::MapperFlush<Size>,
        ),
        x86_64::structures::paging::mapper::UnmapError,
    > {
        self.page_table.unmap(page)
    }

    unsafe fn update_flags(
        &mut self,
        page: Page<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlush<Size>,
        x86_64::structures::paging::mapper::FlagUpdateError,
    > {
        unsafe { self.page_table.update_flags(page, flags) }
    }

    unsafe fn set_flags_p4_entry(
        &mut self,
        page: Page<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlushAll,
        x86_64::structures::paging::mapper::FlagUpdateError,
    > {
        unsafe { self.page_table.set_flags_p4_entry(page, flags) }
    }

    unsafe fn set_flags_p3_entry(
        &mut self,
        page: Page<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlushAll,
        x86_64::structures::paging::mapper::FlagUpdateError,
    > {
        unsafe { self.page_table.set_flags_p3_entry(page, flags) }
    }

    unsafe fn set_flags_p2_entry(
        &mut self,
        page: Page<Size>,
        flags: x86_64::structures::paging::PageTableFlags,
    ) -> Result<
        x86_64::structures::paging::mapper::MapperFlushAll,
        x86_64::structures::paging::mapper::FlagUpdateError,
    > {
        unsafe { self.page_table.set_flags_p2_entry(page, flags) }
    }

    fn translate_page(
        &self,
        page: Page<Size>,
    ) -> Result<PhysFrame<Size>, x86_64::structures::paging::mapper::TranslateError> {
        self.page_table.translate_page(page)
    }
}
