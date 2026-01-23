#![no_std]
#![no_main]

pub mod entry_point;

use core::panic::PanicInfo;

use matrix_boot_args::{MatrixBootInfo, MatrixPixel};

pub fn kernel_entry(boot_info: &mut MatrixBootInfo) -> u64 {
    let frame_buffer = &mut boot_info.frame_buffer;

    for x in 0..frame_buffer.width() {
        for y in 0..frame_buffer.height() {
            frame_buffer.draw_pixel(&MatrixPixel::new(69, 69, 69), x, y);
        }
    }


    &mut *boot_info as *mut MatrixBootInfo as u64
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
