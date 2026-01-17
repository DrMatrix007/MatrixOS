use std::{path::PathBuf, process::Command};

use anyhow::{Context, Result};

pub fn get_target_dir() -> Result<PathBuf> {
    let metadata = cargo_metadata::MetadataCommand::new().exec()?;
    Ok(metadata.target_directory.into_std_path_buf())
}

pub fn run_qemu(drive_path: PathBuf) -> Result<()> {
    
    println!("🚀  Launching QEMU...");
    Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg("if=pflash,format=raw,readonly=on,file=ovmf/OVMF.4m.fd")
        .arg("-drive")
        .arg(format!("format=raw,file=fat:rw:{}", drive_path.display()))
        .arg("-net")
        .arg("none")
        .status()
        .context("QEMU failed to start")?;
        
    Ok(())
}
