#![no_main]
#![no_std]

use log::info;
use uefi::Status;

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("Hello from Matrix bootloader!");

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
