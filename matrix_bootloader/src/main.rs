#![no_main]
#![no_std]

use log::info;
use uefi::{
    Status,
    boot::{self},
    proto::media::fs::SimpleFileSystem,
};

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let fs_handle = boot::get_handle_for_protocol::<SimpleFileSystem>().unwrap();
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(fs_handle).unwrap();

    let root = fs.open_volume().unwrap();

    info!("Hello from Matrix bootloader!");

    hlt();
}
fn hlt() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
