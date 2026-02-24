#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// extern crate alloc;

pub mod arch;
pub mod entry_point;
pub mod logger;
pub mod memory;
pub mod panics;

use log::info;
use matrix_boot_args::{MatrixBootInfo, frame_buffer::MatrixPixel};
use x86_64::VirtAddr;

use crate::panics::hlt;

fn get_rip() -> u64 {
    let rip: u64;
    unsafe {
        core::arch::asm!("lea rax, [rip]", "mov {}, rax", out(reg) rip);
    }
    rip
}

pub fn kernel_entry(boot_info: &mut MatrixBootInfo) -> ! {
    boot_info
        .frame_buffer
        .get_slice_mut()
        .fill(MatrixPixel::new(0x69, 0x69, 0x69));

    info!("starting matrix os...");
    info!("we are runinng at 0x{:x}!", get_rip());
    info!("got physical offset at 0x{:x}", boot_info.phys_offset);
    info!("got args: {:#?}", boot_info);
    info!(
        "got memory map with len: {:?}",
        boot_info.memory_map.get_slice().len()
    );

    unsafe {
        arch::x64::init_x64();
        memory::init_memory(VirtAddr::new(boot_info.phys_offset), &boot_info.memory_map);
    }

    info!("did not crash!!!");

    hlt();
}
