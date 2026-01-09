static KERNEL_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/kernel.img"));

pub fn get_kernel_image() -> &'static [u8] {
    return KERNEL_BYTES;
}
