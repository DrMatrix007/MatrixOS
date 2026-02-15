use anyhow::{Context, Result};
use log::info;
use uefi::boot::{self, PAGE_SIZE};

pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 20;

pub struct KernelStack {
    pub end_stack_ptr: *mut u8,
    pub size: u64,
}

impl KernelStack {
    pub fn new() -> Result<Self> {
        let ptr = unsafe {
            uefi::boot::allocate_pages(
                boot::AllocateType::AnyPages,
                boot::MemoryType::LOADER_DATA,
                KERNEL_STACK_SIZE / PAGE_SIZE,
            )
            .context("allocate kernel stack")?
            .add(KERNEL_STACK_SIZE)
            .as_ptr()
        };

        info!("allocated kernel stack at 0x{:x}", ptr as u64);

        Ok(Self {
            end_stack_ptr: ptr,
            size: KERNEL_STACK_SIZE as _,
        })
    }

    /// # Safety
    ///
    /// This function changes the RSP register
    #[inline(always)]
    pub unsafe fn switch(&self) {
        unsafe {
            core::arch::asm! {
                "mov rsp, {0}",
                in(reg) self.end_stack_ptr,
                options(nostack, preserves_flags)
            }
        }
    }
}
