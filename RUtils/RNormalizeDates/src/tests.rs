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