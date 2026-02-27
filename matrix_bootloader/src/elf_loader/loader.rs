use alloc::slice;
use anyhow::{Context, Result, anyhow};
use bytemuck::{Pod, Zeroable};
use log::info;

use matrix_boot_common::boot_info::MatrixEntryPointRaw;
use uefi::boot::{MemoryType, PAGE_SIZE};

use crate::elf_loader::elf::{
    header_raw::{ELF_MAGIC, ElfHeaderRaw},
    program_header_raw::{ElfProgramHeaderRaw, ElfProgramHeaderType},
    section_header_raw::{Elf64Rela, ElfSectionHeaderRaw, ElfSectionType},
};

pub struct LoadedElf {
    pub entry: MatrixEntryPointRaw,
    pub image_base: u64,
    pub image_size: u64,
}

fn read_object_mut<T: Pod + Zeroable>(data: &mut [u8], start: u64) -> Result<&mut T> {
    read_objects_mut(data, start, 1)?
        .first_mut()
        .ok_or_else(|| anyhow!("object missing"))
}

fn read_objects_mut<T: Pod + Zeroable>(
    file: &mut [u8],
    start: u64,
    count: u64,
) -> Result<&mut [T]> {
    let start = start as usize;
    let end = start + size_of::<T>() * count as usize;
    let value_slice = file
        .get_mut(start..end)
        .ok_or_else(|| anyhow!("bad range"))?;

    bytemuck::try_cast_slice_mut(value_slice).map_err(|e| anyhow!("bytemuck cast failed: {}", e))
}

fn read_object<T: Pod + Zeroable>(file: &[u8], start: u64) -> Result<&T> {
    read_objects(file, start, 1)?
        .first()
        .ok_or_else(|| anyhow!("object missing"))
}

fn read_objects<T: Pod + Zeroable>(file: &[u8], start: u64, count: u64) -> Result<&[T]> {
    let start = start as usize;
    let end = start + size_of::<T>() * count as usize;
    let value_slice = file.get(start..end).ok_or_else(|| anyhow!("bad range"))?;

    bytemuck::try_cast_slice(value_slice).map_err(|e| anyhow!("bytemuck cast failed: {}", e))
}

pub fn load_elf(file: &[u8], relocation_target: u64) -> Result<LoadedElf> {
    let header = read_object::<ElfHeaderRaw>(file, 0).context("getting the elf header")?;

    if ELF_MAGIC != header.magic {
        return Err(anyhow!("bad elf magic"));
    }

    let program_headers =
        read_objects::<ElfProgramHeaderRaw>(file, header.e_phoff, header.e_phnum as u64)
            .context("getting the program header")?;

    let section_headers =
        read_objects::<ElfSectionHeaderRaw>(file, header.e_shoff, header.e_shnum as u64)
            .context("getting the section headers")?;

    let total_size = calc_size(program_headers)?;

    let image = allocate_elf(file, program_headers, total_size)?;

    fix_reloactions(file, section_headers, image, relocation_target)?;

    let entry_raw = MatrixEntryPointRaw::new(header.e_entry);

    info!("kernel entry point: {:#x}", entry_raw.entry() as usize);

    Ok(LoadedElf {
        entry: entry_raw,
        image_base: image.as_ptr() as u64,
        image_size: image.len() as u64,
    })
}

fn fix_reloactions(
    file: &[u8],
    section_headers: &[ElfSectionHeaderRaw],
    image: &mut [u8],
    relocation_target: u64,
) -> Result<(), anyhow::Error> {
    for section_header in section_headers
        .iter()
        .filter(|section_header| matches!(section_header.get_type(), Ok(ElfSectionType::Rela)))
        .filter(|section_header| section_header.sh_addr != 0)
    {
        let relocations = section_header.sh_size / core::mem::size_of::<Elf64Rela>() as u64;

        let reloactions = read_objects::<Elf64Rela>(file, section_header.sh_addr, relocations)
            .context("reading the reloactions from memory")?;

        for relocation in reloactions {
            let value = read_object_mut::<u64>(image, relocation.offset)
                .context("get relocations value")?;

            *value = relocation_target + relocation.addend;
        }
    }

    Ok(())
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
    program_headers: &[ElfProgramHeaderRaw],
    total_size: u64,
) -> Result<&'static mut [u8], anyhow::Error> {
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

    info!(
        "parsed the elf successfuly into 0x{:x}",
        image_base_raw as usize
    );

    Ok(image)
}
