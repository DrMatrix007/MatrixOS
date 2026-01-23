#![no_std]
#![no_main]

pub mod entry_point;

use core::panic::PanicInfo;

static mut VALUE: u64 = 0;

pub fn kernel_entry() -> u64 {
    // unsafe { VALUE += 2 };
    unsafe { VALUE }
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
