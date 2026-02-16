#![no_std]

use crate::frame_buffer::MatrixFrameBuffer;

pub type MatrixEntryPoint = extern "sysv64" fn(*mut MatrixBootInfo) -> !;

pub mod frame_buffer;

#[repr(C)]
pub struct MatrixBootInfo {
    pub data: u64,
    pub frame_buffer: MatrixFrameBuffer,
    pub phys_offset: u64,
}

impl MatrixBootInfo {
    pub fn new(data: u64, frame_buffer: MatrixFrameBuffer, phys_offset: u64) -> Self {
        Self {
            data,
            frame_buffer,
            phys_offset,
        }
    }
}

const _: () = assert!(core::mem::size_of::<MatrixBootInfo>() == 40);
