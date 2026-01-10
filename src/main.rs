use std::process::Command;

use matrix_bootloader_builder::get_bootloader_image;
use matrix_kernel_builder::get_kernel_image;

fn main() {
    std::fs::write("matrix_bootloader.img", get_bootloader_image())
        .expect("faliled to write image");
    std::fs::write("matrix_os.img", get_kernel_image()).expect("faliled to write image");

    let status = Command::new("qemu-system-x86_64")
        .args([
            "-L",
            "OVMF",
            "-drive",
            "if=pflash,format=raw,readonly=on,file=OVMF/OVMF.4m.fd",
            "-drive",
            "file=matrix_os.img,format=raw",
            "-drive",
            "file=matrix_bootloader.img,format=raw"
        ])
        .status()
        .expect("failed to run qemu");

    if !status.success() {
        panic!("command failed with status {}", status);
    }
}
