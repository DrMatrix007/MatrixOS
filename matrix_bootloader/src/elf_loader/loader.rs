use alloc::slice;
use anyhow::{Context, Result, anyhow};
use bytemuck::{Pod, Zeroable};
use log::info;

use matrix_boot_args::MatrixEntryPoint;
use uefi::boot::{MemoryType, PAGE_SIZE};

use crate::elf_loader::elf::{
    header_raw::{ELF_MAGIC, ElfHeaderRaw},
    program_header_raw::{ElfProgramHeaderRaw, ElfProgramHeaderType},
    section_header_raw::ElfSectionHeaderRaw,
};

fn parse_object<T: Pod + Zeroable>(file: &[u8], start: u64) -> Result<&T> {
    parse_objects(file, start, 1)?
        .first()
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

    let total_size = calc_size(program_headers)?;

    let entry = allocate_elf(file, header, program_headers, total_size)?;


    for section_header in section_headers {
        info!("section {:?}", section_header.get_type());
    }


    Ok(entry)
}

fn calc_size(program_headers: &[ElfProgramHeaderRaw]) -> Result<u64, anyhow::Error> {
    static PAGE_MASK: u64 = PAGE_SIZE as u64 - 1;

    let min_vaddr: u64 = program_headers
        .iter()
        .map(|header| header.p_vaddr)
        .min()
        .context("finding minimum vaddr")?;

    let max_vaddr: u64 = program_headers
        .iter()
        .map(|header| header.p_vaddr + header.p_memsz)
        .max()
        .context("finding max vaddr")?;

    let aligned_min = min_vaddr & !PAGE_MASK;
    let total_size = (max_vaddr - aligned_min + PAGE_MASK) & !PAGE_MASK;

    Ok(total_size)
}

fn allocate_elf(
    file: &[u8],
    header: &ElfHeaderRaw,
    program_headers: &[ElfProgramHeaderRaw],
    total_size: u64,
) -> Result<MatrixEntryPoint, anyhow::Error> {
    let image_base_raw = uefi::boot::allocate_pages(
        uefi::boot::AllocateType::AnyPages,
        MemoryType::BOOT_SERVICES_DATA,
        total_size as usize / PAGE_SIZE,
    )
    .context("allocating pages")?
    .as_ptr();
    let image = unsafe { slice::from_raw_parts_mut(image_base_raw, total_size as usize) };
    
    image.fill(0);
    
    for header in program_headers.iter().filter(|header| {
        matches!(header.get_type(), Ok(ElfProgramHeaderType::Load)) && header.get_flags().is_ok()
    }) {
        let (vaddr, vsize) = (header.p_vaddr, header.p_memsz);

        let (faddr, fsize) = (header.p_offset, header.p_filesz);

        let size_to_copy = Ord::min(fsize, vsize);

        let file_segment = file
            .get(faddr as usize..(faddr + size_to_copy) as usize)
            .context("getting the program header file buffer range")?;
        let image_segment = image
            .get_mut(vaddr as usize..(vaddr + size_to_copy) as usize)
            .context("getting the program header image buffer range")?;
        image_segment.copy_from_slice(file_segment);
    }
    let entry: MatrixEntryPoint = unsafe {
        core::mem::transmute(
            (parse_object::<u64>(image, header.e_entry)).context("cant get entry")?,
        )
    };

    info!("parsed the elf successfuly into 0x{:x}", image_base_raw as usize);

    Ok(entry)
}
