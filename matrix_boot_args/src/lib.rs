#![no_std]

pub type MatrixEntryPoint = extern "sysv64" fn(*mut MatrixBootInfo) -> u64;

#[derive(Debug, Clone, Copy, Default)]
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

#[repr(C)]
pub struct MatrixFrameBuffer {
    data: &'static mut [MatrixPixel],
    width: u64,
    height: u64,
}

const _: () = assert!(core::mem::size_of::<MatrixFrameBuffer>() == 32);

impl MatrixFrameBuffer {
    pub fn draw_pixel(&mut self, pixel: &MatrixPixel, x: u64, y: u64) {
        self.data[(x + self.width * y) as usize] = *pixel;
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }
}

impl MatrixFrameBuffer {
    pub fn new(data: &'static mut [MatrixPixel], width: u64, height: u64) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
}

#[repr(C)]
pub struct MatrixBootInfo {
    pub frame_buffer: MatrixFrameBuffer,
}
