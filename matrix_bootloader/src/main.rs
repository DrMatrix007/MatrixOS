#![no_std]
#![no_main]

extern crate alloc;

pub mod args;
pub mod elf_loader;
pub mod kernel_loader;
pub mod kernel_stack;
pub mod protocols;

use anyhow::Context;
use log::info;
use matrix_boot_args::MatrixBootInfo;
use uefi::{
    Status, boot, entry,
    mem::memory_map::{MemoryMap, MemoryMapMut},
};

use crate::{args::make_args, kernel_loader::load_kernel, kernel_stack::KernelStack};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let kernel = load_kernel().context("failed to load kernel").unwrap();
    let entry = kernel.entry;

    let boot_info = make_args(kernel.image_base).context("get bootinfo").unwrap();

    let stack = KernelStack::new()
        .context("creating the kernel stack")
        .unwrap();

    let mut map = unsafe { boot::exit_boot_services(None) };

    map.sort();

    unsafe { stack.switch() };

    entry(boot_info);
}
