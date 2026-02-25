#![no_std]

use core::ptr::null;

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
    pub frame_buffer: MatrixFrameBuffer,
    pub phys_offset: *const (),
    pub memory_map: MatrixMemoryMap,
}

impl MatrixBootInfo {
    pub fn new(frame_buffer: MatrixFrameBuffer, memory_map: MatrixMemoryMap) -> Self {
        Self {
            frame_buffer,
            phys_offset: null(),
            memory_map,
        }
    }

    pub fn phys_offset(&self) -> u64 {
        self.phys_offset as _
    }
}

impl Relocatable for MatrixBootInfo {
    unsafe fn relocated(&self, relocate_addr: u64) -> Self {
        Self {
            frame_buffer: unsafe { self.frame_buffer.relocated(relocate_addr) },
            memory_map: unsafe { self.memory_map.relocated(relocate_addr) },
            phys_offset: relocate_addr as _,
        }
    }
}
