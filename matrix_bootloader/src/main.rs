#![no_std]
#![no_main]

extern crate alloc;

pub mod args;
pub mod elf_loader;
pub mod kernel_loader;
pub mod kernel_stack;
pub mod protocols;

use anyhow::Context;
use matrix_boot_args::MatrixBootInfo;
use uefi::{Status, boot, entry};

use crate::{args::make_args, kernel_loader::load_kernel, kernel_stack::KernelStack};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let entry: extern "sysv64" fn(*mut MatrixBootInfo) -> ! =
        load_kernel().context("failed to load kernel").unwrap();

    let boot_info = make_args().context("get bootinfo").unwrap();

    let stack = KernelStack::new()
        .context("creating the kernel stack")
        .unwrap();

    unsafe {
        let _ = boot::exit_boot_services(None);
    };

    unsafe { stack.switch() };

    entry(boot_info);
}
