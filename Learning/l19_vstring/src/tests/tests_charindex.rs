// vstring unit tests - Functions based on char index tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
pub mod charindex_tests {
    use crate::{glyph2::Glyph2, vstring::*};

    // ------------------------
    // test validate_charindex

    #[test]
    fn test_validate_charindex() {
        validate_byterange(2, 1..2);
        assert_eq!(validate_charindex("ABC", 1), (1..2, 'B'));
        assert_eq!(validate_charindex("Aé♫山𝄞🐗", 4), (9..13, '𝄞'));
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

    #[should_panic(expected = "char index out of bounds: s contains 3 characters, but the index is 5")]
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
        assert_eq!(get_byteslice_from_charindex("Où ça?", 3), [0xC3u8, 0xA7u8]);
    }

    // ------------------------
    // test byte vector

    #[test]
    pub fn test_bytevector_from_charindex() {
        assert_eq!(get_bytevector_from_charindex("Où ça?", 3), vec![0xC3u8, 0xA7u8]);
    }


}
