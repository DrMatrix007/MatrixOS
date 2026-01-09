use matrix_images::image::Image;
use std::{env, fs, path::PathBuf};

fn main() {
    let kernel_path =
        env::var("CARGO_BIN_FILE_MATRIX_KERNEL").expect("CARGO_BIN_FILE_MATRIX_KERNEL is not set");

    let kernel = fs::read(&kernel_path).expect("failed to read kernel binary");

    let out_path =
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set")).join("kernel.img");

    let mut image = Image::new(&out_path).expect("failed to create image");

    image.write_new_file("/kernel.mat", &kernel).unwrap();
}
