// unit tests for vstring
// Learning rust
//
// 2024-12-13   PV      First version

mod tests_glyph2;
mod tests_byteindex;
mod tests_byterange;
mod tests_charindex;
mod tests_charrange;


/*
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
        assert_eq!(super::get_glyph_length("🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"), 2);        
            // Glyph #1:
            //  U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
            // Glyph #2:
            //  U+1F468	MAN
            //  U+1F3FE	EMOJI MODIFIER FITZPATRICK TYPE-5
            //  U+200D	ZERO WIDTH JOINER
            //  U+2764	HEAVY BLACK HEART
            //  U+FE0F	VARIATION SELECTOR-16
            //  U+200D	ZERO WIDTH JOINER
            //  U+1F48B	KISS MARK
            //  U+200D	ZERO WIDTH JOINER
            //  U+1F468	MAN
            //  U+1F3FB	EMOJI MODIFIER FITZPATRICK TYPE-1-2
    }
}
 */