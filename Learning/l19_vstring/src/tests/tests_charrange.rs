// vstring unit tests - Test functions based on char range indexes
//
// 2024-12-16   PV      First version

#[cfg(test)]
pub mod charrange_tests {
    use crate::*;
    use glyph2::Glyph2;

    // ------------------------
    // test validate_charrange

    #[test]
    pub fn test_validate_charrange_normal() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";  // 10 chars, polar bear = 4 characters
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
        

        assert_eq!(validate_charrange(s, 5..7), 13..21);
        assert_eq!(validate_charrange(s, 5..10), 13..30);
        assert_eq!(validate_charrange(s, 5..=7), 13..24);
        assert_eq!(validate_charrange(s, 5..=5), 13..17);
        assert_eq!(validate_charrange(s, 5..=9), 13..30);
        assert_eq!(validate_charrange(s, 5..), 13..30);
        assert_eq!(validate_charrange(s, ..4), 0..9);
        assert_eq!(validate_charrange(s, ..=4), 0..13);
        assert_eq!(validate_charrange(s, ..), 0..30);
        assert_eq!(validate_charrange(s, 3..3), 6..6); // An empty range is accepted
        assert_eq!(validate_charrange(s, 10..10), 30..30); // An empty range at end position is accepted
        assert_eq!(validate_charrange(s, 0..0), 0..0); // An empty range is accepted ==> crash, return a "normal" range
    }

    #[test]
    #[should_panic(expected = "Invalid range, start 3 is greater than end 2")]
    pub fn test_validate_charrange_panic_invalid_range_1() {
        validate_charrange("HelloWorld", 3..2);
    }


    #[test]
    #[should_panic(expected = "Invalid range, start 12 is greater than chars count 10")]
    pub fn test_validate_charrange_panic_invalid_range_2() {
        validate_charrange("HelloWorld", 12..20);
    }

    #[test]
    #[should_panic(expected = "Invalid range, end 11 is greater than chars count 10")]
    pub fn test_validate_charrange_panic_invalid_range_3() {
        validate_charrange("HelloWorld", 3..11);
    }

    // ------------------------
    // get byte slice

    #[test]
    pub fn test_byteslice_from_charrange_normal() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";
    
        assert_eq!(get_byteslice_from_charrange(s, 1..3), [0xC3, 0xA9, 0xE2, 0x99, 0xAB]);
        assert_eq!(get_byteslice_from_charrange(s, 2..2), []);
        assert_eq!(
            get_byteslice_from_charrange(s, ..),
            [0x41, 0xC3, 0xA9, 0xE2, 0x99, 0xAB, 0xE5, 0xB1, 0xB1, 0xF0, 0x9D, 0x84, 0x9E, 0xF0, 0x9F, 0x90, 0x97, 0xF0, 0x9F, 0x90, 0xBB, 0xE2, 0x80, 0x8D, 0xE2, 0x9D, 0x84, 0xEF, 0xB8, 0x8F,]
        );
        assert_eq!(get_byteslice_from_charrange("", ..), []);
        assert_eq!(get_byteslice_from_charrange(s, 5..), [0xF0, 0x9F, 0x90, 0x97, 0xF0, 0x9F, 0x90, 0xBB, 0xE2, 0x80, 0x8D, 0xE2, 0x9D, 0x84, 0xEF, 0xB8, 0x8F,]);
        assert_eq!(get_byteslice_from_charrange(s, ..2), [0x41, 0xC3, 0xA9]);
        assert_eq!(get_byteslice_from_charrange(s, ..=2), [0x41, 0xC3, 0xA9, 0xE2, 0x99, 0xAB]);
    }

    #[test]
    pub fn test_byteslice_from_startcharcount() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";
        assert_eq!(get_byteslice_from_startcharcount(s, 4), [0x41, 0xC3, 0xA9, 0xE2, 0x99, 0xAB, 0xE5, 0xB1, 0xB1]);
    }

    #[test]
    pub fn test_charslice_from_endcharcount() {
        let s = "Aé♫山𝄞🐗🐻‍❄️";
        assert_eq!(get_byteslice_from_endcharcount(s, 3), [0xE2, 0x80, 0x8D, 0xE2, 0x9D, 0x84, 0xEF, 0xB8, 0x8F]);
    }

    // ------------------------
    // get_bytevector, copying bytes

    /*
    // Returning a Vec<u8> is Ok, but it duplicates characters
    #[test]
    pub fn test_bytevector_from_charrange() {
        assert_eq!(get_bytevector_from_charrange("Hello", 2..4), vec!['l' as u8, 'l' as u8]);
        assert_eq!(get_bytevector_from_charrange("Hello", 2..=4), vec!['l' as u8, 'l' as u8, 'o' as u8]);
        assert_eq!(get_bytevector_from_charrange("Hello", 2..), vec!['l' as u8, 'l' as u8, 'o' as u8]);
        assert_eq!(get_bytevector_from_charrange("Hello", 0..3), vec!['H' as u8, 'e' as u8, 'l' as u8]);
        assert_eq!(
            get_bytevector_from_charrange("Hello", 0..=3),
            vec!['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8]
        );
        assert_eq!(
            get_bytevector_from_charrange("Hello", ..),
            vec!['H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8]
        );
    }

    // ------------------------
    // get char vector

    #[test]
    pub fn test_charvector_from_charrange() {
        assert_eq!(get_charvector_from_charrange("Aé♫山𝄞🐗", 3..9), vec!['♫', '山']);
        assert_eq!(get_charvector_from_charrange("Aé♫山𝄞🐗", 3..=8), vec!['♫', '山']);
        assert_eq!(get_charvector_from_charrange("Aé♫山𝄞🐗", ..3), vec!['A', 'é']);
        assert_eq!(get_charvector_from_charrange("Aé♫山𝄞🐗", ..=2), vec!['A', 'é']);
        assert_eq!(get_charvector_from_charrange("Aé♫山𝄞🐗", ..), vec!['A', 'é', '♫', '山', '𝄞', '🐗']);
        assert_eq!(get_charvector_from_charrange("Aé♫山𝄞🐗", 0..0), vec![]);
    }

    // ------------------------
    // get glyph vector

    #[test]
    pub fn test_glyphvector_from_charrange() {
        assert_eq!(
            get_glyphvector_from_charrange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", 13..16),
            vec![Glyph2 {
                byte_range: 13..16,
                char_range: 4..6
            }]
        );

        assert_eq!(
            get_glyphvector_from_charrange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", ..),
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

    // ----------------------------------
    // get byte iterator

    #[test]
    pub fn test_byteiterator_from_charrange() {
        let mut it = get_byteiterator_from_charrange("Hello", 2..5);
        assert_eq!(it.next(), Some('l' as u8));
        assert_eq!(it.next(), Some('l' as u8));
        assert_eq!(it.next(), Some('o' as u8));
        assert!(it.next().is_none());

        let mut it = get_byteiterator_from_charrange("Aé♫山𝄞🐗", 3..3);
        assert!(it.next().is_none());
    }

    // ----------------------------------
    // get char iterator

    #[test]
    pub fn test_chariterator_from_charrange() {
        let mut it = get_chariterator_from_charrange("Aé♫山𝄞🐗", 3..=12);
        assert_eq!(it.next(), Some('♫'));
        assert_eq!(it.next(), Some('山'));
        assert_eq!(it.next(), Some('𝄞'));
        assert!(it.next().is_none());

        let mut it = get_byteiterator_from_charrange("Aé♫山𝄞🐗", 3..3);
        assert!(it.next().is_none());
    }

    // ----------------------------------
    // get glyph iterator

    #[test]
    pub fn test_glyphiterator_from_charrange() {
        let mut it = get_glyphiterator_from_charrange("🐻‍❄️e\u{0301}👨🏾‍❤️‍💋‍👨🏻", 13..16);
        assert_eq!(
            it.next(),
            Some(Glyph2 {
                byte_range: 13..16,
                char_range: 4..6
            })
        );
        assert!(it.next().is_none());

        let mut it = get_glyphiterator_from_charrange("ABC", ..2);
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
    pub fn test_refstr_from_charrange() {
        assert_eq!(get_strref_from_charrange("Aé♫山𝄞🐗", 3..=12), "♫山𝄞");
        assert_eq!(get_strref_from_charrange("Aé♫山𝄞🐗", 3..3), "");
    }

    // ------------------------
    // get String

    #[test]
    pub fn test_string_from_charrange() {
        assert_eq!(get_string_from_charrange("Aé♫山𝄞🐗", 3..=12), "♫山𝄞".to_string());
        assert!(String::is_empty(&get_string_from_charrange("Aé♫山𝄞🐗", 3..3)));
    }
    */
}
