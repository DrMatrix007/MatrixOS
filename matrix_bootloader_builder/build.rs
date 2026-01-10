use matrix_images::image::Image;
use std::{env, fs, path::PathBuf};

fn main() {
    let bootloader_path = env::var("CARGO_BIN_FILE_MATRIX_BOOTLOADER")
        .expect("CARGO_BIN_FILE_MATRIX_BOOTLOADER is not set");

    let bootloader = fs::read(&bootloader_path).expect("failed to read bootloader binary");

    let out_path =
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set")).join("bootloader.img");

    let mut image = Image::new(&out_path).expect("failed to create image");

    image.create_dir("/EFI/").unwrap();
    image.create_dir("/EFI/BOOT/").unwrap();

    image
        .write_new_file("/EFI/BOOT/BOOTX64.EFI", &bootloader)
        .unwrap();

    image.create_dir("/matrix/").unwrap();

    image
        .write_new_file("/matrix/config.json", include_bytes!("config.json"))
        .unwrap();
}
