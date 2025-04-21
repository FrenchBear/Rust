// unit tests for vstring
// Learning rust
//
// 2024-12-13   PV      First version

mod tests_byteindex;
mod tests_byterange;
mod tests_charindex;
mod tests_charrange;
mod tests_glyph2;
mod tests_glyphindex;
mod tests_glyphrange;

#[cfg(test)]
use super::*;

#[test]
fn test_byte_length() {
    let s = "Aé♫山𝄞🐗";
    assert_eq!(get_byte_length(s), 17);
    assert_eq!(get_byte_length(""), 0);
    assert_eq!(get_byte_length("e\u{0301}"), 3); // e + U+0301 COMBINING ACUTE ACCENT
    assert_eq!(get_byte_length("🐻‍❄️"), 13); // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
}

#[test]
fn test_char_length() {
    assert_eq!(get_char_length("Aé♫山𝄞🐗"), 6);
    assert_eq!(get_char_length(""), 0);
    assert_eq!(get_char_length("e\u{0301}"), 2);
    assert_eq!(get_char_length("🐻‍❄️"), 4); // U+1F43B BEAR FACE, U+200D ZWJ, U+2744 SNOWFLAKE, U+FE0F VS-16
}

#[test]
fn test_glyph_length() {
    assert_eq!(get_glyph_length("Aé♫山𝄞🐗"), 6);
    assert_eq!(get_glyph_length(""), 0);
    assert_eq!(get_glyph_length("e\u{0301}"), 1);
    assert_eq!(get_glyph_length("🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"), 2);
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

// ------------------------
// test conversion to str&

#[test]
fn test_strref_from_byteslice() {
    for s in vec!["", "Aé♫山𝄞🐗", "e\u{0301}🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"]
    {
        assert_eq!(
            s,
            get_strref_from_byteslice(get_byteslice_from_byterange(s, ..))
        );
    }
}

#[test]
fn test_strref_from_bytevector() {
    for s in vec!["", "Aé♫山𝄞🐗", "e\u{0301}🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"]
    {
        assert_eq!(
            s,
            get_strref_from_bytevector(&get_bytevector_from_byterange(s, ..))
        );
        assert_eq!(
            s,
            get_strref_from_bytevector(&get_bytevector_from_charrange(s, ..))
        );
        assert_eq!(
            s,
            get_strref_from_bytevector(&get_bytevector_from_glyphrange(s, ..))
        );
    }
}

// ------------------------
// test conversion to string

#[test]
fn test_string_from_byteslice() {
    for s in vec!["", "Aé♫山𝄞🐗", "e\u{0301}🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"]
    {
        assert_eq!(
            s.to_string(),
            get_string_from_byteslice(get_byteslice_from_byterange(s, ..))
        );
    }
}

#[test]
fn test_string_from_bytevector() {
    for s in vec!["", "Aé♫山𝄞🐗", "e\u{0301}🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"]
    {
        assert_eq!(
            s.to_string(),
            get_string_from_bytevector(get_bytevector_from_byterange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_bytevector(get_bytevector_from_charrange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_bytevector(get_bytevector_from_glyphrange(s, ..))
        );
    }
}

#[test]
fn test_string_from_bytevectorref() {
    for s in vec!["", "Aé♫山𝄞🐗", "e\u{0301}🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"]
    {
        assert_eq!(
            s.to_string(),
            get_string_from_bytevectorref(&get_bytevector_from_byterange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_bytevectorref(&get_bytevector_from_charrange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_bytevectorref(&get_bytevector_from_glyphrange(s, ..))
        );
    }
}

#[test]
fn test_string_from_byteiterator() {
    for s in vec!["", "Aé♫山𝄞🐗", "e\u{0301}🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"]
    {
        assert_eq!(
            s.to_string(),
            get_string_from_byteiterator(get_byteiterator_from_byterange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_byteiterator(get_byteiterator_from_charrange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_byteiterator(get_byteiterator_from_glyphrange(s, ..))
        );
    }
}

// ------------------------

#[test]
fn test_string_from_charslice() {
    assert_eq!(
        get_string_from_charslice(&['A', 'é', '♫', '山', '𝄞', '🐗']),
        "Aé♫山𝄞🐗".to_string()
    );
}

#[test]
fn test_string_from_charvector() {
    for s in vec!["", "Aé♫山𝄞🐗", "e\u{0301}🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"]
    {
        assert_eq!(
            s.to_string(),
            get_string_from_charvector(get_charvector_from_byterange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_charvector(get_charvector_from_charrange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_charvector(get_charvector_from_glyphrange(s, ..))
        );
    }
}

#[test]
fn test_string_from_charvectorref() {
    for s in vec!["", "Aé♫山𝄞🐗", "e\u{0301}🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"]
    {
        assert_eq!(
            s.to_string(),
            get_string_from_charvectorref(&get_charvector_from_byterange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_charvectorref(&get_charvector_from_charrange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_charvectorref(&get_charvector_from_glyphrange(s, ..))
        );
    }
}

#[test]
fn test_string_from_chariterator() {
    for s in vec!["", "Aé♫山𝄞🐗", "e\u{0301}🐻‍❄️👨🏾‍❤️‍💋‍👨🏻"]
    {
        assert_eq!(
            s.to_string(),
            get_string_from_chariterator(get_chariterator_from_charrange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_chariterator(get_chariterator_from_charrange(s, ..))
        );
        assert_eq!(
            s.to_string(),
            get_string_from_chariterator(get_chariterator_from_glyphrange(s, ..))
        );
    }
}
