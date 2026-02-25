use core::slice;

use alloc::vec::Vec;
use anyhow::{Context, Ok, Result};
use log::info;
use matrix_boot_args::{
    MatrixBootInfo,
    frame_buffer::{MatrixFrameBuffer, MatrixPixel},
    memory_map::{MatrixMemoryMap, MatrixMemoryRegion, MatrixMemoryRegionKind},
    relocatable::Relocatable,
};
use uefi::{
    boot::{self, MemoryType, PAGE_SIZE, memory_map},
    mem::memory_map::{MemoryMap, MemoryMapMut},
    proto::console::gop::GraphicsOutput,
};

use crate::protocols::get_procotol;

pub fn make_args(phys_offset: u64) -> Result<*mut MatrixBootInfo> {
    let boot_info: *mut MatrixBootInfo = boot::allocate_pages(
        boot::AllocateType::AnyPages,
        MemoryType::BOOT_SERVICES_DATA,
        core::mem::size_of::<MatrixBootInfo>().div_ceil(PAGE_SIZE),
    )
    .context("allocating for the data")?
    .cast()
    .as_ptr();
    let frame_buffer = make_frame_buffer().context("getting frame buffer")?;

    let memory_regions = make_memory_map().context("get memory regions")?;

    unsafe {
        boot_info.write(
            MatrixBootInfo::new(0x1b, frame_buffer, phys_offset, memory_regions)
                .relocated(phys_offset),
        )
    };

    Ok((boot_info as u64 + phys_offset) as *mut MatrixBootInfo)
}

fn make_memory_map() -> Result<MatrixMemoryMap> {
    let mut map = memory_map(MemoryType::BOOT_SERVICES_DATA)?;
    map.sort();
    let data: Vec<_> = map
        .entries()
        .map(|x| {
            let kind = match x.ty {
                MemoryType::CONVENTIONAL => MatrixMemoryRegionKind::Usable,
                unknown_uefi => MatrixMemoryRegionKind::UnkownUefi(unknown_uefi.0),
            };
            MatrixMemoryRegion::new(kind, x.phys_start, x.page_count)
        })
        .collect();

    info!("got memory regions");

    let slice = boot::allocate_pages(
        boot::AllocateType::AnyPages,
        MemoryType::BOOT_SERVICES_DATA,
        (core::mem::size_of::<MatrixMemoryRegion>() * data.len()).div_ceil(PAGE_SIZE),
    )
    .context("allocating the memory map")
    .map(|x| {
        let slice =
            unsafe { core::slice::from_raw_parts_mut::<'static>(x.cast().as_ptr(), data.len()) };
        slice.copy_from_slice(&data);
        slice
    })?;

    Ok(MatrixMemoryMap::new_from_slice(slice))
}

fn make_frame_buffer() -> Result<MatrixFrameBuffer> {
    let mut gop = get_procotol::<GraphicsOutput>().context("getting the graphics output")?;

    let mut gop_frame_buffer = gop.frame_buffer();
    let pixel_ptr = gop_frame_buffer.as_mut_ptr().cast::<MatrixPixel>();
    let pixel_count = gop_frame_buffer.size() / core::mem::size_of::<MatrixPixel>();

    let slice = unsafe { slice::from_raw_parts_mut(pixel_ptr, pixel_count) };
    let slice: &'static mut [MatrixPixel] = unsafe { core::mem::transmute(slice) };

    let (width, height) = gop.current_mode_info().resolution();

    Ok(MatrixFrameBuffer::new(slice, width as u64, height as u64))
}
