use bytemuck::{Pod, Zeroable};

use crate::elf_loader::elf::{FileAddress, RvaAddress};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ElfSectionHeaderRaw {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: RvaAddress,
    pub sh_offset: FileAddress,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralgin: u64,
    pub sh_entsize: u64,
}

const _: () = assert!(size_of::<ElfSectionHeaderRaw>() == 64);
