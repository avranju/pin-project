#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]
#![allow(dead_code)]

use pin_project::pin_project;

#[pin_project]
struct A {
    #[pin]
    field: u8,
}

mod project_use_1 {
    use crate::A;
    use core::pin::Pin;
    use pin_project::project_ref;

    #[project_ref]
    use crate::A;

    #[project_ref]
    fn project_use() {
        let mut x = A { field: 0 };
        #[project_ref]
        let A { field } = Pin::new(&mut x).into_ref().project_ref();
        let _: Pin<&u8> = field;
    }
}

mod project_use_2 {
    #[project_ref]
    use crate::A;
    use pin_project::project_ref;

    #[project_ref]
    impl A {
        fn project_use(self) {}
    }
}

fn main() {}
