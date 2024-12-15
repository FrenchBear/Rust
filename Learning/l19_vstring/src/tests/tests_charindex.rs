// vstring unit tests - Functions based on char index tests
//
// 2024-12-13   PV      First version

#[cfg(test)]
pub mod charindex_tests {
    use crate::*;

    // ------------------------
    // test validate_charindex

    #[test]
    fn test_validate_charindex() {
        assert_eq!(validate_charindex("ABC", 1), (1..2, 'B'));
        assert_eq!(validate_charindex("Aé♫山𝄞🐗", 4), (9..13, '𝄞'));
    }

    // ------------------------
    // test get char

    #[test]
    fn test_char_from_charindex_normal() {
        let s = "Aé♫山𝄞🐗";
        assert_eq!(get_char_from_charindex(s, 0), 'A');
        assert_eq!(get_char_from_charindex(s, get_char_length(s)-1), '\u{1F417}');     // U+1F417 BOAR = UTF8: F0 9F 90 97
        assert_eq!(get_char_from_charindex("🐻‍❄️", 2), '❄');        // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
    }

    #[should_panic]
    #[test]
    fn test_char_from_charindex_panic_out_of_bounds() {
        let _ = get_char_from_charindex("abc", 5);
    }

    // ------------------------
    // test get char option

    #[test]
    fn test_charoption_from_charindex() {
        assert_eq!(get_charoption_from_charindex("Aé♫山𝄞🐗", 1), Some('é'));
        assert_eq!(get_charoption_from_charindex("abc", 5), None);
    }    

}
