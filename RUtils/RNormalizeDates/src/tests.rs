// RNormalizeDates tests
//
// 2025-04-14   PV

#![cfg(test)]

use super::*;

#[test]
fn test_re_dmy() {
    let d = DatePatterns::new();

    assert!(d.re_date_dmy.is_match("xx 26 02 2025 xx"));
    assert!(d.re_date_dmy.is_match("xx 26 F vrier 1965 xx"));
    assert!(d.re_date_dmy.is_match("xx 26 Février 1965 xx"));
    assert!(d.re_date_dmy.is_match("xx 26 Fevrier 1965 xx"));
    assert!(d.re_date_dmy.is_match("xx 31 Jan 1995 xx"));
    assert!(d.re_date_dmy.is_match("xx 03 12 2024 xx"));
}

#[test]
fn test_re_dmdmy() {
    let d = DatePatterns::new();

    assert!(d.re_date_dmdmy.is_match("xx 26 02 au 13 03 2025 xx"));
    assert!(d.re_date_dmdmy.is_match("xx 1 jan 31 12 2025 xx"));
}

#[test]
fn test_re_mymy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mymy.is_match("xx 12 2024 - 02 2025 xx"));
    assert!(d.re_date_mymy.is_match("xx déc 2024 à fév 2025 xx"));
    assert!(d.re_date_mymy.is_match("xx June 2024 January 2025 xx"));
}

#[test]
fn test_re_mmy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mmy.is_match("xx 01 03 2025 xx"));
    assert!(d.re_date_mmy.is_match("xx jan à mar 2025 xx"));
    assert!(d.re_date_mmy.is_match("xx July - August 2025 xx"));
}

#[test]
fn test_re_my() {
    let d = DatePatterns::new();

    assert!(d.re_date_my.is_match("xx 01 2025 xx"));
    assert!(d.re_date_my.is_match("xx jan 2025 xx"));
    assert!(d.re_date_my.is_match("xx August 2025 xx"));
}

#[test]
fn test_re_ym() {
    let d = DatePatterns::new();

    assert!(d.re_date_ym.is_match("xx 2025 01 xx"));
    assert!(d.re_date_ym.is_match("xx 2024 jan xx"));
    assert!(d.re_date_ym.is_match("xx 2023    August xx"));
}

#[test]
fn test_re_ymd() {
    let d = DatePatterns::new();

    assert!(d.re_date_ymd.is_match("xx 2025 01 01 xx"));
    assert!(d.re_date_ymd.is_match("xx 2024 jan  15 xx"));
    assert!(d.re_date_ymd.is_match("xx 2023   August 31 xx"));
}