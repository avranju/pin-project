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
    use pin_project::project;

    #[project]
    use crate::A;

    #[project]
    fn project_use() {
        let mut x = A { field: 0 };
        #[project]
        let A { field } = Pin::new(&mut x).project();
        let _: Pin<&mut u8> = field;
    }
}

mod project_use_2 {
    #[project]
    use crate::A;
    use pin_project::project;

    #[project]
    impl A {
        fn project_use(self) {}
    }
}

fn main() {}
