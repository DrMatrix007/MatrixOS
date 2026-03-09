use core::panic::PanicInfo;

use log::error;

pub fn make_panic_handler(program_name: &str) -> impl FnOnce(&PanicInfo) -> ! {
    move |info| {
        error!("PANIC!!! ({}) {}", program_name, info.message());
        match info.location() {
            Some(location) => {
                error!("panic location: {}:{}", location.file(), location.line());
            }
            _ => {
                error!("NO LOCATION!")
            }
        }

        loop {
            unsafe { core::arch::asm!("hlt") };
        }
    }
}
