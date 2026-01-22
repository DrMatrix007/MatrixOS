use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::runner::get_target_dir;

static BOOTLOADER_PATH: &str = "EFI/BOOT/BOOTX64.EFI";
static KERNEL_PATH: &str = "kernel.mat";

pub fn create_image(bootloader_bin: PathBuf, kernel_bin: PathBuf) -> Result<PathBuf> {
    let mut drive_path = get_target_dir()?;
    drive_path.push("esp/");
    let boot = drive_path.join(BOOTLOADER_PATH);

    std::fs::create_dir_all(&boot.parent().expect("this path SHOULD have a parent"))?;
    std::fs::copy(&bootloader_bin, boot)?;
    println!(
        "    📂 Binary copied {} to {}",
        bootloader_bin.strip_prefix(env::current_dir()?)?.display(),
        BOOTLOADER_PATH
    );
    std::fs::copy(&kernel_bin, drive_path.join(KERNEL_PATH))?;
    println!(
        "    📂 Binary copied {} to {}",
        kernel_bin.strip_prefix(env::current_dir()?)?.display(),
        KERNEL_PATH
    );

    Ok(drive_path)
}
