use matrix_boot_common::boot_info::{MatrixBootInfo, MatrixEntryPoint};

use crate::{kernel_entry, logger::init_basic_logger};

#[unsafe(no_mangle)]
extern "sysv64" fn _start(boot_info: *mut MatrixBootInfo) -> ! {
    init_basic_logger();
    kernel_entry(unsafe { &mut *boot_info })
}

const _: MatrixEntryPoint = _start;
