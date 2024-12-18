// vstring unit tests - Test functions based on glyph range indexes
//
// 2024-12-18   PV      First version

#[cfg(test)]
pub mod glyphrange_tests {
    use crate::*;
    use glyph2::Glyph2;

    // ------------------------
    // test validate_glyphrange é

    #[test]
    pub fn test_validate_glyphrange_normal() {
        let s = "Aé♫山𝄞🐗🐻‍❄️"; // 10 chars, polar bear = 4 characters
                              //       UTF8        char byte
                              // Char  bytes       ix   ix
                              // A     41          0    0
                              // é     C3 A9       1    1
                              // ♫     E2 99 AB    2    3
                              // 山    E5 B1 B1    3    6
                              // 𝄞    F0 9D 84 9E  4    9
                              // 🐗   F0 9F 90 97  5    13
                              // 🐻   F0 9F 90 BB  6    17
                              // ZWJ   E2 80 8D     7    21
                              // ❄    E2 9D 84     8    24
                              // VS-16 EF B8 8F     9    27
                              //                   10    30

        assert_eq!(
            validate_glyphrange(s, 5..7),
            ByteCharGlyphRange {
                byte_range: 13..30,
                char_range: 5..10,
                glyph_range: 5..7
            }
        );
        assert_eq!(
            validate_glyphrange(s, 2..=3),
            ByteCharGlyphRange {
                byte_range: 3..9,
                char_range: 2..4,
                glyph_range: 2..4
            }
        );
        assert_eq!(
            validate_glyphrange(s, 4..),
            ByteCharGlyphRange {
                byte_range: 9..30,
                char_range: 4..10,
                glyph_range: 4..7
            }
        );
        assert_eq!(
            validate_glyphrange(s, ..),
            ByteCharGlyphRange {
                byte_range: 0..30,
                char_range: 0..10,
                glyph_range: 0..7
            }
        );
        assert_eq!(
            validate_glyphrange(s, 3..3),
            ByteCharGlyphRange {
                byte_range: 6..6,
                char_range: 3..3,
                glyph_range: 3..3
            }
        ); // An empty range is accepted
        assert_eq!(
            validate_glyphrange(s, 7..7),
            ByteCharGlyphRange {
                byte_range: 30..30,
                char_range: 10..10,
                glyph_range: 7..7
            }
        ); // An empty range at end position is accepted
        assert_eq!(
            validate_glyphrange(s, 0..0),
            ByteCharGlyphRange {
                byte_range: 0..0,
                char_range: 0..0,
                glyph_range: 0..0
            }
        ); // An empty range is accepted ==> crash, return a "normal" range
    }

    #[test]
    #[should_panic(expected = "Invalid glyph range, start 3 is greater than end 2")]
    pub fn test_validate_glyphrange_panic_invalid_range_1() {
        validate_glyphrange("HelloWorld", 3..2);
    }

    #[test]
    #[should_panic(expected = "Invalid glyph range, start 12 is greater than glyph count 7")]
    pub fn test_validate_glyphrange_panic_invalid_range_2() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";
        validate_glyphrange(s, 12..20);
    }

    #[test]
    #[should_panic(expected = "Invalid glyph range, end 11 is greater than glyph count 7")]
    pub fn test_validate_glyphrange_panic_invalid_range_3() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";
        validate_glyphrange(s, 3..11);
    }

    // ------------------------
    // get byte slice

    #[test]
    pub fn test_byteslice_from_glyphrange_normal() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";

        assert_eq!(get_byteslice_from_glyphrange(s, 1..3), [0xC3, 0xA9, 0xE2, 0x99, 0xAB]);
        assert_eq!(get_byteslice_from_glyphrange(s, 2..2), []);
        assert_eq!(
            get_byteslice_from_glyphrange(s, ..),
            [
                0x41, 0xC3, 0xA9, 0xE2, 0x99, 0xAB, 0xE5, 0xB1, 0xB1, 0xF0, 0x9D, 0x84, 0x9E, 0xF0, 0x9F, 0x90, 0x97, 0xF0, 0x9F, 0x90, 0xBB, 0xE2,
                0x80, 0x8D, 0xE2, 0x9D, 0x84, 0xEF, 0xB8, 0x8F,
            ]
        );
        assert_eq!(get_byteslice_from_glyphrange("", ..), []);
        assert_eq!(
            get_byteslice_from_glyphrange(s, 5..),
            [0xF0, 0x9F, 0x90, 0x97, 0xF0, 0x9F, 0x90, 0xBB, 0xE2, 0x80, 0x8D, 0xE2, 0x9D, 0x84, 0xEF, 0xB8, 0x8F,]
        );
        assert_eq!(get_byteslice_from_glyphrange(s, ..2), [0x41, 0xC3, 0xA9]);
        assert_eq!(get_byteslice_from_glyphrange(s, ..=2), [0x41, 0xC3, 0xA9, 0xE2, 0x99, 0xAB]);
    }

    #[test]
    pub fn test_byteslice_from_startglyphcount() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";
        assert_eq!(
            get_byteslice_from_startglyphcount(s, 4),
            [0x41, 0xC3, 0xA9, 0xE2, 0x99, 0xAB, 0xE5, 0xB1, 0xB1]
        );
    }

    #[test]
    pub fn test_byteslice_from_endglyphcount() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";
        assert_eq!(
            get_byteslice_from_endglyphcount(s, 3),
            [0xF0, 0x9D, 0x84, 0x9E, 0xF0, 0x9F, 0x90, 0x97, 0xF0, 0x9F, 0x90, 0xBB, 0xE2, 0x80, 0x8D, 0xE2, 0x9D, 0x84, 0xEF, 0xB8, 0x8F]
        );
    }

    // ------------------------
    // get byte vector

    #[test]
    pub fn test_bytevector_from_glyphrange() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";
        assert_eq!(get_bytevector_from_glyphrange(s, 1..3), vec![0xC3, 0xA9, 0xE2, 0x99, 0xAB]);
        assert_eq!(get_bytevector_from_glyphrange(s, 2..2), vec![]);
        assert_eq!(
            get_bytevector_from_glyphrange(s, ..),
            vec![
                0x41, 0xC3, 0xA9, 0xE2, 0x99, 0xAB, 0xE5, 0xB1, 0xB1, 0xF0, 0x9D, 0x84, 0x9E, 0xF0, 0x9F, 0x90, 0x97, 0xF0, 0x9F, 0x90, 0xBB, 0xE2,
                0x80, 0x8D, 0xE2, 0x9D, 0x84, 0xEF, 0xB8, 0x8F,
            ]
        );
        assert_eq!(get_bytevector_from_glyphrange("", ..), vec![]);
        assert_eq!(
            get_bytevector_from_glyphrange(s, 5..),
            vec![0xF0, 0x9F, 0x90, 0x97, 0xF0, 0x9F, 0x90, 0xBB, 0xE2, 0x80, 0x8D, 0xE2, 0x9D, 0x84, 0xEF, 0xB8, 0x8F,]
        );
        assert_eq!(get_bytevector_from_glyphrange(s, ..2), vec![0x41, 0xC3, 0xA9]);
        assert_eq!(get_bytevector_from_glyphrange(s, ..=2), vec![0x41, 0xC3, 0xA9, 0xE2, 0x99, 0xAB]);
    }

    // ------------------------
    // get char vector

    #[test]
    pub fn test_charvector_from_glyphrange() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";
        assert_eq!(get_charvector_from_glyphrange(s, 1..3), vec!['é', '♫']);
        assert_eq!(get_charvector_from_glyphrange(s, 2..2), vec![]);
        assert_eq!(
            get_charvector_from_glyphrange(s, ..),
            vec!['A', 'é', '♫', '山', '𝄞', '🐗', '🐻', '\u{200D}', '❄', '\u{FE0F}']
        );
        assert_eq!(get_charvector_from_glyphrange("", ..), vec![]);
        assert_eq!(get_charvector_from_glyphrange(s, 5..), vec!['🐗', '🐻', '\u{200D}', '❄', '\u{FE0F}']);
        assert_eq!(get_charvector_from_glyphrange(s, ..2), vec!['A', 'é']);
        assert_eq!(get_charvector_from_glyphrange(s, ..=2), vec!['A', 'é', '♫']);
    }

    // ------------------------
    // get glyph vector

    #[test]
    pub fn test_glyphvector_from_glyphrange() {
        assert_eq!(
            get_glyphvector_from_glyphrange("e\u{0301}", ..1),
            vec![Glyph2 {
                byte_range: 0..3,
                char_range: 0..2
            }]
        );

        assert_eq!(
            get_glyphvector_from_glyphrange("👨‍❤‍👩e\u{0301}🐻‍❄️", 1..3),
            vec![
                Glyph2 {
                    byte_range: 17..20,
                    char_range: 5..7
                },
                Glyph2 {
                    byte_range: 20..33,
                    char_range: 7..11
                }
            ]
        );

        assert_eq!(
            get_glyphvector_from_glyphrange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", ..),
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
    #[should_panic(expected = "Invalid glyph range, start 6 is greater than glyph count 3")]
    pub fn test_glyphvector_from_glyphrange_panic_start_not_at_glyph_blundary() {
        let _ = get_glyphvector_from_glyphrange("👨‍❤‍👩e\u{0301}🐻‍❄️", 6..7);
    }

    #[test]
    #[should_panic(expected = "Invalid glyph range, end 9 is greater than glyph count 3")]
    pub fn test_glyphvector_from_glyphrange_panic_end_not_at_glyph_blundary() {
        let _ = get_glyphvector_from_glyphrange("👨‍❤‍👩e\u{0301}🐻‍❄️", ..9);
    }

    // ----------------------------------
    // get byte iterator

    #[test]
    pub fn test_byteiterator_from_glyphrange() {
        let s = "👨‍❤‍👩e\u{0301}🐻‍❄️";

        let mut it = get_byteiterator_from_glyphrange(s, 2..);
        assert_eq!(it.next(), Some(0xF0));
        assert_eq!(it.next(), Some(0x9F));
        assert_eq!(it.next(), Some(0x90));
        assert_eq!(it.next(), Some(0xBB));
        assert_eq!(it.next(), Some(0xE2));
        assert_eq!(it.next(), Some(0x80));
        assert_eq!(it.next(), Some(0x8D));
        assert_eq!(it.next(), Some(0xE2));
        assert_eq!(it.next(), Some(0x9D));
        assert_eq!(it.next(), Some(0x84));
        assert_eq!(it.next(), Some(0xEF));
        assert_eq!(it.next(), Some(0xB8));
        assert_eq!(it.next(), Some(0x8F));
        assert!(it.next().is_none());

        let mut it = get_byteiterator_from_glyphrange(s, 2..2);
        assert!(it.next().is_none());
    }

    // ----------------------------------
    // get char iterator

    #[test]
    pub fn test_chariterator_from_glyphrange() {
        let s = "👨‍❤‍👩e\u{0301}🐻‍❄️";

        let mut it = get_chariterator_from_glyphrange(s, 2..);
        assert_eq!(it.next(), Some('🐻'));
        assert_eq!(it.next(), Some('\u{200D}'));    // ZWJ
        assert_eq!(it.next(), Some('❄'));
        assert_eq!(it.next(), Some('\u{FE0F}'));    // VS-16
        assert!(it.next().is_none());

        let mut it = get_byteiterator_from_glyphrange(s, 3..3);
        assert!(it.next().is_none());
    }

    // ----------------------------------
    // get glyph iterator

    #[test]
    pub fn test_glyphiterator_from_glyphrange() {
        let mut it = get_glyphiterator_from_glyphrange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", 1..2);
        assert_eq!(
            it.next(),
            Some(Glyph2 {
                byte_range: 13..16,
                char_range: 4..6
            })
        );
        assert!(it.next().is_none());

        let mut it = get_glyphiterator_from_glyphrange("ABC", ..2);
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
    pub fn test_refstr_from_glyphrange() {
        let s="🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻";

        assert_eq!(get_strref_from_glyphrange(s, 1..=2), "e\u{0301}👨🏾‍❤️‍💋‍👨🏻");
        assert_eq!(get_strref_from_glyphrange(s, 3..3), "");
    }

    // ------------------------
    // get String

    #[test]
    pub fn test_string_from_glyphrange() {
        let s="🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻";

        assert_eq!(get_string_from_glyphrange(s, 1..3), "e\u{0301}👨🏾‍❤️‍💋‍👨🏻".to_string());
        assert!(String::is_empty(&get_string_from_glyphrange(s, 3..3)));
    }

}
