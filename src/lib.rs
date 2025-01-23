#[allow(dead_code, non_upper_case_globals, non_camel_case_types)]
#[allow(rustdoc::all)]
mod bindgen {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindgen::*;
