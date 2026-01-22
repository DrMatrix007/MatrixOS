use anyhow::{Result, anyhow};
use bytemuck::{Pod, Zeroable};

pub static ELF_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

type RvaAddress = u64;
type FileAddress = u64;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ElfHeaderRaw {
    pub magic: [u8; 4],
    pub e_ident_class: u8,
    pub e_ident_data: u8,
    pub e_ident_version: u8,
    pub e_ident_osabi: u8,
    pub e_ident_abi_version: u8,
    pub e_ident_pads: [u8; 7],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: RvaAddress,
    pub e_phoff: FileAddress,
    pub e_shoff: FileAddress,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

const _: () = assert!(size_of::<ElfHeaderRaw>() == 64);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ElfProgramHeaderRaw {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: FileAddress,
    pub p_vaddr: RvaAddress,
    pub p_paddr: RvaAddress,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

const _: () = assert!(size_of::<ElfProgramHeaderRaw>() == 56);

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
