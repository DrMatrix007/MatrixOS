#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod arch;
pub mod entry_point;
pub mod logger;
pub mod panics;

use log::info;
use matrix_boot_args::{MatrixBootInfo, MatrixPixel};

use crate::{logger::init_basic_logger, panics::hlt};

fn get_rip() -> u64 {
    let rip: u64;
    unsafe {
        core::arch::asm!("lea rax, [rip]", "mov {}, rax", out(reg) rip);
    }
    rip
}

pub fn kernel_entry(boot_info: &mut MatrixBootInfo) -> ! {
    for x in 0..boot_info.frame_buffer.width() {
        for y in 0..boot_info.frame_buffer.height() {
            boot_info
                .frame_buffer
                .draw_pixel(&MatrixPixel::new(0, 0, 0), x, y);
        }
    }
    
    init_basic_logger();

    info!("starting matrix os...");
    info!("we are runinng at 0x{:x}!", get_rip());

    arch::x64::init_x64();

    hlt();
}
