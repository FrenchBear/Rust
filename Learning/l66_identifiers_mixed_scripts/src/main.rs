// l66_identifiers_mixed_scripts
//
// Split a string into default identifiers (Unicode UAX #31) and check for compliance with moderately restrictive (UTS #39,
// https://www.unicode.org/reports/tr39/#moderately_restrictive).
// Finally don't use compliance test provided by unicode_security crate, since it doesn't match my needs: purely arabic or 
// runic identifiers are considered non-compliant at any level, so I use my owe version of is_single_script
//
// 2025-10-24   PV      First version

// Reference:
// https://www.unicode.org/reports/tr31/
// https://www.unicode.org/reports/tr39

// Related documentation:
// https://www.unicode.org/L2/L2022/22231r-mixed-script-detection.pdf


#![allow(unused)]

//use unicode_security::RestrictionLevelDetection;
use unicode_ident::{is_xid_start, is_xid_continue};
use unicode_script::{Script, UnicodeScript};
use std::collections::HashSet;

fn main() {
    test("Circle", "all latin");
    test("СігсӀе", "all cyrillic");
    test("Сirсlе", "mixed latin cyrillic");
    test("Circ1e", "latin and a digit");
    test("C𝗂𝗋𝖼𝗅𝖾", "latin and mathematical sans-serif");
    test("СігсӀе2", "cyrillic and a digit");
    // New tests for allowed combinations
    test("C言語", "Latin and Han (Japanese)");
    test("C언어", "Latin and Hangul (Korean)");
    test("C語言", "Latin and Han (Chinese)");
    test("Cㄅㄆㄇㄈ", "Latin and Bopomofo (Chinese)");

    println!("\n--- Testing Identifier Extraction ---");
    test_identifiers("let my_var = 42; // a variable");
    test_identifiers("fn process_data(data: &str) -> Result<(), Error> { ... }");
    test_identifiers("mixed_scripts: Сirсlе and normal_script");
    test_identifiers("Καφορας.docx");
    test_identifiers("ﺶﺠﻀ ﺥﺧﺎﺋ");
    test_identifiers("ᚴᚣᚾᚡᚣᚱᛏᚣᚱ ᛫ ᛏᚱᚫᚾᛋᚠᚩᚱᛗᛋ");
}

fn test(s: &str, desc: &str) {
    //let r = s.detect_restriction_level();
    //let b = s.check_restriction_level(unicode_security::RestrictionLevel::ModeratelyRestrictive);
    let b = is_single_script(s);
    println!("{}\t{}\t{:#?}", s, desc, b);
}

fn is_single_script(s: &str) -> bool {
    // Collect all unique scripts in the string, ignoring common ones.
    let scripts_in_string: HashSet<Script> = s
        .chars()
        .map(|c| c.script())
        .filter(|&sc| sc != Script::Common && sc != Script::Inherited && sc != Script::Unknown)
        .collect();

    // If there are no specific scripts (e.g., string is all digits), it's valid.
    // If there is only one specific script, it's also valid.
    if scripts_in_string.len() <= 1 {
        return true;
    }

    // Define the allowed script combinations as specified in UTS #39
    // for moderately restrictive profiles.
    let allowed_combinations: Vec<HashSet<Script>> = vec![
        // Japanese: Latin + Han + Hiragana + Katakana
        HashSet::from([Script::Latin, Script::Han, Script::Hiragana, Script::Katakana]),
        // Korean: Latin + Han + Hangul
        HashSet::from([Script::Latin, Script::Han, Script::Hangul]),
        // Chinese: Latin + Han + Bopomofo
        HashSet::from([Script::Latin, Script::Han, Script::Bopomofo]),
    ];

    // Check if the set of scripts found in the string is a subset of any allowed combination.
    allowed_combinations
        .iter()
        .any(|combo| scripts_in_string.is_subset(combo))
}

/// Extracts identifiers from a string slice based on UAX #31 default identifier definition.
/// An identifier is a sequence of characters starting with a character with the
/// `XID_Start` property, followed by zero or more characters with the `XID_Continue` property.
/// See https://www.unicode.org/reports/tr31/#R1
pub fn extract_identifiers(text: &str) -> Vec<&str> {
    let mut identifiers = Vec::new();
    let mut it = text.char_indices().peekable();
    let mut start_pos: Option<usize> = None;

    while let Some((i, c)) = it.next() {
        if start_pos.is_none() && is_xid_start(c) {
            start_pos = Some(i);
        }

        if let Some(start) = start_pos {
            let next_is_continue = it.peek().map_or(false, |&(_, next_c)| is_xid_continue(next_c));
            if !next_is_continue {
                identifiers.push(&text[start..i + c.len_utf8()]);
                start_pos = None;
            }
        }
    }
    identifiers
}

fn test_identifiers(s: &str) {
    println!("\nOriginal: \"{}\"", s);
    let identifiers = extract_identifiers(s);
    println!("Found identifiers: {:?}", identifiers);
    for identifier in identifiers {
        test(identifier, "extracted identifier");
    }
}
