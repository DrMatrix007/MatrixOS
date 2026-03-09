use anyhow::Result;
use std::path::Path;

use crate::project::NamedProject;

static BOOTLOADER_PATH: &str = "EFI/BOOT/BOOTX64.EFI";

#[derive(Debug, Clone, Copy, Default)]
pub struct BootloaderProject;

impl NamedProject for BootloaderProject {
    const PACKAGE_NAME: &str = "matrix_bootloader";
    const TARGET: &str = "x86_64-unknown-uefi";

    const IMOJI: &str = "🥾";

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
