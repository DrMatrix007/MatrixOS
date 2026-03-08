mod mappings;

use anyhow::{Context, Result, anyhow};
use log::info;
use matrix_boot_common::memory::mappings::{map_kernel_to_mapper, map_physical_memory_offset};
use uefi::{
    boot::{MemoryType, PAGE_SIZE, memory_map},
    mem::memory_map::{MemoryMap, MemoryMapMut, MemoryMapOwned},
};
use x86_64::structures::paging::{Size1GiB, Size4KiB};

use crate::{
    elf_loader::loader::LoadedElf,
    memory::mappings::{KernelPageTable, UefiPageAllocator, create_page_table},
};

pub fn create_kernel_page_table(
    physical_offset: u64,
    loaded_kernel: &LoadedElf,
    new_kernel_base: u64,
) -> Result<KernelPageTable<'static>> {
    let mut kernel_page_table = create_page_table();

    info!("before mapping stuff");

    map_kernel_to_mapper::<Size4KiB>(
        loaded_kernel.image_base,
        loaded_kernel.image_size,
        new_kernel_base,
        kernel_page_table.page_table_mut(),
        &mut UefiPageAllocator,
    )
    .map_err(|x| anyhow!("{:?}", x))
    .context("mapping kernel")?;

    info!("kenrel mapped");

    let mut memory_map = memory_map(MemoryType::LOADER_DATA)
        .context("getting the memory map for init the memory")
        .unwrap();

    memory_map.sort();

    let phys_end = get_max_phys(memory_map);

    info!("size of memory: 0x{:x}", phys_end);

    map_physical_memory_offset::<Size1GiB>(
        phys_end,
        physical_offset,
        kernel_page_table.page_table_mut(),
        &mut UefiPageAllocator,
    );

    info!("mapped higher phsycial");
    map_physical_memory_offset::<Size1GiB>(
        phys_end,
        0,
        kernel_page_table.page_table_mut(),
        &mut UefiPageAllocator,
    );
    
    info!("mapped identity");

    info!("physical memory mapped");

    Ok(kernel_page_table)
}

fn get_max_phys(memory_map: MemoryMapOwned) -> u64 {
    memory_map
        .entries()
        .map(|e| e.phys_start + e.page_count * PAGE_SIZE as u64)
        .max()
        .unwrap()
}
