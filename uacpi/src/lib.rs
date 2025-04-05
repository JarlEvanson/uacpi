//! Rust bindings to [uACPI](https://github.com/UltraOS/uACPI).

#![no_std]
#![allow(missing_docs)]

use status::Status;

pub mod status;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InitializationFlags(u64);

pub fn initialize(flags: InitializationFlags) -> Status {
    let result = unsafe { uacpi_sys::uacpi_initialize(flags.0) };

    if result 
}


