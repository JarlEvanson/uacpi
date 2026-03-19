//! Build script for raw bindings to [uACPI](https://github.com/UltraOS/uACPI).

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

const UACPI: &str = "../vendor/uacpi";

fn main() {
    let uacpi = PathBuf::from(UACPI)
        .canonicalize()
        .expect("failed to canonicalize UACPI path");

    println!("cargo::rerun-if-changed={}", uacpi.display());
    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=build.rs");

    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_SIZED_FREES");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_FULL_HARDWARE");
    println!("cargo::rerun-if-env-changed=CARGO_FEATURE_AML_INTERPRETER");

    let sized_frees = std::env::var_os("CARGO_FEATURE_SIZED_FREES").is_some();
    let full_hardware = std::env::var_os("CARGO_FEATURE_FULL_HARDWARE").is_some();
    let aml_interpreter = std::env::var_os("CARGO_FEATURE_AML_INTERPRETER").is_some();

    create_bindings(&uacpi, sized_frees, full_hardware, aml_interpreter);
    build_uacpi(&uacpi, sized_frees, full_hardware, aml_interpreter);
}

fn create_bindings(uacpi: &Path, sized_frees: bool, full_hardware: bool, aml_interpreter: bool) {
    let headers = [
        uacpi.join("include/uacpi/acpi.h"),
        uacpi.join("include/uacpi/context.h"),
        uacpi.join("include/uacpi/event.h"),
        uacpi.join("include/uacpi/helpers.h"),
        uacpi.join("include/uacpi/io.h"),
        uacpi.join("include/uacpi/kernel_api.h"),
        uacpi.join("include/uacpi/log.h"),
        uacpi.join("include/uacpi/namespace.h"),
        uacpi.join("include/uacpi/notify.h"),
        uacpi.join("include/uacpi/opregion.h"),
        uacpi.join("include/uacpi/osi.h"),
        uacpi.join("include/uacpi/registers.h"),
        uacpi.join("include/uacpi/resources.h"),
        uacpi.join("include/uacpi/sleep.h"),
        uacpi.join("include/uacpi/status.h"),
        uacpi.join("include/uacpi/tables.h"),
        uacpi.join("include/uacpi/types.h"),
        uacpi.join("include/uacpi/uacpi.h"),
        uacpi.join("include/uacpi/utilities.h"),
    ];

    let args = [
        format!("-I{}/include", uacpi.display()),
        "-ffreestanding".to_string(),
    ];

    let mut builder = bindgen::builder()
        .headers(headers.iter().map(|pathbuf| pathbuf.to_string_lossy()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(args)
        .use_core()
        .prepend_enum_name(false)
        .layout_tests(false);

    if sized_frees {
        builder = builder.clang_arg("-DUACPI_SIZED_FREES");
    }

    if !full_hardware {
        builder = builder.clang_arg("-DUACPI_REDUCED_HARDWARE");
    }

    if !aml_interpreter {
        builder = builder.clang_arg("-DUACPI_BAREBONES_MODE");
    }

    let bindings = builder.generate().expect("binding generation failed");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").expect("Cargo failed to set OUT_DIR"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("binding write-out failed");
}

fn build_uacpi(uacpi: &Path, sized_frees: bool, full_hardware: bool, aml_interpreter: bool) {
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

    if !full_hardware {
        cc.define("UACPI_REDUCED_HARDWARE", "1");
    }

    if !aml_interpreter {
        cc.define("UACPI_BAREBONES_MODE", "1");
    }

    cc.flag("-nostdlib").flag("-ffreestanding");
    cc.flag("-fno-stack-protector").flag("-mgeneral-regs-only");

    let target = std::env::var("TARGET").expect("Cargo failed to set TARGET");
    if target.starts_with("x86") {
        cc.flag("-mno-red-zone");
    }

    cc.compile("uacpi-sys");
}
