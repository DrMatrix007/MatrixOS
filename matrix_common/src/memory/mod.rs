use x86_64::VirtAddr;

pub mod mappings;


#[repr(C)]
pub struct MemoryOptions {
    kernel_start: u64,
    phys_offset: u64,
}

impl MemoryOptions {
    pub fn new(kernel_start: u64, phys_offset: u64) -> Self {
        Self { kernel_start, phys_offset }
    }
}