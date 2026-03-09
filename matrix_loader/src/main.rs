#![no_std]
#![no_main]

use core::panic::PanicInfo;

use matrix_common::panic::make_panic_handler;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    make_panic_handler("loader")(info);
}

fn _start() {}
