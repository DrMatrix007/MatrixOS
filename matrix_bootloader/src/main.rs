#![no_std]
#![no_main]

extern crate alloc;

pub mod kernel_loader;
pub mod loader;
pub mod protocols;

use anyhow::Context;
use uefi::{Status, entry};

use crate::kernel_loader::load_kernel;

fn hlt() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    load_kernel().context("failed to load kernel").unwrap();

    hlt();
}
