#![no_std]
#![no_main]

extern crate alloc;

pub mod args;
pub mod elf_loader;
pub mod kernel_loader;
pub mod protocols;

use anyhow::Context;
use matrix_boot_args::MatrixBootInfo;
use uefi::{Status, entry};

use crate::{args::make_args, kernel_loader::load_kernel};

fn hlt() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let _entry = load_kernel().context("failed to load kernel").unwrap();

    let boot_info = make_args();
    let Ok(_boot_info) = boot_info else {
        return Status::ABORTED;
    };

    hlt();
}
