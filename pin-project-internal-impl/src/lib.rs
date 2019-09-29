//! An internal crate to support pin_project - **do not use directly**

#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/pin-project-internal/0.4.0-beta.1")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub, single_use_lifetimes)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::use_self)]
#![cfg_attr(proc_macro_def_site, feature(proc_macro_def_site))]

extern crate proc_macro;

#[macro_use]
mod utils;

mod pin_project;
mod pinned_drop;
mod project;

use proc_macro::TokenStream;
use syn::parse::Nothing;

use utils::{Immutable, Mutable};

#[doc(hidden)]
pub fn pin_project(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input);
    pin_project::attribute(args.into(), input).into()
}

#[doc(hidden)]
pub fn pinned_drop(args: TokenStream, input: TokenStream) -> TokenStream {
    let _: Nothing = syn::parse_macro_input!(args);
    let input = syn::parse_macro_input!(input);
    pinned_drop::attribute(input).into()
}

#[doc(hidden)]
pub fn project(args: TokenStream, input: TokenStream) -> TokenStream {
    let _: Nothing = syn::parse_macro_input!(args);
    let input = syn::parse_macro_input!(input);
    project::attribute(input, Mutable).into()
}

#[doc(hidden)]
pub fn project_ref(args: TokenStream, input: TokenStream) -> TokenStream {
    let _: Nothing = syn::parse_macro_input!(args);
    let input = syn::parse_macro_input!(input);
    project::attribute(input, Immutable).into()
}

#[doc(hidden)]
pub fn derive_unpin(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input);
    pin_project::derive(input).into()
}
