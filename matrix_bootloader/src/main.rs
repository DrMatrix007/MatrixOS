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
use uefi::{
    Status,
    boot::{self},
    entry,
    mem::memory_map::MemoryMapMut,
};

use crate::{args::make_args, kernel_loader::load_kernel, kernel_stack::KernelStack, memory::init_memory};

static KERNEL_START: u64 = 0xFFFF_FFFF_8000_0000;
static PHYS_OFFSET_START: u64 = 0xFFFF_8000_0000_0000;


#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    
    let kernel = load_kernel(KERNEL_START)
        .context("failed to load kernel")
        .unwrap();

    info!("got kernel with size of 0x{:x}", kernel.image_size);

    let entry = kernel.entry;

    let boot_info = make_args(kernel.image_base)
        .context("get bootinfo")
        .unwrap();
    
    info!("got kernel");

    init_memory(PHYS_OFFSET_START, &kernel, KERNEL_START);

    let stack = KernelStack::new()
        .context("creating the kernel stack")
        .unwrap();

    let mut map = unsafe { boot::exit_boot_services(None) };
    map.sort();

    unsafe { stack.switch() };

    entry(boot_info);
}
