use core::panic::PanicInfo;

use log::error;

use crate::hlt;


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
