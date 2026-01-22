#![no_std]
#![no_main]

pub mod entry_point;

use core::panic::PanicInfo;

pub fn kernel_entry() {
    loop{}
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
