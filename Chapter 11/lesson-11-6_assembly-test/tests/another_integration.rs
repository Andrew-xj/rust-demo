use adder;

mod common;

#[test]
fn it_really_adds_two() {
    common::setup();
    assert_eq!(5, adder::add_two(3));
}