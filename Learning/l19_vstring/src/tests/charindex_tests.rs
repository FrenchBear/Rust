// vstring unit tests - Functions based on char index tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
pub mod charindex_tests {
    use crate::*;

    #[test]
    fn get_char_from_charindex_tests1() {
        let s = "Aé♫山𝄞🐗";
        assert_eq!(get_char_from_charindex(s, 0), 65 as char);
        assert_eq!(get_char_from_charindex(s, get_char_length(s)-1), '\u{1F417}');     // U+1F417 BOAR = UTF8: F0 9F 90 97
        assert_eq!(get_char_from_charindex("🐻‍❄️", 2), '❄');        // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
    }

    #[should_panic]
    #[test]
    fn get_char_from_charindex_tests2() {
        let _ = get_char_from_charindex("abc", 5);
    }

    #[test]
    fn get_charoption_from_charindex_tests() {
        assert_eq!(get_charoption_from_charindex("Aé♫山𝄞🐗", 0), Some('A'));
        assert_eq!(get_charoption_from_charindex("abc", 5), None);     // U+1F417 BOAR = UTF8: F0 9F 90 97
    }    
}
