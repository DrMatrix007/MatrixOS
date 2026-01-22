#![no_std]
#![no_main]

pub mod kernel_loader;
pub mod protocols;

use log::info;
use uefi::{Status, entry, proto::console::gop::GraphicsOutput};

use crate::kernel_loader::load_kernel;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello worldasdasds!");

    let gop_guid = uefi::boot::get_handle_for_protocol::<GraphicsOutput>().expect("no gop");
    let mut gop = uefi::boot::open_protocol_exclusive::<GraphicsOutput>(gop_guid).expect("no gop");

    let (width, height) = gop.current_mode_info().resolution();

    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) * 4;
            unsafe { gop.frame_buffer().write_value(index, 0x69696969) };
        }
    }

    load_kernel();

    Status::SUCCESS
}
