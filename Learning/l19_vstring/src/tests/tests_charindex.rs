// vstring unit tests - Functions based on char index tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
use crate::*;

// ------------------------
// test validate_charindex

#[test]
fn test_validate_charindex() {
    assert_eq!(validate_charindex("ABC", 1), 1..2);
    assert_eq!(validate_charindex("Aé♫山𝄞🐗", 4), 9..13);
}

#[test]
#[should_panic(expected = "char index out of bounds: &str contains 3 character(s), but the index is 5")]
fn test_validate_charindex_panic_out_of_bounds() {
    let _ = validate_charindex("ABC", 5);
}

// ------------------------
// test get char

#[test]
fn test_char_from_charindex_normal() {
    let s = "Aé♫山𝄞🐗";
    assert_eq!(get_char_from_charindex(s, 0), 'A');
    assert_eq!(get_char_from_charindex(s, get_char_length(s) - 1), '\u{1F417}'); // U+1F417 BOAR = UTF8: F0 9F 90 97
    assert_eq!(get_char_from_charindex("🐻‍❄️", 2), '❄'); // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
}

#[should_panic]
#[test]
fn test_char_from_charindex_panic_out_of_bounds() {
    let _ = get_char_from_charindex("abc", 5);
}

#[test]
fn test_charoption_from_charindex() {
    assert_eq!(get_charoption_from_charindex("Aé♫山𝄞🐗", 1), Some('é'));
    assert_eq!(get_charoption_from_charindex("abc", 5), None);
}

// ------------------------
// test get glyph

#[test]
fn test_glyph_from_charindex_normal() {
    assert_eq!(
        get_glyph_from_charindex("<🐻‍❄️>", 0),
        Glyph2 {
            char_range: 0..1,
            byte_range: 0..1
        }
    );
    assert_eq!(
        get_glyph_from_charindex("<🐻‍❄️>", 1),
        Glyph2 {
            byte_range: 1..14,
            char_range: 1..5
        }
    );
}

#[should_panic(expected = "char index out of bounds: &str contains 3 character(s), but the index is 5")]
#[test]
fn test_glyph_from_charindex_panic_out_of_bounds() {
    let _ = get_char_from_charindex("abc", 5);
}

#[test]
fn test_glyphoption_from_charindex() {
    assert_eq!(
        get_glyphoption_from_charindex("<🐻‍❄️>", 1),
        Some(Glyph2 {
            byte_range: 1..14,
            char_range: 1..5
        })
    );

    assert_eq!(get_glyphoption_from_charindex("Hello", 10), None);
}

// ------------------------
// test byte slice

#[test]
pub fn test_byteslice_from_charindex() {
    assert_eq!(get_byteslice_from_charindex("Où ça?", 3), [0xC3, 0xA7]);
}

// ------------------------
// test byte vector

#[test]
pub fn test_bytevector_from_charindex() {
    assert_eq!(get_bytevector_from_charindex("Où ça?", 3), vec![0xC3, 0xA7]);
}

// ------------------------
// test char vector

#[test]
pub fn test_charvector_from_charindex() {
    assert_eq!(get_charvector_from_charindex("Où ça?", 3), vec!['ç']);
}

// ------------------------
// test glyph vector

#[test]
pub fn test_glyphvector_from_charindex() {
    assert_eq!(
        get_glyphvector_from_charindex("<e\u{0301}>", 1),
        vec![Glyph2 {
            byte_range: 1..4,
            char_range: 1..3
        }]
    );
}

// ------------------------
// test byte iterator

#[test]
pub fn test_byteiterator_from_charindex() {
    let s = "Aé♫山𝄞🐗";

    let mut it = get_byteiterator_from_charindex(s, 1); // é
    assert_eq!(it.next(), Some(0xC3));
    assert_eq!(it.next(), Some(0xA9));
    assert!(it.next().is_none());

    let mut it = get_byteiterator_from_charindex(s, 4); // 𝄞
    assert_eq!(it.next(), Some(0xF0));
    assert_eq!(it.next(), Some(0x9D));
    assert_eq!(it.next(), Some(0x84));
    assert_eq!(it.next(), Some(0x9E));
    assert!(it.next().is_none());
}

// ------------------------
// test char iterator

#[test]
pub fn test_chariterator_from_charindex() {
    let s = "Aé♫山𝄞🐗";

    let mut it = get_chariterator_from_charindex(s, 1); // é
    assert_eq!(it.next(), Some('é'));
    assert!(it.next().is_none());

    let mut it = get_chariterator_from_charindex(s, 4); // 𝄞
    assert_eq!(it.next(), Some('𝄞'));
    assert!(it.next().is_none());
}

// ------------------------
// test glyph iterator

#[test]
pub fn test_glyphiterator_from_charindex() {
    let mut it = get_glyphiterator_from_charindex("<e\u{0301}>", 1);
    assert_eq!(
        it.next(),
        Some(Glyph2 {
            byte_range: 1..4,
            char_range: 1..3
        })
    );
    assert!(it.next().is_none());
}

// ------------------------
// test str&

#[test]
pub fn test_strref_from_charindex() {
    let s = "Aé♫山𝄞🐗";
    assert_eq!(get_strref_from_charindex(s, 1), "é");
    assert_eq!(get_strref_from_charindex(s, 5), "🐗");
}

// ------------------------
// test String

#[test]
pub fn test_string_from_charindex() {
    let s = "Aé♫山𝄞🐗";
    assert_eq!(get_string_from_charindex(s, 1), "é".to_string());
    assert_eq!(get_string_from_charindex(s, 5), "🐗".to_string());
}
