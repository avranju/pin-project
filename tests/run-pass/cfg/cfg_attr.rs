#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use pin_project::pin_project;
use std::{marker::PhantomPinned, pin::Pin};

fn is_unpin<T: Unpin>() {}

#[cfg(target_os = "linux")]
pub struct Linux;
#[cfg(not(target_os = "linux"))]
pub struct Other;

// Use this type to check that `cfg(any())` is working properly.
// If `cfg(any())` is not working properly, `is_unpin` will fail.
pub struct Any(PhantomPinned);

fn main() {
    #[pin_project]
    pub struct SameCfg {
        #[cfg(target_os = "linux")]
        #[cfg_attr(target_os = "linux", pin)]
        inner: Linux,
        #[cfg(not(target_os = "linux"))]
        #[cfg_attr(not(target_os = "linux"), pin)]
        inner: Other,
        #[cfg(any())]
        #[cfg_attr(any(), pin)]
        any: Any,
    }

    is_unpin::<SameCfg>();

    #[cfg(target_os = "linux")]
    let mut x = SameCfg { inner: Linux };
    #[cfg(not(target_os = "linux"))]
    let mut x = SameCfg { inner: Other };

    let x = Pin::new(&mut x).project();
    #[cfg(target_os = "linux")]
    let _: Pin<&mut Linux> = x.inner;
    #[cfg(not(target_os = "linux"))]
    let _: Pin<&mut Other> = x.inner;

    #[pin_project]
    pub struct DifferentCfg {
        #[cfg(target_os = "linux")]
        #[cfg_attr(target_os = "linux", pin)]
        inner: Linux,
        #[cfg(not(target_os = "linux"))]
        #[cfg_attr(target_os = "linux", pin)]
        inner: Other,
        #[cfg(any())]
        #[cfg_attr(any(), pin)]
        any: Any,
    }

    is_unpin::<DifferentCfg>();

    #[cfg(target_os = "linux")]
    let mut x = DifferentCfg { inner: Linux };
    #[cfg(not(target_os = "linux"))]
    let mut x = DifferentCfg { inner: Other };

    let x = Pin::new(&mut x).project();
    #[cfg(target_os = "linux")]
    let _: Pin<&mut Linux> = x.inner;
    #[cfg(not(target_os = "linux"))]
    let _: &mut Other = x.inner;

    #[cfg_attr(not(any()), pin_project)]
    struct Foo<T> {
        #[cfg_attr(not(any()), pin)]
        inner: T,
    }

    let mut x = Foo { inner: 0_u8 };
    let x = Pin::new(&mut x).project();
    let _: Pin<&mut u8> = x.inner;
}
