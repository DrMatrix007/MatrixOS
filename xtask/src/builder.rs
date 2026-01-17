use anyhow::{Result, anyhow};
use cargo_metadata::Message;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct BuildOptions {
    pub package: String,
    pub target: String,
    pub release: bool,
}

pub fn build_project(opts: BuildOptions) -> Result<PathBuf> {
    let mut command = Command::new("cargo");

    command.args(&[
        "build",
        "--package",
        &opts.package,
        "--target",
        &opts.target,
        "--message-format=json-render-diagnostics",
    ]);

    if opts.release {
        command.arg("--release");
    }

    let mut child = command.stdout(Stdio::piped()).spawn()?;

    let mut artifact_path = None;
    let reader = std::io::BufReader::new(child.stdout.take().unwrap());

    for message in Message::parse_stream(reader) {
        match message? {
            Message::CompilerArtifact(artifact) => {
                if artifact.target.name == opts.package
                    && artifact.target.kind.contains(&"bin".to_string())
                {
                    artifact_path = Some(PathBuf::from(artifact.filenames[0].clone()));
                }
            }
            Message::BuildFinished(finished) => {
                if !finished.success {
                    return Err(anyhow!("Cargo build finished with errors"));
                }
            }
            _ => (),
        }
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(anyhow!("Cargo process exited with status: {}", status));
    }

    artifact_path.ok_or_else(|| {
        anyhow!(
            "Successfully built but no binary artifact was found for {}",
            opts.package
        )
    })
}
