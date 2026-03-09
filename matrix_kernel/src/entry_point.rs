use matrix_common::{
    boot_info::{MatrixBootInfo, MatrixEntryPoint},
    logger::qemu_logger::init_qemu_logger,
};

use crate::kernel_entry;

#[unsafe(no_mangle)]
extern "sysv64" fn _start(boot_info: *mut MatrixBootInfo) -> ! {
    init_qemu_logger();
    kernel_entry(unsafe { &mut *boot_info })
}

const _: MatrixEntryPoint = _start;
