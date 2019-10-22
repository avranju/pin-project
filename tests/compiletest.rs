#![warn(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]

fn stable(t: &trybuild::TestCases) {
    t.pass("tests/run-pass/cfg/*.rs");
    t.pass("tests/run-pass/pin_project/*.rs");
    t.pass("tests/run-pass/pinned_drop/*.rs");
    t.pass("tests/run-pass/project/*.rs");
    t.pass("tests/run-pass/unsafe_unpin/*.rs");
}

#[rustversion::not(nightly)]
fn nightly(_: &trybuild::TestCases) {}

#[rustversion::nightly]
fn nightly(t: &trybuild::TestCases) {
    t.pass("tests/run-pass/unstable-features/*.rs");

    #[cfg(pin_project_show_unpin_struct)]
    t.compile_fail("tests/compile-fail/cfg/*.rs");
    #[cfg(pin_project_show_unpin_struct)]
    t.compile_fail("tests/compile-fail/pin_project/*.rs");
    #[cfg(pin_project_show_unpin_struct)]
    t.compile_fail("tests/compile-fail/pinned_drop/*.rs");
    #[cfg(pin_project_show_unpin_struct)]
    t.compile_fail("tests/compile-fail/project/*.rs");
    #[cfg(pin_project_show_unpin_struct)]
    t.compile_fail("tests/compile-fail/unsafe_unpin/*.rs");
    #[cfg(pin_project_show_unpin_struct)]
    t.compile_fail("tests/compile-fail/unstable-features/*.rs");
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    stable(&t);
    nightly(&t);
}
