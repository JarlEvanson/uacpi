//! Build script for raw bindings to [uACPI](https://github.com/UltraOS/uACPI).

use std::{ffi::OsStr, path::{Path, PathBuf}};

const UACPI: &str = "../vendor/uacpi";

fn main() {
    let uacpi = PathBuf::from(UACPI)
        .canonicalize()
        .expect("failed to canonicalize UACPI path");

    println!("cargo::rerun-if-changed={}", uacpi.display());
    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=build.rs");

    let sized_frees = std::env::var_os("CARGO_FEATURE_SIZED_FREES").is_some();

    build_uacpi(&uacpi, sized_frees);
}

fn build_uacpi(uacpi: &Path, sized_frees: bool) {
    let mut cc = cc::Build::new();
    cc.include(uacpi.join("include"));

    let directory =
        std::fs::read_dir(uacpi.join("source")).expect("failed to read UACPI directory");
    for entry in directory
        .into_iter()
        .map(|entry| entry.expect("failed to retrieve directory entry"))
    {
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|extension| extension == OsStr::new("c"))
        {
            cc.file(path);
        }
    }

    if sized_frees {
        cc.define("UACPI_SIZED_FREES", "1");
    }

    cc.flag("-nostdlib").flag("-ffreestanding");
    cc.flag("-fno-stack-protector").flag("-mgeneral-regs-only");

    let target = std::env::var("TARGET").expect("Cargo failed to set TARGET");
    println!("{target}");
    if target.starts_with("x86") {
        cc.flag("-mno-red-zone");
    }

    cc.compile("uacpi-sys");
}
