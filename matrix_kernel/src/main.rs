#![no_std]
#![no_main]

pub mod entry_point;
pub mod logger;
pub mod panics;

use core::panic::PanicInfo;

use log::{error, info};
use matrix_boot_args::{MatrixBootInfo, MatrixPixel};

use crate::logger::init_basic_logger;

fn hlt() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

pub fn kernel_entry(boot_info: &mut MatrixBootInfo) -> ! {
    for x in 0..boot_info.frame_buffer.width() {
        for y in 0..boot_info.frame_buffer.height() {
            boot_info
                .frame_buffer
                .draw_pixel(&MatrixPixel::new(0, 0, 0), x, y);
        }
    }

    init_basic_logger();

    info!("starting matrix os...");

    hlt();
}
