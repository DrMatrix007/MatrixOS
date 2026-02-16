use core::slice;

use anyhow::{Context, Result};
use matrix_boot_args::{
    MatrixBootInfo,
    frame_buffer::{MatrixFrameBuffer, MatrixPixel},
};
use uefi::{
    boot::{self, MemoryType},
    proto::console::gop::GraphicsOutput,
};

use crate::protocols::get_procotol;

pub fn make_args(phys_offset: u64) -> Result<*mut MatrixBootInfo> {
    let boot_info: *mut MatrixBootInfo = boot::allocate_pages(
        boot::AllocateType::AnyPages,
        MemoryType::BOOT_SERVICES_DATA,
        core::mem::size_of::<MatrixBootInfo>().div_ceil(0x1000),
    )
    .context("allocating for the data")?
    .cast()
    .as_ptr();
    let frame_buffer = make_frame_buffer().context("getting frame buffer")?;

    unsafe { boot_info.write(MatrixBootInfo::new(0x1b, frame_buffer, phys_offset)) };

    Ok((boot_info as u64 + phys_offset) as *mut MatrixBootInfo)
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
