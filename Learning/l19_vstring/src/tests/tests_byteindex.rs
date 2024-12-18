// vstring unit tests - Test functions based on byte indexes
//
// 2024-12-13   PV      First version

#[cfg(test)]
use crate::*;

// ------------------------
// get byte

#[test]
fn test_byte_from_byteindex_standard() {
    let s = "Aé♫山𝄞🐗";
    assert_eq!(get_byte_from_byteindex(s, 0), 65);
    assert_eq!(get_byte_from_byteindex(s, get_byte_length(s) - 1), 0x97); // U+1F417 BOAR = UTF8: F0 9F 90 97
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_byte_from_byteindex_panic_out_of_bounds() {
    let _ = get_byte_from_byteindex("abc", 5);
}

#[test]
fn test_byteoption_from_byteindex_tests() {
    assert_eq!(get_byteoption_from_byteindex("Aé♫山𝄞🐗", 0), Some(65));
    assert_eq!(get_byteoption_from_byteindex("abc", 5), None); // U+1F417 BOAR = UTF8: F0 9F 90 97
}

// ------------------------
// get char

#[test]
fn test_char_from_byteindex_standard() {
    let s = "Aé♫山𝄞🐗";
    assert_eq!(get_char_from_byteindex(s, 0), 65 as char);
    assert_eq!(get_char_from_byteindex(s, s.len() - 4), '\u{1F417}'); // U+1F417 BOAR = UTF8: F0 9F 90 97
    assert_eq!(get_char_from_byteindex("🐻‍❄️", 7), '❄'); // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
}

#[test]
#[should_panic(expected = "not a char boundary")]
fn test_char_from_byteindex_panic_not_a_char_boundary() {
    let _ = get_char_from_byteindex("🐗", 1); // U+1F417 BOAR = UTF8: F0 9F 90 97
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_char_from_byteindex_panic_out_of_bounds() {
    let _ = get_char_from_byteindex("abc", 5);
}

#[test]
fn test_charoption_from_byteindex_tests() {
    assert_eq!(get_charoption_from_byteindex("Aé♫山𝄞🐗", 0), Some('A'));
    assert_eq!(get_charoption_from_byteindex("abc", 5), None);
    assert_eq!(get_charoption_from_byteindex("🐗", 1), None); // U+1F417 BOAR = UTF8: F0 9F 90 97
}

// ------------------------
// get glyph

#[test]
fn test_glyph_from_byteindex_standard() {
    let s = "Ae\u{0301}𝄞â̧̅🐗🐻‍❄️👨🏾‍❤️‍💋‍👨🏻";

    assert_eq!(
        get_glyph_from_byteindex(s, 0),
        Glyph2 {
            byte_range: (0..1),
            char_range: (0..1)
        }
    ); // A
    assert_eq!(
        get_glyph_from_byteindex(s, 1),
        Glyph2 {
            byte_range: (1..4),
            char_range: (1..3)
        }
    ); // é
    assert_eq!(
        get_glyph_from_byteindex(s, 4),
        Glyph2 {
            byte_range: (4..8),
            char_range: (3..4)
        }
    ); // 𝄞
    assert_eq!(
        get_glyph_from_byteindex(s, 8),
        Glyph2 {
            byte_range: (8..15),
            char_range: (4..8)
        }
    ); // â̧̅
    assert_eq!(
        get_glyph_from_byteindex(s, 15),
        Glyph2 {
            byte_range: (15..19),
            char_range: (8..9)
        }
    ); //🐗
    assert_eq!(
        get_glyph_from_byteindex(s, 19),
        Glyph2 {
            byte_range: (19..32),
            char_range: (9..13)
        }
    ); //🐻‍❄️
    assert_eq!(
        get_glyph_from_byteindex(s, 32),
        Glyph2 {
            byte_range: (32..67),
            char_range: (13..23)
        }
    ); //👨🏾‍❤️‍💋‍👨🏻
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_glyph_from_byteindex_panic_out_of_bounds() {
    let _ = get_glyph_from_byteindex("abc", 5);
}

#[test]
#[should_panic(expected = "not a glyph boundary")]
fn test_glyph_from_byteindex_panic_not_a_glyph_boundary() {
    let _ = get_glyph_from_byteindex("🐗", 1);
}

#[test]
fn test_glyphoption_from_byteindex_standard() {
    assert_eq!(
        get_glyphoption_from_byteindex("ABC", 1),
        Some(Glyph2 {
            byte_range: (1..2),
            char_range: (1..2)
        })
    );
    assert_eq!(get_glyphoption_from_byteindex("ABC", 5), None);
    assert_eq!(get_glyphoption_from_byteindex("🐗", 1), None);
}

// ------------------------
// get byte slice

#[test]
fn test_byteslice_from_byteindex_standard() {
    assert_eq!(get_byteslice_from_byteindex("ABC", 1), ['B' as u8])
}

#[test]
#[should_panic(expected = "out of range")]
fn test_byteslice_from_byteindex_panic_out_of_bounds() {
    let _ = get_byteslice_from_byteindex("ABC", 4);
}

// ------------------------
// get byte vector

#[test]
fn test_bytevector_from_byteindex_standard() {
    assert_eq!(get_bytevector_from_byteindex("ABC", 1), vec!['B' as u8])
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_bytevector_from_byteindex_panic_out_of_bounds() {
    let _ = get_bytevector_from_byteindex("ABC", 4);
}

// ------------------------
// get char vector

#[test]
fn test_charvector_from_byteindex_standard() {
    assert_eq!(get_charvector_from_byteindex("<🐗>", 1), vec!['🐗'])
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_charvector_from_byteindex_panic_out_of_bounds() {
    let _ = get_charvector_from_byteindex("ABC", 4);
}

#[test]
#[should_panic(expected = "not a char boundary")]
fn test_charvector_from_byteindex_panic_not_a_char_boundary() {
    let _ = get_charvector_from_byteindex("<🐗>", 2);
}

// ------------------------
// get glyph vector

#[test]
fn test_glyphvector_from_byteindex_standard() {
    assert_eq!(
        get_glyphvector_from_byteindex("<🐻‍❄️>", 1),
        vec![Glyph2 {
            byte_range: 1..14,
            char_range: 1..5
        }]
    )
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_glyphvector_from_byteindex_panic_out_of_bounds() {
    let _ = get_glyphvector_from_byteindex("<🐻‍❄️>", 123);
}

#[test]
#[should_panic(expected = "not a glyph boundary")]
fn test_glyphvector_from_byteindex_panic_not_a_glyph_boundary() {
    let _ = get_glyphvector_from_byteindex("<🐻‍❄️>", 5);
}

// ------------------------
// get byte iterator

#[test]
fn test_byteiterator_from_byteindex_standard() {
    let mut it = get_byteiterator_from_byteindex("ABC", 1);
    assert!(it.next() == Some('B' as u8));
    assert!(it.next().is_none());
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_byteiterator_from_byteindex_panic_out_of_bounds() {
    let it = get_byteiterator_from_byteindex("ABC", 5);
}

// ------------------------
// get char iterator

#[test]
fn test_chariterator_from_byteindex_standard() {
    let mut it = get_chariterator_from_byteindex("<🐗>", 1);
    assert!(it.next() == Some('🐗'));
    assert!(it.next().is_none());
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_chariterator_from_byteindex_panic_out_of_bounds() {
    let it = get_chariterator_from_byteindex("ABC", 5);
}

#[test]
#[should_panic(expected = "not a char boundary")]
fn test_chariterator_from_byteindex_panic_not_a_char_boundary() {
    let it = get_chariterator_from_byteindex("<🐗>", 2);
}

// ------------------------
// get glyph iterator

#[test]
fn test_glyphiterator_from_byteindex_standard() {
    let mut it = get_glyphiterator_from_byteindex("<🐻‍❄️>", 1);
    assert!(
        it.next()
            == Some(Glyph2 {
                byte_range: 1..14,
                char_range: 1..5
            })
    );
    assert!(it.next().is_none());
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_glyphiterator_from_byteindex_panic_out_of_bounds() {
    let it = get_glyphiterator_from_byteindex("ABC", 5);
}

#[test]
#[should_panic(expected = "not a glyph boundary")]
fn test_glyphiterator_from_byteindex_panic_not_a_glyph_boundary() {
    let _ = get_glyphiterator_from_byteindex("<🐻‍❄️>", 5);
}

// ------------------------
// get strref

#[test]
fn test_strref_from_byteindex_standard() {
    assert_eq!(get_strref_from_byteindex("ABC", 1), "B");
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_strref_from_byteindex_panic_out_of_bounds() {
    let it = get_strref_from_byteindex("ABC", 5);
}

// ------------------------
// get string

#[test]
fn test_string_from_byteindex_standard() {
    assert_eq!(get_string_from_byteindex("ABC", 1), String::from("B"));
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_string_from_byteindex_panic_out_of_bounds() {
    let it = get_string_from_byteindex("ABC", 5);
}
