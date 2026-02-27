use anyhow::{Context, Result};
use matrix_boot_common::stack::KernelStack;
use uefi::boot::PAGE_SIZE;

pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 20;

pub fn make_stack() -> Result<KernelStack> {
    let ptr = unsafe {
        uefi::boot::allocate_pages(
            uefi::boot::AllocateType::AnyPages,
            uefi::boot::MemoryType::LOADER_DATA,
            KERNEL_STACK_SIZE / PAGE_SIZE,
        )
        .context("allocating the stack for the kernel")?
        .add(KERNEL_STACK_SIZE)
        .as_ptr()
    };

    Ok(KernelStack::new(ptr, KERNEL_STACK_SIZE as _))
}
