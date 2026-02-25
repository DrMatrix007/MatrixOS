mod mappings;

use anyhow::{Context, Result};
use log::info;
use uefi::{
    boot::{MemoryType, PAGE_SIZE, memory_map},
    mem::memory_map::{MemoryMap, MemoryMapMut, MemoryMapOwned},
};
use x86_64::structures::paging::{Size1GiB, Size4KiB};

use crate::{
    elf_loader::loader::LoadedElf,
    memory::mappings::{
        KernelPageTable, create_page_table, map_kernel, map_physical_memory_offset,
    },
};

pub fn create_kernel_page_table(
    physical_offset: u64,
    loaded_kernel: &LoadedElf,
    new_kernel_base: u64,
) -> Result<KernelPageTable<'static>> {
    let mut kernel_page_table = create_page_table();

    info!("before mapping stuff");

    map_kernel::<Size4KiB, _>(
        kernel_page_table.page_table_mut(),
        loaded_kernel.image_base,
        loaded_kernel.image_size,
        new_kernel_base,
    )
    .context("mapping kernel")?;

    info!("kenrel mapped");

    let mut memory_map = memory_map(MemoryType::LOADER_DATA)
        .context("getting the memory map for init the memory")
        .unwrap();

    memory_map.sort();

    let phys_end = get_max_phys(memory_map);

    info!("size of memory: 0x{:x}", phys_end);

    map_physical_memory_offset::<Size1GiB>(
        kernel_page_table.page_table_mut(),
        phys_end,
        physical_offset,
    );

    info!("mapped higher phsycial");
    map_physical_memory_offset::<Size1GiB>(kernel_page_table.page_table_mut(), phys_end, 0);
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
