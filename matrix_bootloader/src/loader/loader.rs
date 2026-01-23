use core::ptr::copy;

use anyhow::{Context, Result, anyhow};
use bytemuck::{Pod, Zeroable};
use log::info;
use matrix_boot_args::MatrixEntryPoint;
use uefi::{
    boot::{MemoryType, PAGE_SIZE},
    println,
};

use crate::loader::elf::{
    header_raw::{ELF_MAGIC, ElfHeaderRaw},
    program_header_raw::ElfProgramHeaderRaw,
    section_header_raw::ElfSectionHeaderRaw,
};

fn parse_object<T: Pod + Zeroable>(file: &[u8], start: u64) -> Result<&T> {
    parse_objects(file, start, 1)?
        .get(0)
        .ok_or_else(|| anyhow!("object missing"))
}

fn parse_objects<T: Pod + Zeroable>(file: &[u8], start: u64, count: u64) -> Result<&[T]> {
    let start = start as usize;
    let end = start + size_of::<T>() * count as usize;
    let value_slice = file.get(start..end).ok_or_else(|| anyhow!("bad range"))?;

    bytemuck::try_cast_slice(value_slice).map_err(|e| anyhow!("bytemuck cast failed: {}", e))
}

pub fn load_elf(file: &[u8]) -> Result<MatrixEntryPoint> {
    let header = parse_object::<ElfHeaderRaw>(file, 0).context("getting the elf header")?;

    if ELF_MAGIC != header.magic {
        return Err(anyhow!("bad elf magic"));
    }
    info!(
        "got header: {:?} {:?} {:?} {:?} ",
        header.get_class(),
        header.get_osabi(),
        header.get_data_encoding(),
        header.get_type()
    );

    let x = header.e_entry;
    info!("entry point: 0x{:x}", x);

    let program_headers =
        parse_objects::<ElfProgramHeaderRaw>(file, header.e_phoff, header.e_phnum as u64)
            .context("getting the program header")?;

    let section_headers =
        parse_objects::<ElfSectionHeaderRaw>(file, header.e_shoff, header.e_shnum as u64)
            .context("getting the section headers")?;

    info!(
        "got {} program headers and {} section headers",
        program_headers.len(),
        section_headers.len()
    );

    let page_mask = PAGE_SIZE as u64 - 1;

    let min_vaddr: u64 = program_headers
        .iter()
        .map(|header| header.p_vaddr)
        .min()
        .context("finding minimum vaddr")?;

    let max_vaddr: u64 = program_headers
        .iter()
        .map(|header| header.p_vaddr)
        .max()
        .context("finding max vaddr")?;

    let aligned_min = min_vaddr & !page_mask;
    let total_size = (max_vaddr - aligned_min + page_mask) & !page_mask;

    println!("position: 0x{:x} size: 0x{:x}", aligned_min, total_size);

    let image_base = uefi::boot::allocate_pages(
        uefi::boot::AllocateType::AnyPages,
        MemoryType::BOOT_SERVICES_DATA,
        total_size as usize / PAGE_SIZE,
    )
    .context("allocating pages")?
    .as_ptr();

    for header in program_headers {
        let (vaddr, vsize) = (
            unsafe { image_base.add(header.p_vaddr as usize) },
            header.p_memsz,
        );

        let (faddr, fsize) = (header.p_offset, header.p_memsz);

        let data = file
            .get((faddr as usize)..(faddr as usize + fsize as usize))
            .context("getting the program header data from buffer")?;

        let size_to_copy = Ord::min(vsize, fsize);

        unsafe { copy(data.as_ptr(), vaddr, size_to_copy as usize) };
    }

    let entry: MatrixEntryPoint =
        unsafe { core::mem::transmute(image_base.add(header.e_entry as usize)) };

    

    Ok(entry)
}
