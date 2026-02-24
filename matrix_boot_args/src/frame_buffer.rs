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
#[derive(Debug)]
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

    pub fn get_slice_mut(&mut self) -> &mut [MatrixPixel] {
        unsafe {
            core::slice::from_raw_parts_mut(self.data, (self.width() * self.height()) as usize)
        }
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
