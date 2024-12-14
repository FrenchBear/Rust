// unit tests for vstring
// Learning rust
//
// 2024-12-13   PV      First version

mod glyph2_tests;
mod byteindex_tests;
mod byterange_tests;
mod charindex_tests;

#[cfg(test)]
use super::*;
pub mod length_tests {
    #[test]
    fn get_byte_length_tests() {
        let s = "Aé♫山𝄞🐗";
        assert_eq!(super::get_byte_length(s), 17);
        assert_eq!(super::get_byte_length(""), 0);
        assert_eq!(super::get_byte_length("e\u{0301}"), 3);  // e + U+0301 COMBINING ACUTE ACCENT
        assert_eq!(super::get_byte_length("🐻‍❄️"), 13);       // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
    }

    #[test]
    fn get_char_length_tests() {
        assert_eq!(super::get_char_length("Aé♫山𝄞🐗"), 6);
        assert_eq!(super::get_char_length(""), 0);
        assert_eq!(super::get_char_length("e\u{0301}"), 2);
        assert_eq!(super::get_char_length("🐻‍❄️"), 4);        // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
    }

    #[test]
    fn get_glyph_length_tests() {
        assert_eq!(super::get_glyph_length("Aé♫山𝄞🐗"), 6);
        assert_eq!(super::get_glyph_length(""), 0);
        assert_eq!(super::get_glyph_length("e\u{0301}"), 1);
        assert_eq!(super::get_glyph_length("🐻‍❄️"), 1);        // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
    }
}


