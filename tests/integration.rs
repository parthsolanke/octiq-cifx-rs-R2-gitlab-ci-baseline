use fixture::{add, double};

#[test]
fn add_works_from_outside() {
    assert_eq!(add(40, 2), 42);
}

#[test]
fn double_works_from_outside() {
    assert_eq!(double(5), 10);
}

#[test]
fn add_handles_negatives_from_outside() {
    assert_eq!(add(-10, 4), -6);
}

#[test]
fn double_handles_zero_from_outside() {
    assert_eq!(double(0), 0);
}

#[test]
fn add_handles_zero_from_outside() {
    assert_eq!(add(0, 0), 0);
}

#[test]
fn double_handles_negatives_from_outside() {
    assert_eq!(double(-3), -6);
}
