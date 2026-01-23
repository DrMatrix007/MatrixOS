use std::process::Command;

use anyhow::{Context, Result, anyhow};

pub fn run_clippy(package_name: &str, target: &str) -> Result<()> {
    let success = Command::new("cargo")
        .args(["clippy", "--target", target, "--package", package_name])
        .spawn()
        .context("running clippy command")?
        .wait()
        .context("waiting for clippy command")?
        .success();

    if !success {
        return Err(anyhow!("clippy failed"));
    }

    Ok(())
}
