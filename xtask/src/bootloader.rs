use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;

use crate::builder::{BuildOptions, build_project};
static BOOTLOADER_PACKAGE_NAME: &str = "matrix_bootloader";
static BOOTLAODER_TARGET: &str = "x86_64-unknown-uefi";

pub fn build_bootloader_project(release: bool) -> Result<PathBuf> {
    let mode = if release { "release" } else { "debug" };
    println!("    🥾  Building bootloader in {} mode...", mode);

    build_project(BuildOptions {
        package: BOOTLOADER_PACKAGE_NAME.to_string(),
        target: BOOTLAODER_TARGET.to_string(),
        release,
    })
    .context("❌ No binary found")
}
