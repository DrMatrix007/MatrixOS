use uefi::boot::{self, PAGE_SIZE};
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        FrameAllocator, OffsetPageTable, PageSize, PageTable, PhysFrame, Size4KiB,
    },
};

static UEFI_PHYS_OFFSET: u64 = 0;

pub(crate) struct UefiPageAllocator;

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
