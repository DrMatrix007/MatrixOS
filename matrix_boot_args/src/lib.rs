#![no_std]

pub type MatrixEntryPoint = extern "sysv64" fn(*mut MatrixBootInfo) -> u64;

#[repr(C)]
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
    pub data: *mut MatrixPixel,
    pub width: u64,
    pub height: u64,
}

const _: () = assert!(core::mem::size_of::<MatrixFrameBuffer>() == 24);

impl MatrixFrameBuffer {
    pub fn draw_pixel(&mut self, pixel: &MatrixPixel, x: u64, y: u64) {
        let index = x + self.width * y;
        if index >= self.width * self.height {
            return;
        }

        unsafe { self.data.add(index as usize).write_volatile(*pixel) };
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
            data: data.as_mut_ptr(),
            width,
            height,
        }
    }
}

#[repr(C)]
pub struct MatrixBootInfo {
    pub data: u64,
    pub frame_buffer: MatrixFrameBuffer,
}

// const _: () = assert!(core::mem::size_of::<MatrixBootInfo>() == 24);
