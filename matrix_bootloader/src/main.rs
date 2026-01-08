#![no_main]
#![no_std]

use log::info;
use uefi::{
    Status, boot,
    proto::console::gop::{self, BltOp, BltPixel, GraphicsOutput},
};

#[uefi::entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    info!("Hello from Matrix bootloader!");

    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();

    let (x,y) = gop.current_mode_info().resolution();

    gop.blt(BltOp::VideoFill {
        color: BltPixel::new(0x66, 0x69, 0x69),
        dest: (0, 0),
        dims: (x, y),
    }).unwrap();

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
