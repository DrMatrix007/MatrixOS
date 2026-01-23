use core::slice;

use anyhow::{Context, Result};
use matrix_boot_args::{MatrixBootInfo, MatrixFrameBuffer, MatrixPixel};
use uefi::proto::console::gop::GraphicsOutput;

use crate::protocols::get_procotol;

pub fn make_args() -> Result<MatrixBootInfo> {
    let frame_buffer = make_frame_buffer().context("getting frame buffer")?;

    Ok(MatrixBootInfo { frame_buffer })
}

fn make_frame_buffer() -> Result<MatrixFrameBuffer> {
    let mut gop = get_procotol::<GraphicsOutput>().context("getting the graphics output")?;

    let mut gop_frame_buffer = gop.frame_buffer();
    let pixel_ptr = gop_frame_buffer.as_mut_ptr().cast::<MatrixPixel>();
    let pixel_count = gop_frame_buffer.size() / core::mem::size_of::<MatrixPixel>();

    let slice = unsafe { slice::from_raw_parts_mut(pixel_ptr, pixel_count) };
    let slice: &'static mut [MatrixPixel] = unsafe { core::mem::transmute(slice) };

    let (width, height) = gop.current_mode_info().resolution();

    unsafe {core::mem::forget(gop);}

    Ok(MatrixFrameBuffer::new(slice, width as u64, height as u64))
}
