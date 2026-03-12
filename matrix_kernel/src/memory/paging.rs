use spin::Mutex;
use x86_64::{
    VirtAddr,
    registers::control::Cr3,
    structures::paging::{Mapper, OffsetPageTable, Page, PageSize, PageTable, PhysFrame, Size4KiB},
};

use crate::memory::once_objects::OnceMapper;

pub static PAGE_TABLE: Mutex<OnceMapper<OffsetPageTable>> = Mutex::new(OnceMapper::new());

pub fn init_paging(page_table: OffsetPageTable<'static>) {
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

pub struct KernelMapper<'a> {
    page_table: OffsetPageTable<'a>,
    phys_frame: PhysFrame<Size4KiB>,
}

impl KernelMapper<'_> {
    fn phys_offset(&self) -> VirtAddr {
        self.phys_offset()
    }

    fn is_active(&self) -> bool {
        let Ok(page) = Page::<Size4KiB>::from_start_address(VirtAddr::new(self.level_4_table()
            as *const PageTable
            as u64))
        else {
            return false;
        };

        let Ok(frame) = Mapper::<Size4KiB>::translate_page(self, page) else {
            return false;
        };

        Cr3::read().0 == frame
    }
}
