use matrix_boot_args::{MatrixBootInfo, MatrixEntryPoint};

use crate::kernel_entry;

#[unsafe(no_mangle)]
extern "sysv64" fn _start(boot_info: *mut MatrixBootInfo) -> ! {
    kernel_entry(unsafe { &mut *boot_info })
}

const _: MatrixEntryPoint = _start;
