#![no_std]
#![no_main]

pub mod entry_point;

use core::panic::PanicInfo;

use matrix_boot_args::{MatrixBootInfo, MatrixPixel};

fn hlt() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

pub fn kernel_entry(boot_info: &mut MatrixBootInfo) -> u64 {
    for x in 0..boot_info.frame_buffer.width() {
        for y in 0..boot_info.frame_buffer.height() {
            unsafe {
                boot_info
                    .frame_buffer
                    .data
                    .add((x + y * boot_info.frame_buffer.width()) as usize)
                    .write_volatile(MatrixPixel {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: !0,
                    })
            };
        }
    }

    hlt()
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
