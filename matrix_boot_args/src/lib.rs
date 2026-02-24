#![no_std]

use crate::{
    frame_buffer::MatrixFrameBuffer,
    memory_map::{MemoryMap, MemoryRegion},
};

pub type MatrixEntryPoint = extern "sysv64" fn(*mut MatrixBootInfo) -> !;

pub mod frame_buffer;
pub mod memory_map;

#[repr(C)]
pub struct MatrixBootInfo {
    pub data: u64,
    pub frame_buffer: MatrixFrameBuffer,
    pub phys_offset: u64,
    pub memory_map: MemoryMap,
}

impl MatrixBootInfo {
    pub fn new(
        data: u64,
        frame_buffer: MatrixFrameBuffer,
        phys_offset: u64,
        memory_regions: &'static mut [MemoryRegion],
    ) -> Self {
        Self {
            data,
            frame_buffer,
            phys_offset,
            memory_map: MemoryMap::from(memory_regions),
        }
    }
}
