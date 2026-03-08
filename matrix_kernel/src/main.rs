#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod arch;
pub mod entry_point;
pub mod logger;
pub mod memory;
pub mod memory_locations;
pub mod panics;
pub mod processes;

use log::info;
use matrix_boot_common::boot_info::{
    MatrixBootInfo, frame_buffer::MatrixPixel, memory_map::MatrixMemoryRegionKind,
};
use x86_64::{
    VirtAddr,
    instructions::hlt,
    structures::paging::{PageSize, Size4KiB},
};

use crate::processes::process::Process;

fn get_rip() -> u64 {
    let rip: u64;
    unsafe {
        core::arch::asm!("lea rax, [rip]", "mov {}, rax", out(reg) rip);
    }
    rip
}

#[inline(always)]
fn get_rsp() -> u64 {
    let x;
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) x);
    }
    x
}

pub fn kernel_entry(boot_info: &'static mut MatrixBootInfo) -> ! {
    boot_info
        .frame_buffer
        .get_slice_mut()
        .fill(MatrixPixel::new(0x69, 0x69, 0x69));

    log_boot_stuff(boot_info);

    let MatrixBootInfo {
        frame_buffer: _,
        phys_offset,
        memory_map,
    } = boot_info;

    unsafe {
        memory::init_memory(VirtAddr::new(*phys_offset), memory_map);
        info!("working memory");
        arch::x64::init_x64(VirtAddr::new(*phys_offset));
    }

    let _ = Process::new().unwrap();

    info!("did not crash!!!");

    loop {
        hlt();
    }
}

fn log_boot_stuff(boot_info: &MatrixBootInfo) {
    info!("starting matrix os...");
    info!("we are runinng at 0x{:x}!", get_rip());
    info!("we are runinng with stack at 0x{:x}!", get_rsp());
    info!("got physical offset at 0x{:x}", boot_info.phys_offset());
    info!("got args: {:#?}", boot_info);

    info!(
        "got memory map with len: {:?}",
        boot_info.memory_map.get_slice().len()
    );

    info!(
        "got memory map with usable regions: {:#?}",
        boot_info
            .memory_map
            .get_slice()
            .iter()
            .filter(|x| matches!(x.kind, MatrixMemoryRegionKind::Usable))
            .count()
    );

    info!(
        "got ram of size: 0x{:x}",
        boot_info
            .memory_map
            .get_slice()
            .iter()
            .filter(|x| matches!(x.kind, MatrixMemoryRegionKind::Usable))
            .map(|x| x.amount_of_4k_pages)
            .sum::<u64>()
            * Size4KiB::SIZE
    );
}
