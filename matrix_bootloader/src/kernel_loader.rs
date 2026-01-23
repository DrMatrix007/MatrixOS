use anyhow::{Context, Result};
use log::info;
use matrix_boot_args::{MatrixBootInfo, MatrixEntryPoint};
use uefi::{CStr16, cstr16};

use crate::elf_loader::{loader::load_elf, read_file::read_file};

static PATH_TO_KERNEL: &CStr16 = cstr16!("kernel.mat");

pub fn load_kernel() -> Result<MatrixEntryPoint> {
    let kernel = read_file(PATH_TO_KERNEL).context("reading kernel from disk")?;

    let entry = load_elf(&kernel).context("parsing the kernel")?;

    info!("parsed the elf successfuly");

    Ok(entry)
}
