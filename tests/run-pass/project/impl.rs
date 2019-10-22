#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use pin_project::{pin_project, project};
use std::pin::Pin;

#[pin_project]
struct HasGenerics<T, U> {
    #[pin]
    field1: T,
    field2: U,
}

#[project]
impl<T, U> HasGenerics<T, U> {
    fn a(self) {
        let Self { field1, field2 } = self;

        let _x: Pin<&mut T> = field1;
        let _y: &mut U = field2;
    }
}

#[pin_project]
struct NoneGenerics {
    #[pin]
    field1: i32,
    field2: u32,
}

#[project]
impl NoneGenerics {}

#[pin_project]
struct HasLifetimes<'a, T, U> {
    #[pin]
    field1: &'a mut T,
    field2: U,
}

#[project]
impl<T, U> HasLifetimes<'_, T, U> {}

#[pin_project]
struct HasOverlappingLifetimes<'pin, T, U> {
    #[pin]
    field1: &'pin mut T,
    field2: U,
}

#[allow(single_use_lifetimes)]
#[project]
impl<'pin, T, U> HasOverlappingLifetimes<'pin, T, U> {}

#[pin_project]
struct HasOverlappingLifetimes2<T, U> {
    #[pin]
    field1: T,
    field2: U,
}

#[allow(single_use_lifetimes)]
#[project]
impl<T, U> HasOverlappingLifetimes2<T, U> {
    fn foo<'pin>(&'pin self) {}
}

fn main() {}
