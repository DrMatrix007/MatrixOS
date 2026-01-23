use anyhow::Result;
use std::path::Path;

use crate::{
    builder::{BuildConfiguration, BuildOptions, build_project},
    clippy::run_clippy,
    project::Project,
};

static KERNEL_PACKAGE_NAME: &str = "matrix_kernel";
static KERNEL_TARGET: &str = "x86_64-unknown-none";
static KERNEL_PATH: &str = "kernel.mat";

#[derive(Debug, Clone, Copy, Default)]
pub struct KernelProject;

impl Project for KernelProject {
    fn build(&self, configuration: BuildConfiguration) -> Result<std::path::PathBuf> {
        println!("    ⁉️  Building Kernel in {} mode", configuration);
        build_project(BuildOptions {
            package: KERNEL_PACKAGE_NAME,
            target: KERNEL_TARGET,
            configuration,
        })
    }

    fn build_image_artifact(&self, esp: &Path, binary: &Path, workspace_root: &Path) -> Result<()> {
        std::fs::copy(binary, esp.join(KERNEL_PATH))?;
        println!(
            "    📂 Binary copied {} to {}",
            binary.strip_prefix(workspace_root)?.display(),
            KERNEL_PATH
        );

        Ok(())
    }

    fn clippy(&self) -> Result<()> {
        run_clippy(KERNEL_PACKAGE_NAME, KERNEL_TARGET)
    }
}
