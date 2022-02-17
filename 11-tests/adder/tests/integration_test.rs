use adder;

mod common;

#[test]
fn it_adds_three_integration() {
    common::setup();
    assert_eq!(5, adder::add_two(2));
}
