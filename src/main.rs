use std::process::Command;

use matrix_bootloader_builder::get_image;

fn main() {
    std::fs::write("matrix_os.img", get_image()).expect("faliled to write image");

    let status = Command::new("qemu-system-x86_64")
        .args(&[
            "-L",
            "OVMF",
            "-drive",
            "if=pflash,format=raw,readonly=on,file=OVMF/OVMF.4m.fd",
            "-cdrom",
            "matrix_os.img",
        ])
        .status()
        .expect(&format!("failed to run qemu"));

    if !status.success() {
        panic!("command failed with status {}", status);
    }
}
