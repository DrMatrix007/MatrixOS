use anyhow::{Context, Result};
use uefi::{
    CStr16, cstr16,
    proto::media::{
        file::{File, FileAttribute, FileMode},
        fs::SimpleFileSystem,
    },
};

use crate::protocols::get_procotol;

static PATH_TO_KERNEL: &CStr16 = cstr16!("/");

pub fn load_kernel() -> Result<()> {
    let mut fs = get_procotol::<SimpleFileSystem>().context("cant get filesystem")?;

    let _kernel_file = fs
        .open_volume()?
        .open(PATH_TO_KERNEL, FileMode::Read, FileAttribute::empty())
        .context("cant get kernel file")?;

    Ok(())
}
