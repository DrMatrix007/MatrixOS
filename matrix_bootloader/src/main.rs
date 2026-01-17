#![no_std]
#![no_main]

use log::info;
use uefi::{Status, entry};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello worldasdasds!");

    Status::SUCCESS
}
