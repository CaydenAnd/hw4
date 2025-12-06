use homework4::*;
use std::collections::HashMap;

#[test]
fn fib_twenty() {
    let v = fib(20);
    assert_eq!(v[19], 4181);
}

#[test]
fn big_palindrome() {
    assert!(is_palindrome(1223221));
    assert!(!is_palindrome(12345678));
}

#[test]
fn nthmax_negative() {
    let arr = [-5, -1, -3, -2];
    assert_eq!(nthmax(0, &arr), Some(-1));
    assert_eq!(nthmax(3, &arr), Some(-5));
}

#[test]
fn zip_basic() {
    let a = vec!["a".into(), "b".into()];
    let b = vec!["1".into(), "2".into()];
    let m = zip_hash(&a, &b).unwrap();
    assert_eq!(m.get("b"), Some(&"2".into()));
}

#[test]
fn phone_unlisted_multi() {
    let mut pb = PhoneBook::new();
    pb.add("A".into(), "111-111-1111".into(), false);
    pb.add("B".into(), "111-111-1111".into(), false);
    pb.add("C".into(), "111-111-1111".into(), false);
    let n = pb.names_by_ac("111");
    assert_eq!(n.len(), 3);
}
