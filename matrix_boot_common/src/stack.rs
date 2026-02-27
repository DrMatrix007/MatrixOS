use crate::relocatable::Relocatable;

pub struct KernelStack {
    pub end_stack_ptr: *mut u8,
    pub size: u64,
}

impl KernelStack {
    pub fn new(end_stack_ptr: *mut u8, size: u64) -> Self {
        Self {
            end_stack_ptr,
            size,
        }
    }

    pub fn top(&self) -> u64 {
        self.end_stack_ptr as _
    }
}

impl Relocatable for KernelStack {
    unsafe fn relocated(&self, relocate_addr: u64) -> Self {
        Self {
            end_stack_ptr: unsafe { self.end_stack_ptr.add(relocate_addr as usize) },
            size: self.size,
        }
    }
}
