#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use pin_project::pin_project;

// Since `cfg(any())` can never be true, it is okay for this to pass.
#[pin_project]
#[cfg_attr(any(), repr(packed))]
struct Struct {
    #[pin]
    field: u32,
}

fn main() {}
