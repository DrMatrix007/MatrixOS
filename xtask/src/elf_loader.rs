use crate::project::NamedProject;

pub struct ElfLoaderProject;

impl NamedProject for ElfLoaderProject {
    const IMOJI: &str = "🔄";
    const PACKAGE_NAME: &str = "matrix_loader";
    const TARGET: &str = "x86_64-unknown-none";

    fn build_image_artifact(
        &self,
        _: &std::path::Path,
        _: &std::path::Path,
        _: &std::path::Path,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
