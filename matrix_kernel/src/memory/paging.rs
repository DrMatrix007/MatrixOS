use x86_64::{registers::control::Cr3, structures::paging::PageTable};


pub unsafe fn get_page_table() -> &mut PageTable {
    let (physical_frame,_flags) = Cr3::read();
}