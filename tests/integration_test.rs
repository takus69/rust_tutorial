extern crate rust_tutorial;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rust_tutorial::add_two(2));
}