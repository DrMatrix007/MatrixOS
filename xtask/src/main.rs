mod bootloader;
mod builder;
mod kernel;
pub mod project;
mod runner;

use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{Context, Result, anyhow};
use cargo_metadata::MetadataCommand;
use clap::{Parser, Subcommand};

use crate::{
    bootloader::BootloaderProject, builder::BuildConfiguration, kernel::KernelProject,
    project::Project, runner::run_qemu,
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
    pub bootloader: BootloaderProject,
    pub kernel: KernelProject,
}

fn main() -> Result<()> {
    let workspace = Workspace {
        bootloader: BootloaderProject,
        kernel: KernelProject,
    };

    let cli = Cli::parse();

    let build_configuration = BuildConfiguration::from_is_release(cli.release);

    match cli.command {
        Commands::Build => {
            build_projects(&workspace, build_configuration)?;
        }
        Commands::Clean => {
            clean_workspace()?;
        }
        Commands::Run => {
            run_workspace(workspace, build_configuration)?;
        }
        Commands::Clippy => {}
    }
    Ok(())
}

fn run_workspace(
    workspace: Workspace,
    build_configuration: BuildConfiguration,
) -> Result<(), anyhow::Error> {
    let workspace_root = get_workspace_root();
    let mut esp = get_target_dir().context("getting target dir for image")?;
    esp.push("esp/");
    let (bootloader, kernel) =
        build_projects(&workspace, build_configuration).context("building projects for image")?;
    workspace
        .bootloader
        .build_image_artifact(&esp, &bootloader, &workspace_root)
        .context("building bootloader artifact")?;
    workspace
        .kernel
        .build_image_artifact(&esp, &kernel, &workspace_root)
        .context("builing kernel artifact")?;
    let mut ovmf_root = workspace_root.clone();
    ovmf_root.push("ovmf");
    run_qemu(&ovmf_root, &esp).context("running qemu")?;
    Ok(())
}

fn build_projects(
    workspace: &Workspace,
    configuration: BuildConfiguration,
) -> Result<(PathBuf, PathBuf), anyhow::Error> {
    let boot_bath = workspace.bootloader.build(configuration)?;
    let kernel_path = workspace.kernel.build(configuration)?;

    Ok((boot_bath, kernel_path))
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
