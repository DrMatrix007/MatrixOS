use anyhow::Result;
use std::path::Path;

use crate::{
    builder::{BuildConfiguration, BuildOptions, build_project},
    project::Project,
};

static BOOTLOADER_PACKAGE_NAME: &str = "matrix_bootloader";
static BOOTLAODER_TARGET: &str = "x86_64-unknown-uefi";
static BOOTLOADER_PATH: &str = "EFI/BOOT/BOOTX64.EFI";

#[derive(Debug, Clone, Copy, Default)]
pub struct BootloaderProject;

impl Project for BootloaderProject {
    fn build(&self, configuration: BuildConfiguration) -> Result<std::path::PathBuf> {
        println!("    🥾  Building Bootloader in {} mode", configuration);
        build_project(BuildOptions {
            package: BOOTLOADER_PACKAGE_NAME,
            target: BOOTLAODER_TARGET,
            configuration,
        })
    }

    fn build_image_artifact(&self, esp: &Path, binary: &Path, workspace_root: &Path) -> Result<()> {
        let boot = esp.join(BOOTLOADER_PATH);

        std::fs::create_dir_all(boot.parent().expect("this path SHOULD have a parent"))?;
        std::fs::copy(binary, boot)?;
        println!(
            "    📂 Binary copied {} to {}",
            binary.strip_prefix(workspace_root)?.display(),
            BOOTLOADER_PATH
        );

        Ok(())
    }
}
