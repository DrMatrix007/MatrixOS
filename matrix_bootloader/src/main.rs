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
use matrix_boot_args::{MatrixBootInfo, MatrixEntryPoint};
use uefi::{
    Status,
    boot::{self},
    entry,
};

use crate::{
    args::make_args, kernel_loader::load_kernel, kernel_stack::KernelStack,
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

    let kernel_stack = KernelStack::new().unwrap();

    info!("relocating in 0x{:x}", kernel.image_base);
    info!("entry at 0x{:x}", kernel.entry as usize);
    info!("got kernel with size of 0x{:x}", kernel.image_size);

    let entry = kernel.entry;

    let boot_info = make_args(PHYS_OFFSET_START)
        .context("get bootinfo")
        .unwrap();

    info!("got args at 0x{:x}", boot_info as u64);

    let page_table = create_kernel_page_table(PHYS_OFFSET_START, &kernel, KERNEL_START).unwrap();
    info!("got memory");

    _ = unsafe { boot::exit_boot_services(None) };

    unsafe { page_table.apply() };

    unsafe { jump_with_stack(kernel_stack, entry, boot_info, PHYS_OFFSET_START) }
}

fn jump_with_stack(kernel_stack: KernelStack, entry: MatrixEntryPoint, boot_info: *mut MatrixBootInfo) -> ! {

    unsafe {
        jump_with_stack_impl(kernel_stack.top(), entry, boot_info)
    }
}

#[unsafe(naked)]
unsafe "sysv64" fn jump_with_stack_impl(
    _stack_top: u64,
    _entry: extern "sysv64" fn(*mut MatrixBootInfo) -> !,
    _info: *const MatrixBootInfo,
) -> ! {
    core::arch::naked_asm!(
        "mov rsp, rdi", // switch to new stack
        "mov rdi, rdx", // first arg = info
        "jmp rsi",      // jump to entry
    );
}
