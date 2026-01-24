#![no_std]
#![no_main]

pub mod entry_point;

use core::panic::PanicInfo;

use matrix_boot_args::{MatrixBootInfo, MatrixPixel};

pub fn kernel_entry(boot_info: &mut MatrixBootInfo) -> u64 {
    for x in 0..boot_info.frame_buffer.width {
        for y in 0..boot_info.frame_buffer.height {
            unsafe {
                boot_info
                    .frame_buffer
                    .data
                    .add((x + y * 100) as usize)
                    .write_volatile(MatrixPixel::new(69, 69, 69))
            };
        }
    }

    0
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
