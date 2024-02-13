use super::*;

use std::cmp::Ordering;

#[test]
fn should_convert_str_to_float() {
    assert_eq!(1.23, str_to_f64("1.23"))
}

#[test]
fn should_convert_str_with_thousands_separator_to_float1() {
    assert_eq!(1600.0, str_to_f64("1,600"))
}

#[test]
fn should_convert_str_with_thousands_separator_to_float2() {
    assert_eq!(1600.0, str_to_f64("1 600"))
}

#[test]
fn should_fail_to_convert_bad_str_to_float1() {
    assert_eq!(std::f64::INFINITY, str_to_f64("1.s23"))
}

#[test]
fn should_fail_to_convert_bad_str_to_float2() {
    assert_eq!(std::f64::INFINITY, str_to_f64("unknown"))
}

#[test]
fn should_compare_two_floats() {
    assert_eq!(Ordering::Less, compare_nums(1.0, 1.2));
    assert_eq!(Ordering::Greater, compare_nums(3.0, 1.2));
    assert_eq!(Ordering::Equal, compare_nums(1.2, 1.2));
}

#[test]
fn should_compare_two_float_strs() {
    assert_eq!(Ordering::Less, compare_strs_as_f64s("1.0", "1.2"));
    assert_eq!(Ordering::Greater, compare_strs_as_f64s("3.0", "1.2"));
    assert_eq!(Ordering::Equal, compare_strs_as_f64s("1.2", "1.2"));
    assert_eq!(Ordering::Greater, compare_strs_as_f64s("unknown", "1.2"));
}
