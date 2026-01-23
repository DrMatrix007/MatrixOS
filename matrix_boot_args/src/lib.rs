#![no_std]

pub type MatrixEntryPoint = extern "C" fn() -> u64;

pub struct MatrixPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

const _: () = assert!(core::mem::size_of::<MatrixPixel>() == 4);

impl MatrixPixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: u8::MAX,
        }
    }
}

pub struct MatrixFrameBuffer {
    data: &'static [MatrixPixel],
    width: u64,
    height: u64,
}

const _: () = assert!(core::mem::size_of::<MatrixFrameBuffer>() == 32);

impl MatrixFrameBuffer {
    pub fn new(data: &'static [MatrixPixel], width: u64, height: u64) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
}

pub struct MatrixBootInfo {
    pub frame_buffer: MatrixFrameBuffer,
}
