use anyhow::Context;
use log::info;
use uefi::{
    boot::{self, MemoryType, PAGE_SIZE, memory_map},
    mem::memory_map::{MemoryMap, MemoryMapMut},
};
use x86_64::{
    PhysAddr, VirtAddr,
    registers::control::Cr3,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags,
        PhysFrame, Size2MiB, Size4KiB,
    },
};

use crate::elf_loader::loader::LoadedElf;

static UEFI_PHYS_OFFSET: u64 = 0;

struct UefiPageAllocator;

unsafe impl<Size: PageSize> FrameAllocator<Size> for UefiPageAllocator {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size>> {
        match boot::allocate_pages(
            boot::AllocateType::AnyPages,
            boot::MemoryType::LOADER_DATA,
            Size::SIZE as usize / PAGE_SIZE,
        )
        .ok()
        {
            Some(ptr) => Some(PhysFrame::containing_address(PhysAddr::new(
                ptr.as_ptr() as u64
            ))),
            None => None,
        }
    }
}

pub fn init_memory(physical_offset: u64, loaded_kernel: &LoadedElf, new_kernel_base: u64) {
    let mut pm4 = get_page_table();

    info!("before mapping stuff");

    map_kernel(
        &mut pm4,
        loaded_kernel.image_base,
        loaded_kernel.image_size,
        new_kernel_base,
    );

    info!("kenrel mapped");

    let mut memory_map = memory_map(MemoryType::LOADER_DATA)
        .context("getting the memory map for init the memory")
        .unwrap();

    memory_map.sort();

    let last_entry = memory_map.entries().last().unwrap();
    let phys_end = last_entry.phys_start + last_entry.page_count * PAGE_SIZE as u64;

    info!("size of memory: {}", phys_end);

    map_physical_memory_offset(&mut pm4, phys_end, physical_offset);
    info!("physical memory mapped");
}

fn map_kernel<M: Mapper<Size2MiB>>(
    mapper: &mut M,
    kernel_phys_start: u64,
    kernel_size: u64,
    new_kernel_base: u64,
) {
    let phys_start = PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(kernel_phys_start))
        .start_address()
        .as_u64();

    let phys_end = {
        let frame = PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(
            kernel_phys_start + kernel_size,
        ));
        frame.start_address().as_u64() + frame.size()
    };

    let mut offset = 0;

    while phys_start + offset < phys_end {
        let phys = phys_start + offset;
        let virt = new_kernel_base + offset;

        let frame = PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(phys));

        let page = Page::<Size2MiB>::containing_address(VirtAddr::new(virt));

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::GLOBAL;

        info!("mapping");

        unsafe {
            mapper
                .map_to(page, frame, flags, &mut UefiPageAllocator)
                .expect("kernel mapping failed")
                .flush();
        }

        offset += Size2MiB::SIZE;
    }
}

fn map_physical_memory_offset<M: Mapper<Size2MiB>>(
    mapper: &mut M,
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

        addr += Size2MiB::SIZE;
    }
}

fn get_page_table() -> OffsetPageTable<'static> {
    let (pm4_frame, _) = Cr3::read();

    let phys = pm4_frame.start_address();

    let page_table_virt_ptr = VirtAddr::new(phys.as_u64() + UEFI_PHYS_OFFSET);
    let page_table_ptr: *mut PageTable = page_table_virt_ptr.as_mut_ptr();

    unsafe { OffsetPageTable::new(&mut *page_table_ptr, VirtAddr::new(UEFI_PHYS_OFFSET)) }
}
