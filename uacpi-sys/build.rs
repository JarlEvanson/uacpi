//! Build script for raw bindings to [uACPI](https://github.com/UltraOS/uACPI).

use std::ffi::OsStr;

fn main() {
    build_uacpi();
}

fn build_uacpi() {
    let mut cc = cc::Build::new();
    cc.include("vendor/include");

    let directory = std::fs::read_dir("vendor/source").unwrap();
    for entry in directory.into_iter().map(|entry| entry.unwrap()) {
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|extension| extension == OsStr::new("c"))
        {
            cc.file(path);
        }
    }

    cc.flag("-nostdlib").flag("-ffreestanding");
    cc.flag("-fno-stack-protector").flag("-mgeneral-regs-only");

    let target = std::env::var("TARGET").unwrap();
    let architecture = target.split_once('-').unwrap().0;
    if architecture == "x86_64" {
        cc.flag("-mno-red-zone");
    }

    cc.compile("uacpi-sys");
}
