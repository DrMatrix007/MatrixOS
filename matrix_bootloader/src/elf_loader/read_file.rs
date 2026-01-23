use alloc::{format, vec::{Vec}};
use alloc::vec;
use anyhow::{Context, Result, anyhow};
use log::info;
use uefi::{
    CStr16,
    proto::media::{
        file::{File, FileAttribute, FileMode, RegularFile},
        fs::SimpleFileSystem,
    },
};

use crate::protocols::get_procotol;

pub fn read_file(path: &CStr16) -> Result<Vec<u8>> {
    let mut fs = get_procotol::<SimpleFileSystem>().context("cant get filesystem")?;

    let file = fs
        .open_volume()?
        .open(path, FileMode::Read, FileAttribute::READ_ONLY)
        .context("cant get kernel file")?;

    let mut file = file
        .into_regular_file()
        .context("this handle should be file")?;

    file.set_position(RegularFile::END_OF_FILE)
        .context("getting size")?;

    let file_size = file.get_position().context("get the size of the file")?;

    file.set_position(0)
        .context("resetting the file position")?;

    info!("got file with size {}", file_size);

    let mut res = vec![0; file_size as usize];

    let bytes_read = file
        .read(&mut res)
        .with_context(|| format!("reading a file {}", path))? as u64;

    if bytes_read != file_size {
        return Err(anyhow!("reading the entire file failed"));
    }

    Ok(res)
}
