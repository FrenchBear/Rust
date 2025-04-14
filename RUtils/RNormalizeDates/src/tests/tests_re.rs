// RNormalizeDates tests
// Regular expressions tests
//
// 2025-04-14   PV

#![cfg(test)]

use crate::*;

#[test]
fn test_re_ymd_head() {
    let d = DatePatterns::new();

    // re_date_ymd_head includes a mandatory space at the beginning
    assert!(d.re_date_ymd_head.is_match(" 2023-02-10 Echappee Belle Magazine"));
    assert!(d.re_date_ymd_head.is_match(" 2023-03-01 Grands Reportages"));
}

#[test]
fn test_re_ynm() {
    let d = DatePatterns::new();

    // re_date_ymd_head includes a mandatory space at the beginning
    assert!(d.re_date_ynm.is_match("BBC Science Focus 2024 №413 December"));
    assert!(d.re_date_ynm.is_match("National Geographic 2025 №04 April"));
    assert!(d.re_date_ynm.is_match("Astronomy 2024 Volume 53 №02 February"));
}

// ---

#[test]
fn test_re_mymy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mymy.is_match("xx 12 2024 - 02 2025 xx"));
    assert!(d.re_date_mymy.is_match("xx déc 2024 à fév 2025 xx"));
    assert!(d.re_date_mymy.is_match("xx June 2024 January 2025 xx"));
}

#[test]
fn test_re_ymym() {
    let d = DatePatterns::new();

    assert!(d.re_date_ymym.is_match("xx 2024 12 - 2025 02 xx"));
}

// ---

#[test]
fn test_re_mmmy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mmmy.is_match("xx 01-02-03 2025 xx"));
    assert!(d.re_date_mmmy.is_match("xx jan-fév-mar 2025 xx"));
    assert!(d.re_date_mmmy.is_match("xx June-July-August 2025 xx"));
}

#[test]
fn test_re_ymmm() {
    let d = DatePatterns::new();

    assert!(d.re_date_ymmm.is_match("xx 2025 01-02-03 xx"));
    assert!(d.re_date_ymmm.is_match("xx 2023 jan fév mar xx"));
    assert!(d.re_date_ymmm.is_match("xx 2024-June-July-August xx"));
}

// ---

#[test]
fn test_re_mymmy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mymmy.is_match("L'essentiel de la science - 12 2024 01-02 2025"));
}

#[test]
fn test_re_ymymm() {
    let d = DatePatterns::new();

    assert!(d.re_date_ymymm.is_match("L'essentiel de la science - 2024 12 2025 01-02"));
    assert!(d.re_date_ymymm.is_match("xx 2024 12 - 2025 01 02 xx"));
}

// ---

#[test]
fn test_re_mmymy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mmymy.is_match("Revue - 11-12 2024 01 2025"));
}

#[test]
fn test_re_ymmym() {
    let d = DatePatterns::new();

    assert!(d.re_date_ymmym.is_match("Revue - 2024 11-12 2025 01"));
    assert!(d.re_date_ymmym.is_match("xx 2024 11 12 - 2025 01 xx"));
}

// ---

#[test]
fn test_re_mmy() {
    let d = DatePatterns::new();

    assert!(d.re_date_mmy.is_match("xx 01 03 2025 xx"));
    assert!(d.re_date_mmy.is_match("xx jan à mar 2025 xx"));
    assert!(d.re_date_mmy.is_match("xx July - August 2025 xx"));
}

#[test]
fn test_re_ymm() {
    let d = DatePatterns::new();

    assert!(d.re_date_ymm.is_match("Elektor n°511 2025-01-02"));
}

// ---

#[test]
fn test_re_dmdmy() {
    let d = DatePatterns::new();

    assert!(d.re_date_dmdmy.is_match("xx 26 02 au 13 03 2025 xx"));
    assert!(d.re_date_dmdmy.is_match("xx 1 jan 31 12 2025 xx"));
}

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
fn test_re_ymd() {
    let d = DatePatterns::new();

    assert!(d.re_date_ymd.is_match("xx 2025 01 01 xx"));
    assert!(d.re_date_ymd.is_match("xx 2024 jan  15 xx"));
    assert!(d.re_date_ymd.is_match("xx 2023   August 31 xx"));
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
    assert!(d.re_date_ym.is_match("CervPsycho99 - 2018-05"));
    assert!(d.re_date_ym.is_match("Voyages à l'Ouest 2024-11-12 $$$ 2024-11..12"));
    assert!(d.re_date_ym.is_match("Voyages à l'Ouest 2024-11-12"));
    assert!(d.re_date_ym.is_match("Voyages à l'Ouest 2024-11"));
    assert!(d.re_date_ym.is_match("Voyages à l'Ouest 2024-11 - Hello"));
}
