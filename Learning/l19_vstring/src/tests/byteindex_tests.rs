// vstring unit tests - Functions based on byte index tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
pub mod byteindex_tests {
    use glyph::Glyph;

    use crate::*;

    #[test]
    fn get_byte_from_byteindex_tests1() {
        let s = "Aé♫山𝄞🐗";
        assert_eq!(get_byte_from_byteindex(s, 0), 65);
        assert_eq!(get_byte_from_byteindex(s, get_byte_length(s) - 1), 0x97); // U+1F417 BOAR = UTF8: F0 9F 90 97
    }

    #[should_panic]
    #[test]
    fn get_byte_from_byteindex_tests2() {
        let _ = get_byte_from_byteindex("abc", 5);
    }

    #[test]
    fn get_byteoption_from_byteindex_tests() {
        assert_eq!(get_byteoption_from_byteindex("Aé♫山𝄞🐗", 0), Some(65));
        assert_eq!(get_byteoption_from_byteindex("abc", 5), None); // U+1F417 BOAR = UTF8: F0 9F 90 97
    }

    #[test]
    fn get_char_from_byteindex_tests1() {
        let s = "Aé♫山𝄞🐗";
        assert_eq!(get_char_from_byteindex(s, 0), 65 as char);
        assert_eq!(get_char_from_byteindex(s, s.len() - 4), '\u{1F417}'); // U+1F417 BOAR = UTF8: F0 9F 90 97
        assert_eq!(get_char_from_byteindex("🐻‍❄️", 7), '❄'); // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
    }

    #[should_panic]
    #[test]
    fn get_char_from_byteindex_tests2() {
        let _ = get_char_from_byteindex("abc", 5);
    }

    #[test]
    fn get_charoption_from_byteindex_tests() {
        assert_eq!(get_charoption_from_byteindex("Aé♫山𝄞🐗", 0), Some('A'));
        assert_eq!(get_charoption_from_byteindex("abc", 5), None); // U+1F417 BOAR = UTF8: F0 9F 90 97
    }

    #[test]
    fn get_glyph_from_byteindex_tests1() {
        let s = "Aé♫山𝄞🐗";
        assert_eq!(get_glyph_from_byteindex(s, 0), Glyph { chars: "A".to_string() });
        assert_eq!(
            get_glyph_from_byteindex(s, s.len() - 4),
            Glyph {
                chars: "\u{1F417}".to_string()
            }
        ); // U+1F417 BOAR = UTF8: F0 9F 90 97
           // For now, Emoji are not supported
        assert_eq!(get_glyph_from_byteindex("🐻‍❄️", 0), Glyph { chars: "🐻".to_string() });
        // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
    }

    #[should_panic]
    #[test]
    fn get_glyph_from_byteindex_tests2() {
        let _ = get_glyph_from_byteindex("abc", 5);
    }

    #[test]
    fn get_glyphoption_from_byteindex_tests1() {
        assert_eq!(get_glyphoption_from_byteindex("ABC", 1), Some(Glyph { chars: "B".to_string() }));
        assert_eq!(get_glyphoption_from_byteindex("ABC", 5), None);
    }
}
