use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PhysFrame},
};

pub const HEAP_START: u64 = 0x0000_0444_4444_0000;
pub const HEAP_SIZE: u64 = 1000 * 4096;

pub const FRAME_ALLOC_BITMAP_LOCATION: u64 = 0x0000_0555_5555_0000;

pub const PROCESS_CREATION_PAGE_MAP_BASE: VirtAddr = VirtAddr::new(0x1111_1111_0000);

pub const APIC_FRAME: PhysFrame = PhysFrame::containing_address(PhysAddr::new(0xFEE00000));
pub const APIC_PAGE: Page = Page::containing_address(VirtAddr::new(0xffff_8111_1111_0000));
