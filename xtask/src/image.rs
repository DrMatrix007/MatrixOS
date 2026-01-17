use std::{env, path::PathBuf};

use anyhow::Result;

use crate::runner::get_target_dir;

pub fn create_image(bin_path: PathBuf) -> Result<PathBuf> {
    let mut drive_path = get_target_dir()?;
    drive_path.push("esp/");
    let boot = drive_path.join("EFI/BOOT");

    std::fs::create_dir_all(&boot)?;
    std::fs::copy(&bin_path, boot.join("BOOTX64.EFI"))?;
    println!("📂 Binary copied {} to /EFI/BOOT/BOOTX64.EFI", bin_path.strip_prefix(env::current_dir()?)?.display());

    Ok(drive_path)
}
