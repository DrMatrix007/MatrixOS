#![no_std]
#![no_main]

extern crate alloc;

pub mod args;
pub mod elf_loader;
pub mod kernel_loader;
pub mod protocols;

use anyhow::Context;
use log::info;
use matrix_boot_args::{MatrixBootInfo, MatrixPixel};
use uefi::{Status, boot, entry, print};

use crate::{args::make_args, kernel_loader::load_kernel};

fn hlt() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let entry: extern "sysv64" fn(*mut MatrixBootInfo) -> u64 =
        load_kernel().context("failed to load kernel").unwrap();

    let boot_info = make_args().context("get bootinfo").unwrap();

    // for x in 0..unsafe { boot_info.read() }.frame_buffer.width {
    //     for y in 0..unsafe { boot_info.read() }.frame_buffer.height {
    //         unsafe {
    //             unsafe { boot_info.read() }
    //                 .frame_buffer
    //                 .data
    //                 .add((x + y * unsafe { boot_info.read() }.frame_buffer.width) as usize)
    //                 .write_volatile(MatrixPixel::new(69, 69, 69))
    //         };
    //     }
    // }

    let result = entry(boot_info);

    info!("{}", unsafe { boot_info.read() }.data);

    // hlt();

    Status::SUCCESS
}
