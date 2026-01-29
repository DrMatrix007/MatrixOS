#![no_std]
#![no_main]

extern crate alloc;

pub mod args;
pub mod elf_loader;
pub mod kernel_loader;
pub mod protocols;

use anyhow::Context;
use matrix_boot_args::MatrixBootInfo;
use uefi::{Status, boot, entry};

use crate::{args::make_args, kernel_loader::load_kernel};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let entry: extern "sysv64" fn(*mut MatrixBootInfo) -> ! =
        load_kernel().context("failed to load kernel").unwrap();

    let boot_info = make_args().context("get bootinfo").unwrap();

    unsafe {
        let _ = boot::exit_boot_services(None);
    };

    entry(boot_info);
}
