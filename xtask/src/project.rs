use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::builder::BuildConfiguration;
pub trait Project {
    fn build(&self, release: BuildConfiguration) -> Result<PathBuf>;
    fn build_image_artifact(&self, esp: &Path, binary: &Path, workspace_root: &Path) -> Result<()>;
    fn clippy(&self) -> Result<()>;
}
