#![no_std]

use crate::{
    frame_buffer::MatrixFrameBuffer, memory_map::MatrixMemoryMap, relocatable::Relocatable,
};

pub type MatrixEntryPoint = extern "sysv64" fn(*mut MatrixBootInfo) -> !;

pub mod frame_buffer;
pub mod memory_map;
pub mod relocatable;

#[repr(C)]
#[derive(Debug)]
pub struct MatrixBootInfo {
    pub data: u64,
    pub frame_buffer: MatrixFrameBuffer,
    pub phys_offset: u64,
    pub memory_map: MatrixMemoryMap,
}

impl MatrixBootInfo {
    pub fn new(
        data: u64,
        frame_buffer: MatrixFrameBuffer,
        phys_offset: u64,
        memory_map: MatrixMemoryMap,
    ) -> Self {
        Self {
            data,
            frame_buffer,
            phys_offset,
            memory_map,
        }
    }
}

impl Relocatable for MatrixBootInfo {
    unsafe fn relocated(&self, relocate_addr: u64) -> Self {
        Self {
            data: self.data,
            frame_buffer: unsafe { self.frame_buffer.relocated(relocate_addr) },
            memory_map: unsafe { self.memory_map.relocated(relocate_addr) },
            phys_offset: self.phys_offset,
        }
    }
}
