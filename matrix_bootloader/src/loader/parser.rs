use anyhow::{Context, Result, anyhow};
use bytemuck::{Pod, Zeroable};
use log::info;

use crate::loader::elf::{ELF_MAGIC, ElfHeaderRaw, ElfProgramHeaderRaw, ElfSectionHeaderRaw};

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

pub fn parse_elf(file: &[u8]) -> Result<()> {
    let header = parse_object::<ElfHeaderRaw>(file, 0).context("getting the elf header")?;

    if ELF_MAGIC != header.magic {
        return Err(anyhow!("bad elf magic"));
    }

    let x = header.e_entry;
    info!("entry point: {}", x);

    let program_headers =
        parse_objects::<ElfProgramHeaderRaw>(file, header.e_phoff, header.e_phnum as u64)
            .context("getting the program header")?;

    let section_headers =
        parse_objects::<ElfSectionHeaderRaw>(file, header.e_shoff, header.e_shnum as u64)
            .context("getting the section headers")?;

    info!("{:?}", header);
    info!("got {}", section_headers.len());
    info!("{:?}", section_headers);

    Ok(())
}
