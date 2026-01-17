mod bootloader;
mod builder;
mod clean;
mod image;
mod runner;
mod kernel;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    bootloader::build_bootloader_project, clean::clean_taret, image::create_image, kernel::build_kernel_project, runner::run_qemu
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
    BuildImage,
    Run,
    Clean,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::BuildImage => {
            build_bootloader_project(cli.release)?;
            build_kernel_project(cli.release)?;
        }
        Commands::Clean => {
            clean_taret()?;
        }
        Commands::Run => {
            let bootloader = build_bootloader_project(cli.release)?;
            let kernel = build_kernel_project(cli.release)?;
            let drive_path = create_image(bootloader, kernel)?;
            run_qemu(drive_path)?;
        }
    }
    Ok(())
}
