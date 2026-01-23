#![no_std]
#![no_main]

extern crate alloc;

pub mod args;
pub mod elf_loader;
pub mod kernel_loader;
pub mod protocols;

use anyhow::Context;
use log::info;
use matrix_boot_args::{MatrixEntryPoint, MatrixPixel};
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

    let entry = load_kernel().context("failed to load kernel").unwrap();
    
    let mut boot_info = make_args().context("get bootinfo").unwrap();


    entry(&mut boot_info);

    drop(boot_info);

    Status::SUCCESS
}
