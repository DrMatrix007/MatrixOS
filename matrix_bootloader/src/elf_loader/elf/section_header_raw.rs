use bytemuck::{Pod, Zeroable};

use crate::{
    elf_loader::elf::{FileAddress, RvaAddress, enum_values::ValueMismatch},
    impl_try_from_enum_values,
};

#[derive(Debug, Clone, Copy)]
pub enum ElfSectionType {
    Progbits,
    Symtab,
    Strtab,
    Rela,
    Hash,
    Dynamic,
    Note,
    Nobits,
    Rel,
    Shlib,
    Dynsym,
    InitArray,
    FiniArray,
    PreinitArray,
    Group,
    SymtabShndx,
    Num,
}

impl_try_from_enum_values!(u32, ElfSectionType {
    Progbits = 0x1,
    Symtab = 0x2,
    Strtab = 0x3,
    Rela = 0x4,
    Hash = 0x5,
    Dynamic = 0x6,
    Note = 0x7,
    Nobits = 0x8,
    Rel = 0x9,
    Shlib = 0x0A,
    Dynsym = 0x0B,
    InitArray = 0x0E,
    FiniArray = 0x0F,
    PreinitArray = 0x10,
    Group = 0x11,
    SymtabShndx = 0x12,
    Num = 0x13,
});

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
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

impl ElfSectionHeaderRaw {
    pub fn get_type(&self) -> Result<ElfSectionType, ValueMismatch> {
        ElfSectionType::try_from(self.sh_type)
    }
}
