// Example of integration tests
// 2018-11-29	PV

// No special annotation is required
// tests is a special directory, content is automatically complied and executed when running cargo test

extern crate tests;

// Each file in tests directory is compiled as its own separate crate.
// To use common code for such tests, instead of creating tests/common.rs, do create tests/common/mod.rs (so it's won't be compiled indepdently)
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tests::add_two(2));
}
