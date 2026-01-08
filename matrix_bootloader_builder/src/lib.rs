static IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fat.img"));

pub fn get_image() -> &'static [u8] {
    IMAGE
}
