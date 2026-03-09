use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{
    builder::{BuildConfiguration, BuildOptions, build_project},
    clippy::run_clippy,
};
pub trait Project {
    fn name(&self) -> &'static str;
    fn build(&self, configuration: BuildConfiguration) -> Result<PathBuf>;
    fn build_image_artifact(&self, esp: &Path, binary: &Path, workspace_root: &Path) -> Result<()>;
    fn clippy(&self) -> Result<()>;
}

pub trait NamedProject {
    const PACKAGE_NAME: &str;
    const TARGET: &str;
    const IMOJI: &str;

    fn build_image_artifact(&self, esp: &Path, binary: &Path, workspace_root: &Path) -> Result<()>;
}

impl<T: NamedProject> Project for T {
    fn name(&self) -> &'static str {
        Self::PACKAGE_NAME
    }

    fn build(&self, configuration: BuildConfiguration) -> Result<PathBuf> {
        println!(
            "    {}  Building {} in {} mode for {}",
            Self::IMOJI,
            Self::PACKAGE_NAME,
            configuration,
            Self::TARGET
        );
        build_project(BuildOptions {
            configuration,
            package: Self::PACKAGE_NAME,
            target: Self::TARGET,
        })
    }

    fn build_image_artifact(&self, esp: &Path, binary: &Path, workspace_root: &Path) -> Result<()> {
        NamedProject::build_image_artifact(self, esp, binary, workspace_root)
    }

    fn clippy(&self) -> Result<()> {
        run_clippy(Self::PACKAGE_NAME, Self::TARGET)
    }
}
