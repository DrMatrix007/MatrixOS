use matrix_boot_args::MatrixBootInfo;

use crate::kernel_entry;

#[unsafe(no_mangle)]
extern "sysv64" fn _start(boot_info: *mut MatrixBootInfo) -> u64 {
    kernel_entry(unsafe { &mut *boot_info })
}
