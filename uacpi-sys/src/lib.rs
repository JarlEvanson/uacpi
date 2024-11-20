//! Raw bindings to [uACPI](https://github.com/UltraOS/uACPI).

#![no_std]
#![expect(non_camel_case_types)]
#![expect(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
