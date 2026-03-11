mod bootloader;
mod builder;
pub mod clippy;
mod kernel;
pub mod project;
mod qemu;

use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{Context, Result, anyhow};
use cargo_metadata::MetadataCommand;
use clap::{Parser, Subcommand};

use crate::{
    bootloader::BootloaderProject, builder::BuildConfiguration, kernel::KernelProject,
    project::Project, qemu::run_qemu,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    release: bool,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    Run,
    Clean,
    Clippy,
}

pub struct Workspace {
    pub projects: Vec<Box<dyn Project>>,
}

impl Workspace {
    pub fn new(all_projects: impl IntoIterator<Item = Box<dyn Project>>) -> Self {
        Self {
            projects: all_projects.into_iter().collect(),
        }
    }
}

fn main() -> Result<()> {
    let workspace = Workspace::new([
        Box::new(BootloaderProject),
        Box::new(KernelProject),
    ] as [Box<dyn Project>; _]);

    let cli = Cli::parse();

    let build_configuration = BuildConfiguration::from_is_release(cli.release);

    match cli.command {
        Commands::Build => {
            build_workspace(&workspace, build_configuration)?;
        }
        Commands::Clean => {
            clean_workspace()?;
        }
        Commands::Run => {
            run_workspace(workspace, build_configuration)?;
        }
        Commands::Clippy => {
            clippy_workspace(workspace)?;
        }
    }

    Ok(())
}

fn clippy_workspace(workspace: Workspace) -> Result<()> {
    for project in workspace.projects {
        project.clippy()?;
    }

    Ok(())
}

fn run_workspace(
    workspace: Workspace,
    build_configuration: BuildConfiguration,
) -> Result<(), anyhow::Error> {
    build_workspace(&workspace, build_configuration).context("building projects for image")?;

    let workspace_root = get_workspace_root();
    let mut esp = get_target_dir().context("getting target dir for image")?;
    esp.push(format!("{}/esp/", build_configuration));
    let mut ovmf_root = workspace_root.clone();
    ovmf_root.push("ovmf");

    run_qemu(&ovmf_root, &esp).context("running qemu")?;

    Ok(())
}

fn build_workspace(workspace: &Workspace, config: BuildConfiguration) -> Result<()> {
    let mut esp = get_target_dir().context("getting target dir for image")?;
    esp.push(format!("{}/esp/", config));
    let workspace_root = get_workspace_root();

    std::fs::create_dir_all(&esp)?;

    for x in &workspace.projects {
        let bin = x
            .build(config)
            .with_context(|| format!("building project {}", x.name()))?;
        x.build_image_artifact(&esp, &bin, &workspace_root)?
    }

    Ok(())
}

fn clean_workspace() -> Result<()> {
    let mut command = Command::new("cargo");

    command.arg("clean").stdout(Stdio::piped());

    let res = command
        .spawn()
        .context("spawning clean command")?
        .wait()
        .context("waiting for clean command")?
        .success();

    if !res {
        return Err(anyhow!("{} failed", command.get_program().display()));
    }

    println!(">> successfuly cleaned workspace");

    Ok(())
}

fn get_workspace_root() -> PathBuf {
    let metadata = MetadataCommand::new()
        .no_deps()
        .exec()
        .expect("Failed to execute cargo metadata");

    PathBuf::from(metadata.workspace_root)
}

fn get_target_dir() -> Result<PathBuf> {
    let metadata = cargo_metadata::MetadataCommand::new().exec()?;
    Ok(metadata.target_directory.into_std_path_buf())
}
