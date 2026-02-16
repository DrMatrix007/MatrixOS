use anyhow::{Context, Result};
use log::info;
use uefi::{CStr16, cstr16};

use crate::elf_loader::{
    loader::{LoadedElf, load_elf},
    read_file::read_file,
};

static PATH_TO_KERNEL: &CStr16 = cstr16!("kernel.mat");

pub fn load_kernel(relocation_target: u64) -> Result<LoadedElf> {
    let kernel = read_file(PATH_TO_KERNEL).context("reading kernel from disk")?;

    info!("read the elf successfuly");

    load_elf(&kernel, relocation_target).context("parsing the kernel")
}
