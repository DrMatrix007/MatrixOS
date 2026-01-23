use bytemuck::{Pod, Zeroable};

use crate::{
    impl_try_from_enum_values,
    elf_loader::elf::{FileAddress, RvaAddress, enum_values::ValueMismatch},
};

pub static ELF_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfClass {
    ELF32,
    ELF64,
}

impl_try_from_enum_values!(u8, ElfClass {
    ELF32 = 1,
    ELF64 = 2
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfEndianess {
    LittleEndian,
    BigEndian,
}

impl_try_from_enum_values!(u8, ElfEndianess {
    LittleEndian = 1,
    BigEndian = 2
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfOsAbi {
    SystemV,
}

impl_try_from_enum_values!(u8, ElfOsAbi {
    SystemV = 0,
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfType {
    Relocatable,
    Executable,
    SharedObject,
    Core,
}

impl_try_from_enum_values!(u16, ElfType {
    Relocatable = 1,
    Executable = 2,
    SharedObject = 3,
    Core = 4,
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfMachine {
    X86,
    X86_64,
}

impl_try_from_enum_values!(u16, ElfMachine {
    X86 = 3,
    X86_64 = 62
});

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

impl ElfHeaderRaw {
    pub fn get_class(&self) -> Result<ElfClass, ValueMismatch> {
        ElfClass::try_from(self.e_ident_class)
    }

    pub fn get_data_encoding(&self) -> Result<ElfEndianess, ValueMismatch> {
        ElfEndianess::try_from(self.e_ident_data)
    }

    pub fn get_osabi(&self) -> Result<ElfOsAbi, ValueMismatch> {
        ElfOsAbi::try_from(self.e_ident_osabi)
    }

    pub fn get_type(&self) -> Result<ElfType, ValueMismatch> {
        ElfType::try_from(self.e_type)
    }

    pub fn get_machine(&self) -> Result<ElfMachine, ValueMismatch> {
        ElfMachine::try_from(self.e_machine)
    }
}
