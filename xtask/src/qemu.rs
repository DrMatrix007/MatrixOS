use std::{path::Path, process::Command};

use anyhow::{Context, Result};

pub fn run_qemu(ovmf_root: &Path, esp_path: &Path) -> Result<()> {
    Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg(format!(
            "if=pflash,format=raw,readonly=on,file={}/OVMF.4m.fd",
            ovmf_root.display()
        ))
        .arg("-drive")
        .arg(format!("format=raw,file=fat:rw:{}", esp_path.display()))
        .arg("-net")
        .arg("none")
        .args(["-serial", "stdio"])
        // .args(["-monitor", "stdio"])
        .status()
        .context("QEMU failed to start")?;

    Ok(())
}
