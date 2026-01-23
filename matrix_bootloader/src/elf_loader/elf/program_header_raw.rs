use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};

use crate::{
    impl_try_from_enum_values,
    elf_loader::elf::{FileAddress, RvaAddress, enum_values::ValueMismatch},
};

#[derive(Debug)]
pub enum ElfProgramHeaderType {
    Load,
}

impl_try_from_enum_values!(u32, ElfProgramHeaderType {
    Load = 1
});

bitflags! {
    #[derive(Debug)]
    pub struct ElfProgramHeaderFlags: u32 {
        const Executable = 1;
        const Writable = 2;
        const Readable = 4;
    }
}
impl TryFrom<u32> for ElfProgramHeaderFlags {
    type Error = ValueMismatch;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value & ElfProgramHeaderFlags::all().bits() != 0 {
            Ok(ElfProgramHeaderFlags::from_bits_retain(value))
        } else {
            Err(ValueMismatch)
        }
    }
}

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

impl ElfProgramHeaderRaw {
    pub fn get_type(&self) -> Result<ElfProgramHeaderType, ValueMismatch> {
        ElfProgramHeaderType::try_from(self.p_type)
    }

    pub fn get_flags(&self) -> Result<ElfProgramHeaderFlags, ValueMismatch> {
        ElfProgramHeaderFlags::try_from(self.p_flags)
    }
}

const _: () = assert!(size_of::<ElfProgramHeaderRaw>() == 56);
