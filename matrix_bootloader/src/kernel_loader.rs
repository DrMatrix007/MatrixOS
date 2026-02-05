use anyhow::{Context, Result};
use log::info;
use matrix_boot_args::MatrixEntryPoint;
use uefi::{CStr16, cstr16};

use crate::elf_loader::{
    loader::{LoadedElf, load_elf},
    read_file::read_file,
};

static PATH_TO_KERNEL: &CStr16 = cstr16!("kernel.mat");

pub fn load_kernel() -> Result<LoadedElf> {
    let kernel = read_file(PATH_TO_KERNEL).context("reading kernel from disk")?;
    
    info!("read the elf successfuly");

    Ok(load_elf(&kernel, 0).context("parsing the kernel")?)
}
