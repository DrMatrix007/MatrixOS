use anyhow::{Context, Result};
use uefi::{CStr16, cstr16};

use crate::loader::{parser::parse_elf, read_file::read_file};

static PATH_TO_KERNEL: &CStr16 = cstr16!("kernel.mat");

pub fn load_kernel() -> Result<()> {
    let kernel = read_file(PATH_TO_KERNEL).context("reading kernel from disk")?;

    parse_elf(&kernel).context("parsing the kernel")?;

    Ok(())
}
