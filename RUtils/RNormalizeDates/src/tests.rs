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
fn test_re_mmmy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mmmy.is_match("xx 01-02-03 2025 xx"));
    assert!(d.re_date_mmmy.is_match("xx jan-fév-mar 2025 xx"));
    assert!(d.re_date_mmmy.is_match("xx June-July-August 2025 xx"));
}

#[test]
fn test_re_mymmy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mymmy.is_match("L'essentiel de la science - 12 2024 01-02 2025"));
}

#[test]
fn test_re_mmymy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mmymy.is_match("Revue - 11-12 2024 01 2025"));
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

#[test]
fn test_re_ymm() {
    let d = DatePatterns::new();

    assert!(d.re_date_ymm.is_match("Elektor n°511 2025-01-02"));
}

#[test]
fn test_re_ymd_head() {
    let d = DatePatterns::new();

    // re_date_ymd_head includes a mandatory space at the beginning
    assert!(d.re_date_ymd_head.is_match(" 2023-02-10 Echappee Belle Magazine"));
    assert!(d.re_date_ymd_head.is_match(" 2023-03-01 Grands Reportages"));
}

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
