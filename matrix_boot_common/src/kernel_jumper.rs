use crate::{
    boot_info::{BoxedMatrixBootInfo, MatrixBootInfo, MatrixEntryPoint},
    relocatable::Relocatable,
    stack::KernelStack,
};

pub struct KernelJumper {
    stack: KernelStack,
    entry: MatrixEntryPoint,
    info: BoxedMatrixBootInfo,
}

impl KernelJumper {
    pub fn new(stack: KernelStack, entry: MatrixEntryPoint, info: BoxedMatrixBootInfo) -> Self {
        Self { stack, entry, info }
    }

    #[inline]
    pub fn jump(self, phys_offset: u64) -> ! {
        unsafe { self.relocated(phys_offset) }.jump_impl();
    }

    #[inline]
    fn jump_impl(self) -> ! {
        unsafe { jump_with_stack(self.stack.top(), self.entry, self.info.info()) }
    }
}

impl Relocatable for KernelJumper {
    unsafe fn relocated(&self, relocate_addr: u64) -> Self {
        Self {
            entry: unsafe { self.entry.relocated(relocate_addr) },
            info: unsafe { self.info.relocated(relocate_addr) },
            stack: unsafe { self.stack.relocated(relocate_addr) },
        }
    }
}

#[unsafe(naked)]
unsafe extern "sysv64" fn jump_with_stack(
    _stack_top: u64,
    _entry: MatrixEntryPoint,
    _info: *const MatrixBootInfo,
) -> ! {
    core::arch::naked_asm!(
        "mov rsp, rdi", // switch to new stack
        "mov rdi, rdx", // first arg = info
        "jmp rsi",      // jump to entry
    );
}
