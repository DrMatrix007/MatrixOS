#![no_main]
#![no_std]

pub mod config;

use log::info;
use uefi::{
    Status, boot::{self}, cstr16, proto::media::{file::{File, FileAttribute, FileMode}, fs::SimpleFileSystem}
};

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let fs_handle = boot::get_handle_for_protocol::<SimpleFileSystem>().unwrap();
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(fs_handle).unwrap();

    let mut root = fs.open_volume().unwrap();

    let _file= root.open(cstr16!("matrix"), FileMode::Read, FileAttribute::READ_ONLY).unwrap();

    info!("Hello from Matrix bootloader!");
    return Status::SUCCESS;    
}
