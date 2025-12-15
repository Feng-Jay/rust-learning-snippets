use adder::add_two;
mod common;

use common::setup;

#[test]
fn integrated() {
    setup();
    let result = add_two(2);
    assert_eq!(result, 4);
}