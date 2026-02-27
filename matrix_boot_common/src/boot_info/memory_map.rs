use crate::relocatable::Relocatable;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MatrixMemoryRegionKind {
    Usable,
    Bootloader,
    UnkownUefi(u32),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MatrixMemoryRegion {
    pub kind: MatrixMemoryRegionKind,
    pub phys_start: u64,
    pub amount_of_4k_pages: u64,
}

impl MatrixMemoryRegion {
    pub fn new(kind: MatrixMemoryRegionKind, phys_start: u64, amount_of_4k_pages: u64) -> Self {
        Self {
            kind,
            phys_start,
            amount_of_4k_pages,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MatrixMemoryMap {
    mem_regions: *const MatrixMemoryRegion,
    len: u64,
}

impl MatrixMemoryMap {
    /// # Safety
    ///
    /// `mem_regions` should point to an array with length `len`
    ///
    pub unsafe fn new(mem_regions: *const MatrixMemoryRegion, len: u64) -> Self {
        Self { mem_regions, len }
    }

    pub fn new_from_slice(value: &'static mut [MatrixMemoryRegion]) -> Self {
        unsafe { Self::new(value.as_ptr(), value.len() as _) }
    }

    pub fn get_slice(&self) -> &[MatrixMemoryRegion] {
        unsafe { core::slice::from_raw_parts(self.mem_regions, self.len as _) }
    }
}

impl Relocatable for MatrixMemoryMap {
    unsafe fn relocated(&self, relocate_addr: u64) -> Self {
        MatrixMemoryMap {
            mem_regions: (self.mem_regions as u64 + relocate_addr) as *const MatrixMemoryRegion,
            len: self.len,
        }
    }
}
