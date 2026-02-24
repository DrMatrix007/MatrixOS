
#[repr(C)]
pub enum MemoryRegionKind {
    Usable,
    Bootloader,
    UnkownUefi(u32),
}

#[repr(C)]
pub struct MemoryRegion {
    kind: MemoryRegionKind,
    phys_start: u64,
    amount_of_4k_pages: u64,
}

impl MemoryRegion {
    pub fn new(kind: MemoryRegionKind, phys_start: u64, amount_of_4k_pages: u64) -> Self {
        Self {
            kind,
            phys_start,
            amount_of_4k_pages,
        }
    }
}

#[repr(C)]
pub struct MemoryMap {
    mem_regions: *const MemoryRegion,
    size: u64,
}

impl From<&'static mut [MemoryRegion]> for MemoryMap {
    fn from(value: &'static mut [MemoryRegion]) -> Self {
        MemoryMap {
            mem_regions: value.as_mut_ptr(),
            size: value.len() as _,
        }
    }
}

impl MemoryMap {
    pub fn get_slice(&self) -> &[MemoryRegion] {
        unsafe { core::slice::from_raw_parts(self.mem_regions, self.size as _) }
    }
}
