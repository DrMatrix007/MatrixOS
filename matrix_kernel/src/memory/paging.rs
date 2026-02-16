use x86_64::{VirtAddr, registers::control::Cr3, structures::paging::PageTable};

pub unsafe fn get_page_table(phys_offset: VirtAddr) -> &'static mut PageTable {
    let (physical_frame, _flags) = Cr3::read();

    let phys = physical_frame.start_address();
    let virt = phys_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}
