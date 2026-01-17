use std::{env, path::PathBuf};

use anyhow::Result;

use crate::runner::get_target_dir;

pub fn create_image(bootloader_bin: PathBuf, kernel_bin: PathBuf) -> Result<PathBuf> {
    let mut drive_path = get_target_dir()?;
    drive_path.push("esp/");
    let boot = drive_path.join("EFI/BOOT");

    std::fs::create_dir_all(&boot)?;
    std::fs::copy(&bootloader_bin, boot.join("BOOTX64.EFI"))?;
    println!("    📂 Binary copied {} to /EFI/BOOT/BOOTX64.EFI", bootloader_bin.strip_prefix(env::current_dir()?)?.display());
    std::fs::copy(&kernel_bin, drive_path.join("kernel_matrix.bin"))?;
    println!("    📂 Binary copied {} to /kernel_matrix.bin", kernel_bin.strip_prefix(env::current_dir()?)?.display());

    Ok(drive_path)
}
