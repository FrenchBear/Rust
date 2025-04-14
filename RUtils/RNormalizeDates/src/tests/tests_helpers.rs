// RNormalizeDates tests
// Helpers tests
//
// 2025-04-14   PV

#![cfg(test)]

use crate::*;

#[test]
fn test_get_year_num() {
    assert_eq!(get_year_num("1984"), 1984);
    assert_eq!(get_year_num("84"), 1984);
    assert_eq!(get_year_num("25"), 2025);
}

#[test]
fn test_get_month_num() {
    assert_eq!(get_month_num("Janvier"), 1);
    assert_eq!(get_month_num("07"), 7);
    assert_eq!(get_month_num("F vrier"), 2);
    assert_eq!(get_month_num("été"), 14);
    assert_eq!(get_month_num("HIVER"), 16);
}

#[test]
fn test_get_day_num() {
    assert_eq!(get_day_num("12"), 12);
    assert_eq!(get_day_num("31"), 31);
    assert_eq!(get_day_num("1er"), 1);
}

#[test]
fn test_get_month_name() {
    assert_eq!(get_month_name(12), "12");
    assert_eq!(get_month_name(1), "01");
    assert_eq!(get_month_name(13), "Printemps");
    assert_eq!(get_month_name(16), "Hiver");
}
