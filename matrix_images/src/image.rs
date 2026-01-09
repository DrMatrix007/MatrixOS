use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use fatfs::{FormatVolumeOptions, FsOptions, format_volume};

type RawFs = fatfs::FileSystem<File>;

pub struct Image {
    fs: RawFs,
}

impl Image {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        const SIZE: u64 = 1024 * 1024;

        let real_file = OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;

        real_file.set_len(SIZE)?;

        format_volume(&real_file, FormatVolumeOptions::new())?;

        let fs = RawFs::new(real_file, FsOptions::new())?;

        Ok(Self { fs })
    }

    pub fn create_dir(&mut self, path: &str) -> anyhow::Result<()> {
        self.fs.root_dir().create_dir(path)?;
        Ok(())
    }

    pub fn write_new_file(&mut self, path: &str, data: &[u8]) -> anyhow::Result<()> {
        let mut file: fatfs::File<'_, File> = self.fs.root_dir().create_file(path)?;

        file.truncate()?;
        file.write_all(data)?;

        Ok(())
    }
}
