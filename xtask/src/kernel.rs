use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;

use crate::builder::{BuildOptions, build_project};
static KERNEL_PACKAGE_NAME: &str = "matrix_kernel";
static KERNEL_TARGET: &str = "x86_64-unknown-none";

pub fn build_kernel_project(release: bool) -> Result<PathBuf> {
    let mode = if release { "release" } else { "debug" };
    println!("    ⁉️  Building Kernel in {} mode...", mode);

    build_project(BuildOptions {
        package: KERNEL_PACKAGE_NAME.to_string(),
        target: KERNEL_TARGET.to_string(),
        release,
    })
    .context("❌ No binary found")
}
