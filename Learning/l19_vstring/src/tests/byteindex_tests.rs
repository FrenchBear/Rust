// vstring unit tests - Functions based on byte index tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
pub mod byteindex_tests {
    use glyph2::Glyph2;

    use crate::*;

    // ------------------------
    // get_byte

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

    // ------------------------
    // get_char
    
    #[test]
    fn get_char_from_byteindex_standard() {
        let s = "Aé♫山𝄞🐗";
        assert_eq!(get_char_from_byteindex(s, 0), 65 as char);
        assert_eq!(get_char_from_byteindex(s, s.len() - 4), '\u{1F417}'); // U+1F417 BOAR = UTF8: F0 9F 90 97
        assert_eq!(get_char_from_byteindex("🐻‍❄️", 7), '❄'); // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
    }

    #[should_panic(expected = "not a char boundary")]
    #[test]
    fn get_char_from_byteindex_panic_not_a_char_boundary() {
        let _ = get_char_from_byteindex("🐗", 1); // U+1F417 BOAR = UTF8: F0 9F 90 97
    }

    #[should_panic(expected = "out of bounds")]
    #[test]
    fn get_char_from_byteindex_panic_out_of_bounds() {
        let _ = get_char_from_byteindex("abc", 5);
    }

    #[test]
    fn get_charoption_from_byteindex_tests() {
        assert_eq!(get_charoption_from_byteindex("Aé♫山𝄞🐗", 0), Some('A'));
        assert_eq!(get_charoption_from_byteindex("abc", 5), None);
        assert_eq!(get_charoption_from_byteindex("🐗", 1), None); // U+1F417 BOAR = UTF8: F0 9F 90 97
    }

    // ------------------------
    // get_glyph

    #[test]
    fn get_glyph_from_byteindex_standard() {
        let s = "Ae\u{0301}𝄞â̧̅🐗🐻‍❄️👨🏾‍❤️‍💋‍👨🏻";

        assert_eq!(get_glyph_from_byteindex(s, 0), Glyph2 { byte_range: (0usize..=0), char_range:    (0usize..=0usize)});   // A
        assert_eq!(get_glyph_from_byteindex(s, 1), Glyph2 { byte_range: (1usize..=3), char_range:    (1usize..=2usize)});   // é
        assert_eq!(get_glyph_from_byteindex(s, 4), Glyph2 { byte_range: (4usize..=7), char_range:    (3usize..=3usize)});   // 𝄞
        assert_eq!(get_glyph_from_byteindex(s, 8), Glyph2 { byte_range: (8usize..=14), char_range:   (4usize..=7usize)});   // â̧̅
        assert_eq!(get_glyph_from_byteindex(s, 15), Glyph2 { byte_range: (15usize..=18), char_range:  (8usize..=8usize)});   //🐗
        assert_eq!(get_glyph_from_byteindex(s, 19), Glyph2 { byte_range: (19usize..=31), char_range:  (9usize..=12usize)});  //🐻‍❄️
        assert_eq!(get_glyph_from_byteindex(s, 32), Glyph2 { byte_range: (32usize..=66), char_range:  (13usize..=22usize)}); //👨🏾‍❤️‍💋‍👨🏻
    }

    #[should_panic(expected = "out of bounds")]
    #[test]
    fn get_glyph_from_byteindex_panic_out_of_bounds() {
        let _ = get_glyph_from_byteindex("abc", 5);
    }

    #[should_panic(expected = "not a glyph boundary")]
    #[test]
    fn get_glyph_from_byteindex_panic_not_a_glyph_boundary() {
        let _ = get_glyph_from_byteindex("🐗", 1);
    }


    #[test]
    fn get_glyphoption_from_byteindex_standard() {
        assert_eq!(get_glyphoption_from_byteindex("ABC", 1), Some(Glyph2 { byte_range: (1usize..=1), char_range: (1usize..=1usize) }));
        assert_eq!(get_glyphoption_from_byteindex("ABC", 5), None);
        assert_eq!(get_glyphoption_from_byteindex("🐗", 1), None);
    }
}
