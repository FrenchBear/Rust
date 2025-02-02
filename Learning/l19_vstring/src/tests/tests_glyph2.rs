// vstring unit tests - Glyph2 tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
use crate::glyph2::Glyph2;

#[test]
fn test_glyph2_simple() {
    let s = "AB";
    let v = Glyph2::glyph2_indices(s).collect::<Vec<Glyph2>>();

    assert!(v.len() == 2);
    assert!(v[0].byte_range == (0..1));
    assert!(v[0].char_range == (0..1));
    assert!(v[1].byte_range == (1..2));
    assert!(v[1].char_range == (1..2));
}

#[test]
fn test_glyph2_combining_accent() {
    let s = "ae\u{0301}z";
    // UTF8/byte: 61 'a', 65 'e', CC 81 {COMBINING ACUTE ACCENT}, 7A 'z'
    // Codepoints/char: U+0061, U+0065, U+0301, U+007A
    // Glyps: a é z
    let v = Glyph2::glyph2_indices(s).collect::<Vec<Glyph2>>();

    assert!(v.len() == 3);
    assert!(v[0].byte_range == (0..1));
    assert!(v[0].char_range == (0..1));
    assert!(v[1].byte_range == (1..4));
    assert!(v[1].char_range == (1..3));
    assert!(v[2].byte_range == (4..5));
    assert!(v[2].char_range == (3..4));
}

#[test]
fn tests_glyph2_empty() {
    let v = Glyph2::glyph2_indices("").collect::<Vec<Glyph2>>();
    assert!(v.len() == 0);
}
