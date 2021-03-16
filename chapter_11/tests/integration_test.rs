use adder;
mod common;

#[test]
fn id_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
