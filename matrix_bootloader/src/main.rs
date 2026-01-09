#![no_main]
#![no_std]

use log::info;
use uefi::{
    boot::{self},
    proto::media::fs::SimpleFileSystem,
    Status,
};

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let fs_handle = boot::get_handle_for_protocol::<SimpleFileSystem>().unwrap();
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(fs_handle).unwrap();

    let _root = fs.open_volume().unwrap();

    info!("Hello from Matrix bootloader!");

    return Status::SUCCESS;
}
