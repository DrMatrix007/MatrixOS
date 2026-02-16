use core::fmt::Debug;

use anyhow::Context;
use log::info;
use uefi::{
    boot::{self, MemoryType, PAGE_SIZE, memory_map},
    mem::memory_map::{MemoryMap, MemoryMapMut, MemoryMapOwned},
};
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags,
        PhysFrame, Size1GiB, Size2MiB, Size4KiB,
    },
};

use crate::elf_loader::loader::LoadedElf;

static UEFI_PHYS_OFFSET: u64 = 0;

type MappingPageSize = Size2MiB;

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
}

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

pub fn create_kernel_page_table(
    physical_offset: u64,
    loaded_kernel: &LoadedElf,
    new_kernel_base: u64,
) -> KernelPageTable<'static> {
    let mut kernel_page_table = create_page_table();

    info!("before mapping stuff");

    map_kernel::<Size4KiB, _>(
        &mut kernel_page_table.page_table,
        loaded_kernel.image_base,
        loaded_kernel.image_size,
        new_kernel_base,
    );

    info!("kenrel mapped");

    let mut memory_map = memory_map(MemoryType::LOADER_DATA)
        .context("getting the memory map for init the memory")
        .unwrap();

    memory_map.sort();

    let phys_end = get_max_phys(memory_map);

    info!("size of memory: {}", phys_end);

    map_physical_memory_offset::<Size1GiB>(
        &mut kernel_page_table.page_table,
        phys_end,
        physical_offset,
    );

    info!("mapped higher phsycial");
    map_physical_memory_offset::<Size1GiB>(&mut kernel_page_table.page_table, phys_end, 0);
    info!("mapped identity");

    info!("physical memory mapped");

    kernel_page_table
}

fn get_max_phys(memory_map: MemoryMapOwned) -> u64 {
    let last_entry = memory_map.entries().last().unwrap();
    last_entry.phys_start + last_entry.page_count * PAGE_SIZE as u64
}

fn map_kernel<Size: PageSize + Debug, M: Mapper<Size>>(
    mapper: &mut M,
    kernel_phys_start: u64,
    kernel_size: u64,
    new_kernel_base: u64,
) {
    let phys_start = PhysFrame::<Size>::containing_address(PhysAddr::new(kernel_phys_start))
        .start_address()
        .as_u64();

    let phys_end = {
        let frame = PhysFrame::<MappingPageSize>::containing_address(PhysAddr::new(
            kernel_phys_start + kernel_size,
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

        offset += page.size();
    }
}

fn map_physical_memory_offset<Size: PageSize + Debug>(
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

fn create_page_table() -> KernelPageTable<'static> {
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
