// vstring unit tests - Functions based on glyph index tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
use crate::*;

// ------------------------
// test validate_glyphindex

#[test]
fn test_validate_glyphindex() {
    assert_eq!(
        validate_glyphindex("ABC", 1),
        Glyph2 {
            byte_range: 1..2,
            char_range: 1..2
        }
    );
    assert_eq!(
        validate_glyphindex("Aé♫山𝄞🐗", 4),
        Glyph2 {
            byte_range: 9..13,
            char_range: 4..5
        }
    );
    assert_eq!(
        validate_glyphindex("ae\u{0301}z", 1),
        Glyph2 {
            byte_range: 1..4,
            char_range: 1..3
        }
    );
}

#[test]
#[should_panic(expected = "glyph index out of bounds: &str contains 1 glyph(s), but the index is 1")]
fn test_validate_glyphindex_panic_out_of_bounds() {
    let _ = validate_glyphindex("🐻‍❄️", 1);
}

// ------------------------
// test get glyph

#[test]
fn test_glyph_from_glyphindex_normal() {
    assert_eq!(
        get_glyph_from_glyphindex("<🐻‍❄️>", 0),
        Glyph2 {
            char_range: 0..1,
            byte_range: 0..1
        }
    );
    assert_eq!(
        get_glyph_from_glyphindex("<🐻‍❄️>", 1),
        Glyph2 {
            byte_range: 1..14,
            char_range: 1..5
        }
    );
}

#[should_panic(expected = "glyph index out of bounds: &str contains 3 glyph(s), but the index is 5")]
#[test]
fn test_glyph_from_glyphindex_panic_out_of_bounds() {
    let _ = get_glyph_from_glyphindex("abc", 5);
}

#[test]
fn test_glyphoption_from_glyphindex() {
    assert_eq!(
        get_glyphoption_from_glyphindex("<🐻‍❄️>", 1),
        Some(Glyph2 {
            byte_range: 1..14,
            char_range: 1..5
        })
    );

    assert_eq!(get_glyphoption_from_glyphindex("Hello", 10), None);
}

// ------------------------
// test byte slice

#[test]
pub fn test_byteslice_from_glyphindex() {
    assert_eq!(get_byteslice_from_glyphindex("Ou\u{0300}?", 1), [0x75, 0xCC, 0x80]);
}

// ------------------------
// test byte vector

#[test]
pub fn test_bytevector_from_glyphindex() {
    let s = "👨‍🚒"; // {MAN}{ZERO WIDTH JOINER}{FIRE ENGINE}
    assert_eq!(
        get_bytevector_from_glyphindex(s, 0),
        vec![0xF0, 0x9F, 0x91, 0xA8, 0xE2, 0x80, 0x8D, 0xF0, 0x9F, 0x9A, 0x92,]
    );
}

// ------------------------
// test char vector

#[test]
pub fn test_charvector_from_glyphindex() {
    let s = "<👨‍🚒>"; // {MAN}{ZERO WIDTH JOINER}{FIRE ENGINE}
    assert_eq!(get_charvector_from_glyphindex(s, 1), vec!['\u{1F468}', '\u{200D}', '\u{1F692}',]);
}

// ------------------------
// test glyph vector

#[test]
pub fn test_glyphvector_from_glyphindex() {
    assert_eq!(
        get_glyphvector_from_glyphindex("<e\u{0301}>", 1),
        vec![Glyph2 {
            byte_range: 1..4,
            char_range: 1..3
        }]
    );
}

// ------------------------
// test byte iterator

#[test]
pub fn test_byteiterator_from_glyphindex() {
    let s = "Aé♫山𝄞🐗";

    let mut it = get_byteiterator_from_glyphindex(s, 4); // 𝄞
    assert_eq!(it.next(), Some(0xF0));
    assert_eq!(it.next(), Some(0x9D));
    assert_eq!(it.next(), Some(0x84));
    assert_eq!(it.next(), Some(0x9E));
    assert!(it.next().is_none());
}

// ------------------------
// test char iterator

#[test]
pub fn test_chariterator_from_glyphindex() {
    let s = "<👨‍🚒>"; // {MAN}{ZERO WIDTH JOINER}{FIRE ENGINE}

    let mut it = get_chariterator_from_glyphindex(s, 1); // é
    assert_eq!(it.next(), Some('\u{1F468}'));
    assert_eq!(it.next(), Some('\u{200D}'));
    assert_eq!(it.next(), Some('\u{1F692}'));
    assert!(it.next().is_none());

    let mut it = get_chariterator_from_glyphindex(s, 2); // 𝄞
    assert_eq!(it.next(), Some('>'));
    assert!(it.next().is_none());
}

// ------------------------
// test glyph iterator

#[test]
pub fn test_glyphiterator_from_glyphindex() {
    let s = "<👨‍🚒>"; // {MAN}{ZERO WIDTH JOINER}{FIRE ENGINE}
    let mut it = get_glyphiterator_from_glyphindex(s, 1);
    assert_eq!(
        it.next(),
        Some(Glyph2 {
            byte_range: 1..12,
            char_range: 1..4
        })
    );
    assert!(it.next().is_none());
}

// ------------------------
// test str&

#[test]
pub fn test_strref_from_glyphindex() {
    let s = "<👨‍🚒>"; // {MAN}{ZERO WIDTH JOINER}{FIRE ENGINE}
    assert_eq!(get_strref_from_glyphindex(s, 1), "👨‍🚒");
    assert_eq!(get_strref_from_glyphindex(s, 2), ">");
}

// ------------------------
// test String

#[test]
pub fn test_string_from_glyphindex() {
    let s = "<👨‍🚒>"; // {MAN}{ZERO WIDTH JOINER}{FIRE ENGINE}
    assert_eq!(get_string_from_glyphindex(s, 1), "👨‍🚒".to_string());
    assert_eq!(get_string_from_glyphindex(s, 2), ">".to_string());
}
