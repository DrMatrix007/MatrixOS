use crate::{
    boot_info::{BoxedMatrixBootInfo, MatrixBootInfo, MatrixEntryPoint, MatrixEntryPointRaw},
    relocatable::Relocatable,
    stack::KernelStack,
};

pub struct KernelJumper {
    stack: KernelStack,
    entry: MatrixEntryPointRaw,
    info: BoxedMatrixBootInfo,
}

impl KernelJumper {
    pub fn new(stack: KernelStack, entry: MatrixEntryPointRaw, info: BoxedMatrixBootInfo) -> Self {
        Self { stack, entry, info }
    }

    #[inline]
    pub fn jump(self, phys_offset: u64, kernel_offset: u64) -> ! {
        let relocated_self = Self {
            entry: unsafe { self.entry.relocated(kernel_offset) },
            info: unsafe { self.info.relocated(phys_offset) },
            stack: unsafe { self.stack.relocated(phys_offset) },
        };

        relocated_self.jump_impl();
    }

    #[inline]
    fn jump_impl(self) -> ! {
        unsafe { jump_with_stack(self.stack.top(), self.entry.0, self.info.info()) }
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
        "mov rdi, rdx", // first arg info
        "jmp rsi",      // jump to entry
    );
}
