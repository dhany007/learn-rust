use chap12_test_organization::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(3,4);
    assert_eq!(result, 7);
}