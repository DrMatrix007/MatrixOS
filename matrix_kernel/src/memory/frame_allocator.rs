use matrix_boot_common::boot_info::memory_map::MatrixMemoryMap;
use x86_64::VirtAddr;
use x86_64::structures::paging::FrameDeallocator;
use x86_64::structures::paging::Page;
use x86_64::structures::paging::PageSize;
use x86_64::structures::paging::PhysFrame;

use x86_64::structures::paging::FrameAllocator;

use x86_64::structures::paging::PageTableFlags;
use x86_64::structures::paging::Size4KiB;

use crate::memory::memory_locations::FRAME_ALLOC_BITMAP_LOCATION;
use crate::memory::silly_frame_allocator::SillyMemoryMapFrameAllocator;
use x86_64::structures::paging::Mapper;

static BITS_IN_U64: usize = 64;

pub(crate) struct MemoryMapPageAllocator {
    pub(crate) bitmap: &'static mut [u64],
    pub(crate) memory_map: &'static MatrixMemoryMap,
    pub(crate) next: usize,
}

impl MemoryMapPageAllocator {
    pub(crate) fn new(
        memory_map: &'static MatrixMemoryMap,
        mapper: &mut impl Mapper<Size4KiB>,
    ) -> Self {
        let mut silly = SillyMemoryMapFrameAllocator::new(memory_map);

        let pages_count = memory_map.frame_iter().count();

        let bytes_needed = pages_count / BITS_IN_U64 as usize;
        let pages_needed = bytes_needed / Size4KiB::SIZE as usize;

        let pages = {
            let start_page =
                Page::<Size4KiB>::containing_address(VirtAddr::new(FRAME_ALLOC_BITMAP_LOCATION));

            let end_page = start_page + pages_needed as _;
            Page::range_inclusive(start_page, end_page)
        };

        for page in pages {
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

            let frame = silly.allocate_frame().unwrap();

            unsafe {
                mapper
                    .map_to(page, frame, flags, &mut silly)
                    .unwrap()
                    .flush();
            };
        }

        let mut res = Self {
            bitmap: unsafe {
                core::slice::from_raw_parts_mut(FRAME_ALLOC_BITMAP_LOCATION as _, bytes_needed)
            },
            memory_map,
            next: 0,
        };

        for index in 0..silly.get_next_index() {
            res.set_present(index, true);
        }

        res
    }

    pub fn is_present(&self, page_index: usize) -> bool {
        let byte_index = page_index / BITS_IN_U64;
        let bit_index = page_index % BITS_IN_U64;

        (self.bitmap[byte_index as usize] & (1 << bit_index)) != 0
    }

    pub fn set_present(&mut self, page_index: usize, is_present: bool) {
        let byte_index = page_index / BITS_IN_U64;
        let bit_index = page_index % BITS_IN_U64;

        let val = &mut self.bitmap[byte_index as usize];

        if is_present {
            *val |= 1u64 << bit_index;
        } else {
            *val &= !(1u64 << bit_index);
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for MemoryMapPageAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let pages = self.memory_map.frame_iter();

        for (index, frame) in pages.enumerate().skip(self.next) {
            if !self.is_present(index) {
                self.set_present(index, true);
                self.next = index + 1;

                return Some(frame);
            }
        }

        None
    }
}

impl FrameDeallocator<Size4KiB> for MemoryMapPageAllocator {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        for (index, candidate) in self.memory_map.frame_iter().enumerate() {
            if candidate == frame {
                self.set_present(index, false);

                if index < self.next {
                    self.next = index;
                }

                return;
            }
        }

        panic!("this frame ({:?}) is not managed by the allocator", frame);
    }
}
