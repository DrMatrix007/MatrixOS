use anyhow::Result;
use std::path::Path;

use crate::project::NamedProject;

static KERNEL_PATH: &str = "kernel.mat";

#[derive(Debug, Clone, Copy, Default)]
pub struct KernelProject;

impl NamedProject for KernelProject {
    const PACKAGE_NAME: &str = "matrix_kernel";
    const TARGET: &str = "x86_64-unknown-none";
    const IMOJI: &str = "⁉️";

    fn build_image_artifact(&self, esp: &Path, binary: &Path, workspace_root: &Path) -> Result<()> {
        std::fs::copy(binary, esp.join(KERNEL_PATH))?;
        println!(
            "    📂 Binary copied {} to {}",
            binary.strip_prefix(workspace_root)?.display(),
            KERNEL_PATH
        );

        Ok(())
    }
}
