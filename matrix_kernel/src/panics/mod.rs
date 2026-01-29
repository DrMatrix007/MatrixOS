use core::panic::PanicInfo;

use log::error;


pub fn hlt() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    error!("PANIC!!! {}", info.message());
    match info.location() {
        Some(location) => {
            error!("panic location: {}@{}", location.file(), location.line());
        }
        _ => {
            error!("NO LOCATION!")
        }
    }

    hlt()
}
