use std::{path::PathBuf, process::Command};

pub fn run_cmd(cmd: &str, args: &[&str]) {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .expect(&format!("failed to execute {}", cmd));

    if !status.success() {
        panic!("command `{}` failed with status {}", cmd, status);
    }
}

pub fn main() {
    let bootloader_path =
        std::env::var("CARGO_BIN_FILE_MATRIX_BOOTLOADER").expect("not bootloader???");

    let out_dir_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_dir = out_dir_path.to_str().unwrap();

    let path_buff = out_dir_path.join("fat.img");
    let output_file = path_buff.as_path().to_str().unwrap();

    run_cmd(
        "dd",
        &[
            "if=/dev/zero",
            format!("of={}", output_file).as_str(),
            "bs=1k",
            "count=1440",
        ],
    );
    run_cmd("mkdir", &["-p", out_dir]);
    run_cmd("mformat", &["-i", output_file, "-f", "1440", "::"]);
    run_cmd("mmd", &["-i", output_file, "::/EFI"]);
    run_cmd("mmd", &["-i", output_file, "::/EFI/BOOT"]);
    run_cmd(
        "mcopy",
        &[
            "-i",
            &output_file,
            &bootloader_path,
            "::/EFI/BOOT/BOOTX64.EFI",
        ],
    );
    run_cmd(
        "mcopy",
        &[
            "-i",
            &output_file,
            "startup.nsh",
            "::/startup.nsh",
        ],
    );
}
