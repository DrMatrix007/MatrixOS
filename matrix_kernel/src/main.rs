#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub fn kernel_entry() {

}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
