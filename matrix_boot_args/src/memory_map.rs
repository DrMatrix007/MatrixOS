#[repr(C)]
pub enum MemoryKind {
    Usable,
    Bootloader,
    UnkownUefi(u32),
}

#[repr(C)]
pub struct MemoryRegion {
    kind: MemoryKind,
    start: u64,
    size: u64,
} 

#[repr(C)]
pub struct MemoryMap {
    ptr: *const MemoryRegion,
    size: u64,
}

impl MemoryMap {
    fn get_slice(&self) -> &[MemoryRegion] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.size as _) }
    } 
}
