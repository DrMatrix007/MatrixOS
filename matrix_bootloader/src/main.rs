#![no_std]
#![no_main]

extern crate alloc;

pub mod args;
pub mod elf_loader;
pub mod kernel_loader;
pub mod kernel_stack;
pub mod memory;
pub mod protocols;

use anyhow::Context;
use log::info;
use matrix_boot_common::kernel_jumper::KernelJumper;
use uefi::{
    Status,
    boot::{self},
    entry,
};

use crate::{
    args::make_args, kernel_loader::load_kernel, kernel_stack::make_stack,
    memory::create_kernel_page_table,
};

static KERNEL_START: u64 = 0xFFFF_FFFF_8000_0000;
static PHYS_OFFSET_START: u64 = 0xFFFF_8000_0000_0000;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let kernel = load_kernel(KERNEL_START)
        .context("failed to load kernel")
        .unwrap();

    let kernel_stack = make_stack().unwrap();

    info!("relocating in 0x{:x}", kernel.image_base);
    info!("entry at 0x{:x}", kernel.entry.entry() as usize);
    info!("got kernel with size of 0x{:x}", kernel.image_size);

    let boot_info = make_args().context("get bootinfo").unwrap();

    info!("got args at 0x{:x}", boot_info.info() as u64);

    let page_table = create_kernel_page_table(PHYS_OFFSET_START, &kernel, KERNEL_START).unwrap();
    info!("got memory");

    _ = unsafe { boot::exit_boot_services(None) };

    unsafe { page_table.apply() };

    let kernel_jumper = KernelJumper::new(kernel_stack, kernel.entry, boot_info);
    kernel_jumper.jump(PHYS_OFFSET_START, KERNEL_START);
}
