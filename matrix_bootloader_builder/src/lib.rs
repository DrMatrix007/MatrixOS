static IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fat.img"));

pub fn get_bootloader_image() -> &'static [u8] {
    IMAGE
}
