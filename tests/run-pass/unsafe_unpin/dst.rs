#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use pin_project::pin_project;

#[pin_project(UnsafeUnpin)]
pub struct A<T: ?Sized> {
    x: T,
}

#[pin_project(UnsafeUnpin)]
pub struct B<T: ?Sized> {
    #[pin]
    x: T,
}

#[pin_project(UnsafeUnpin)]
pub struct C<T: ?Sized>(T);

#[pin_project(UnsafeUnpin)]
pub struct D<T: ?Sized>(#[pin] T);

fn main() {}
