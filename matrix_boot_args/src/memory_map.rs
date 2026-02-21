#[repr(C)]
pub enum MemoryKind {
    Usable,
    Bootloader,
    UnkownUefi(u32),
}

#[repr(C)]
pub struct MemoryMap {
    
}
