// Write test for src/lib.rs

use module1::add;
use module1::div;
use module1::mul;
use module1::sub;

#[test]
fn test_add() {
    assert_eq!(add(3, 2), 5);
}

#[test]
fn test_sub() {
    assert_eq!(sub(3, 2), 1);
}

#[test]
fn test_mul() {
    assert_eq!(mul(3, 2), 6);
}

#[test]
fn test_div() {
    assert_eq!(div(3, 2), 1);
}
