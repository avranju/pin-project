#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use pin_project::{pin_project, project_ref};
use std::pin::Pin;

#[project_ref] // Nightly does not need a dummy attribute to the function.
fn stmt_expr() {
    // struct

    #[pin_project]
    struct Foo<T, U> {
        #[pin]
        field1: T,
        field2: U,
    }

    let foo = Foo { field1: 1, field2: 2 };

    #[project_ref]
    let Foo { field1, field2 } = Pin::new(&foo).project_ref();

    let x: Pin<&i32> = field1;
    assert_eq!(*x, 1);

    let y: &i32 = field2;
    assert_eq!(*y, 2);

    // tuple struct

    #[pin_project]
    struct Bar<T, U>(#[pin] T, U);

    let bar = Bar(1, 2);

    #[project_ref]
    let Bar(x, y) = Pin::new(&bar).project_ref();

    let x: Pin<&i32> = x;
    assert_eq!(*x, 1);

    let y: &i32 = y;
    assert_eq!(*y, 2);

    // enum

    #[pin_project]
    enum Baz<A, B, C, D> {
        Variant1(#[pin] A, B),
        Variant2 {
            #[pin]
            field1: C,
            field2: D,
        },
        None,
    }

    let baz = Baz::Variant1(1, 2);

    let baz = Pin::new(&baz).project_ref();

    #[project_ref]
    match &baz {
        Baz::Variant1(x, y) => {
            let x: &Pin<&i32> = x;
            assert_eq!(**x, 1);

            let y: &&i32 = y;
            assert_eq!(**y, 2);
        }
        Baz::Variant2 { field1, field2 } => {
            let _x: &Pin<&i32> = field1;
            let _y: &&i32 = field2;
        }
        Baz::None => {}
    }

    #[project_ref]
    let val = match &baz {
        Baz::Variant1(_, _) => true,
        Baz::Variant2 { .. } => false,
        Baz::None => false,
    };
    assert_eq!(val, true);
}

fn main() {}
