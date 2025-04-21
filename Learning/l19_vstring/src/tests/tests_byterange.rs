// vstring unit tests - Test functions based on byte range indexes
//
// 2024-12-13   PV      First version
// 2025-04-21   PV      Clippy optimizations

#[cfg(test)]
use crate::*;

// ------------------------
// test validate_byterange

#[test]
pub fn test_validate_byterange_normal() {
    assert_eq!(validate_byterange(10, 5..7), 5..7);
    assert_eq!(validate_byterange(10, 5..10), 5..10);
    assert_eq!(validate_byterange(10, 5..=7), 5..8);
    assert_eq!(validate_byterange(10, 5..=5), 5..6);
    assert_eq!(validate_byterange(10, 5..=9), 5..10);
    assert_eq!(validate_byterange(10, 5..=4), 5..5); // Empty range
    assert_eq!(validate_byterange(10, 5..), 5..10);
    assert_eq!(validate_byterange(10, ..4), 0..4);
    assert_eq!(validate_byterange(10, ..=4), 0..5);
    assert_eq!(validate_byterange(10, ..), 0..10);
    assert_eq!(validate_byterange(10, 3..3), 3..3); // An empty range is accepted
    assert_eq!(validate_byterange(10, 10..10), 10..10); // An empty range at end position is accepted
    assert_eq!(validate_byterange(10, 0..0), 0..0); // An empty range is accepted ==> crash, return a "normal" range
}

#[test]
#[should_panic(expected = "Invalid range, start 3 is greater than end 2")]
pub fn test_validate_byterange_panic_invalid_range_1() {
    validate_byterange(10, 3..2);
}

#[test]
#[should_panic(expected = "Invalid range, start 12 is greater than byte count 10")]
pub fn test_validate_byterange_panic_invalid_range_2() {
    validate_byterange(10, 12..20);
}

#[test]
#[should_panic(expected = "Invalid range, end 11 is greater than byte count 10")]
pub fn test_validate_byterange_panic_invalid_range_3() {
    validate_byterange(10, 3..11);
}

// ------------------------
// get byte slice
// Test all range variants

#[test]
pub fn test_byteslice_from_byterange_normal() {
    assert_eq!(
        get_byteslice_from_byterange("Hello", 1..3),
        ['e' as u8, 'l' as u8]
    );
    assert_eq!(get_byteslice_from_byterange("Hello", 2..2), []);
    assert_eq!(
        get_byteslice_from_byterange("Hello", ..),
        ['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8]
    );
    assert_eq!(get_byteslice_from_byterange("", ..), []);
    assert_eq!(get_byteslice_from_byterange("Hello", 2..=2), ['l' as u8]);
    assert_eq!(
        get_byteslice_from_byterange("Hello", 2..),
        ['l' as u8, 'l' as u8, 'o' as u8]
    );
    assert_eq!(
        get_byteslice_from_byterange("Hello", ..2),
        ['H' as u8, 'e' as u8]
    );
    assert_eq!(
        get_byteslice_from_byterange("Hello", ..=2),
        ['H' as u8, 'e' as u8, 'l' as u8]
    );
}

#[test]
#[should_panic]
pub fn test_byteslice_from_byterange_panic_invalid_range() {
    let _ = get_byteslice_from_byterange("Hello", 3..1);
}

#[test]
#[should_panic]
pub fn test_byteslice_from_byterange_panic_invalid_range_start() {
    let _ = get_byteslice_from_byterange("Hello", 10..12);
}

#[test]
#[should_panic]
pub fn test_byteslice_from_byterange_panic_invalid_range_end() {
    let _ = get_byteslice_from_byterange("Hello", 2..12);
}

#[test]
pub fn test_bytesliceoption_from_byterange() {
    assert_eq!(
        get_bytesliceoption_from_byterange("Hello", 1..3),
        Some(&['e' as u8, 'l' as u8][..])
    );
    assert_eq!(
        get_bytesliceoption_from_byterange("Hello", 2..2),
        Some(&[][..])
    );
    assert_eq!(get_bytesliceoption_from_byterange("Hello", 3..1), None);
    assert_eq!(get_bytesliceoption_from_byterange("Hello", 10..12), None);
    assert_eq!(get_bytesliceoption_from_byterange("Hello", 2..12), None);
}

#[test]
pub fn test_bytesliceresult_from_byterange() {
    assert_eq!(
        get_bytesliceresult_from_byterange("Hello", 1..3),
        Ok(&['e' as u8, 'l' as u8][..])
    );
    assert_eq!(
        get_bytesliceresult_from_byterange("Hello", 2..2),
        Ok(&[][..])
    );
    assert!(get_bytesliceresult_from_byterange("Hello", 3..1).is_err());
    assert!(get_bytesliceresult_from_byterange("Hello", 10..12).is_err());
    assert!(get_bytesliceresult_from_byterange("Hello", 2..12).is_err());
}

#[test]
pub fn test_byteslicetolerant_from_byterange() {
    assert_eq!(
        get_byteslicetolerant_from_byterange("Hello", 1..3),
        ['e' as u8, 'l' as u8]
    );
    assert_eq!(get_byteslicetolerant_from_byterange("Hello", 2..2), []);
    assert_eq!(get_byteslicetolerant_from_byterange("Hello", 3..1), []);
    assert_eq!(get_byteslicetolerant_from_byterange("Hello", 10..12), []);
    assert_eq!(
        get_byteslicetolerant_from_byterange("Hello", 2..12),
        ['l' as u8, 'l' as u8, 'o' as u8]
    );
}

#[test]
pub fn test_byteslice_from_startbytecount() {
    assert_eq!(
        get_byteslice_from_startbytecount("Hello", 3),
        ['H' as u8, 'e' as u8, 'l' as u8]
    );
}

#[test]
pub fn test_byteslice_from_endbytecount() {
    assert_eq!(
        get_byteslice_from_endbytecount("Hello", 3),
        ['l' as u8, 'l' as u8, 'o' as u8]
    );
}

// ------------------------
// get_bytevector, copying bytes

// Returning a Vec<u8> is Ok, but it duplicates characters
#[test]
pub fn test_bytevector_from_byterange() {
    assert_eq!(
        get_bytevector_from_byterange("Hello", 2..4),
        vec!['l' as u8, 'l' as u8]
    );
    assert_eq!(
        get_bytevector_from_byterange("Hello", 2..=4),
        vec!['l' as u8, 'l' as u8, 'o' as u8]
    );
    assert_eq!(
        get_bytevector_from_byterange("Hello", 2..),
        vec!['l' as u8, 'l' as u8, 'o' as u8]
    );
    assert_eq!(
        get_bytevector_from_byterange("Hello", 0..3),
        vec!['H' as u8, 'e' as u8, 'l' as u8]
    );
    assert_eq!(
        get_bytevector_from_byterange("Hello", 0..=3),
        vec!['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8]
    );
    assert_eq!(
        get_bytevector_from_byterange("Hello", ..),
        vec!['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8]
    );
}

// ------------------------
// get char vector

#[test]
pub fn test_charvector_from_byterange() {
    assert_eq!(
        get_charvector_from_byterange("Aé♫山𝄞🐗", 3..9),
        vec!['♫', '山']
    );
    assert_eq!(
        get_charvector_from_byterange("Aé♫山𝄞🐗", 3..=8),
        vec!['♫', '山']
    );
    assert_eq!(
        get_charvector_from_byterange("Aé♫山𝄞🐗", ..3),
        vec!['A', 'é']
    );
    assert_eq!(
        get_charvector_from_byterange("Aé♫山𝄞🐗", ..=2),
        vec!['A', 'é']
    );
    assert_eq!(
        get_charvector_from_byterange("Aé♫山𝄞🐗", ..),
        vec!['A', 'é', '♫', '山', '𝄞', '🐗']
    );
    assert_eq!(get_charvector_from_byterange("Aé♫山𝄞🐗", 0..0), vec![]);
}

// ------------------------
// get glyph vector

#[test]
pub fn test_glyphvector_from_byterange() {
    assert_eq!(
        get_glyphvector_from_byterange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", 13..16),
        vec![Glyph2 {
            byte_range: 13..16,
            char_range: 4..6
        }]
    );

    assert_eq!(
        get_glyphvector_from_byterange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", ..),
        vec![
            Glyph2 {
                byte_range: 0..13,
                char_range: 0..4
            },
            Glyph2 {
                byte_range: 13..16,
                char_range: 4..6
            },
            Glyph2 {
                byte_range: 16..51,
                char_range: 6..16
            }
        ]
    );
}

#[test]
#[should_panic(
    expected = "Byte range start 10 is not a glyph boundary; it is inside glyph '🐻‍❄️' (bytes 0..13)"
)]
pub fn test_glyphvector_from_byterange_fail_start_not_at_glyph_boundary() {
    let _ = get_glyphvector_from_byterange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", 10..16);
}

#[test]
#[should_panic(
    expected = "Byte range end 15 is not a glyph boundary; it is inside glyph 'e\u{301}' (bytes 13..16)"
)]
pub fn test_glyphvector_from_byterange_fail_end_not_at_glyph_boundary() {
    let _ = get_glyphvector_from_byterange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", 13..15);
}

// ----------------------------------
// get byte iterator

#[test]
pub fn test_byteiterator_from_byterange() {
    let mut it = get_byteiterator_from_byterange("Hello", 2..5);
    assert_eq!(it.next(), Some('l' as u8));
    assert_eq!(it.next(), Some('l' as u8));
    assert_eq!(it.next(), Some('o' as u8));
    assert!(it.next().is_none());

    let mut it = get_byteiterator_from_byterange("Aé♫山𝄞🐗", 3..3);
    assert!(it.next().is_none());
}

// ----------------------------------
// get char iterator

#[test]
pub fn test_chariterator_from_byterange() {
    let s = "Aé♫山𝄞🐗";

    let mut it = get_chariterator_from_byterange(s, 3..=12);
    assert_eq!(it.next(), Some('♫'));
    assert_eq!(it.next(), Some('山'));
    assert_eq!(it.next(), Some('𝄞'));
    assert!(it.next().is_none());

    let mut it = get_byteiterator_from_byterange(s, 3..3);
    assert!(it.next().is_none());
}

// ----------------------------------
// get glyph iterator

#[test]
pub fn test_glyphiterator_from_byterange() {
    let mut it = get_glyphiterator_from_byterange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", 13..16);
    assert_eq!(
        it.next(),
        Some(Glyph2 {
            byte_range: 13..16,
            char_range: 4..6
        })
    );
    assert!(it.next().is_none());

    let mut it = get_glyphiterator_from_byterange("ABC", ..2);
    assert_eq!(
        it.next(),
        Some(Glyph2 {
            byte_range: 0..1,
            char_range: 0..1
        })
    );
    assert_eq!(
        it.next(),
        Some(Glyph2 {
            byte_range: 1..2,
            char_range: 1..2
        })
    );
    assert!(it.next().is_none());
}

// ------------------------
// get &str

#[test]
pub fn test_refstr_from_byterange() {
    let s = "Aé♫山𝄞🐗";

    assert_eq!(get_strref_from_byterange(s, 3..=12), "♫山𝄞");
    assert_eq!(get_strref_from_byterange(s, 3..3), "");
}

// ------------------------
// get String

#[test]
pub fn test_string_from_byterange() {
    let s = "Aé♫山𝄞🐗";

    assert_eq!(get_string_from_byterange(s, 3..=12), "♫山𝄞".to_string());
    assert!(String::is_empty(&get_string_from_byterange(s, 3..3)));
}
