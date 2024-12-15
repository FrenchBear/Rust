// glyph2.rs
// Learning Rust, implementation of a Glyph2 to iterate over graphemes
//
// Note that the actual rules defined in Unicode Standard Annex #29 Unicode Text Segmentation are much more complex,
// Emoji detection and combination rules defined in Unicode Technical Standard #51are also more complex.
// This is just a learning exercise. For full implementation of the rules, use rust unicode_segmentation crate.
//
// 2024-12-13   PV      First version, inefficient, only supporing simple combining diacriticals, but working!
// 2024-12-14   PV      Store char_indices iterator instead of a Vec<char>; return ranges rather than strings

use core::ops::Range;
use std::str::CharIndices;

// Returned by glyph2_indices iterator
#[derive(Debug, PartialEq)]
pub struct Glyph2 {
    pub byte_range: Range<usize>,
    pub char_range: Range<usize>,
}

// Private internal iterator object storing current state
struct Glyph2Iterator<'a> {
    byte_count: usize,                          // Length of the string in UTF-8 bytes
    charit: CharIndices<'a>,                    // Iterator over string returning Option<(usize, char)>
    current_char_index: usize,                  // Current position in chars in string, charit provides current position in bytes
    next_charindice_opt: Option<(usize, char)>, // charit return for next character, since we must read one ahead to decide if we combine or not
}

// Iterator returns (char_index, string)
impl<'a> Iterator for Glyph2Iterator<'a> {
    type Item = Glyph2;

    fn next(&mut self) -> Option<Glyph2> {
        let current_charindice_opt = if self.next_charindice_opt.is_some() {
            self.next_charindice_opt
        } else {
            self.charit.next()
        };
        if current_charindice_opt.is_none() {
            return None;
        }

        self.next_charindice_opt = self.charit.next();

        // byte index of current char, covering bix_start..=bix_end
        let bix_start = current_charindice_opt.unwrap().0;
        let mut bix_end = if self.next_charindice_opt.is_some() {
            self.next_charindice_opt.unwrap().0 - 1
        } else {
            self.byte_count - 1
        };

        // char index, start with current char
        let mut cix_end = self.current_char_index;

        // current codepoint
        let mut cc = current_charindice_opt.unwrap().1 as u32;

        // Graphemes detection and combination loop
        while self.next_charindice_opt.is_some() {
            let nc = self.next_charindice_opt.unwrap().1 as u32; // next codepoint
            if is_combining(nc) || is_emoji(cc) && (is_zwj(nc) || is_vs(nc)) || is_vs(cc) && is_zwj(nc) || is_zwj(cc) && is_emoji(nc) {
                // We combine, fetch next character
                self.next_charindice_opt = self.charit.next();

                // Adjust byte end
                bix_end = if self.next_charindice_opt.is_some() {
                    self.next_charindice_opt.unwrap().0 - 1
                } else {
                    self.byte_count - 1
                };

                // adjust char end
                cix_end += 1;

                cc = nc;
            } else {
                break;
            }
        }

        let result: Option<Glyph2> = Some(Glyph2 {
            byte_range: bix_start..bix_end+1,
            char_range: self.current_char_index..cix_end+1,
        });

        self.current_char_index = cix_end + 1;

        result
    }
}

// COMBINING DIACRITICAL MARKS U+0300..0.036F
fn is_combining(cp: u32) -> bool {
    cp >= 0x0300 && cp <= 0x036F
}

// Quick-and-dirty test
fn is_emoji(cp: u32) -> bool {
    cp>=0x2600 && cp<=0x26FF        // MISCELLANEOUS SYMBOLS
    || cp>=2700 && cp<=0x27BF       // DINGBATS
    || cp>=0x1F300 && cp<=0x1F5FF   // MISCELLANEOUS SYMBOLS AND PICTOGRAPHS
    || cp>=0x1F600 && cp<=0x1F64F   // EMOTICONS
    || cp>=0x1F680 && cp<=0x1F6FF   // TRANSPORT AND MAP SYMBOLS
    || cp>=0x1F900 && cp<=0x1F9FF   // SUPPLEMENTAL SYMBOLS AND PICTOGRAPHS
    || cp>=0x1FA70 && cp<=0x1FAFF // SYMBOLS AND PICTOGRAPHS EXTENDED-A
}

// U+200D ZERO WIDTH JOINER
fn is_zwj(cp: u32) -> bool {
    cp == 0x200D
}

fn is_vs(cp: u32) -> bool {
    cp>=0xFE00 && cp<=0xFE0F        // VARIATION SELECTORS
    || cp>=0x1F3FB && cp<=0x1F3FF // EMOJI MODIFIER FITZPATRICK
}

impl Glyph2 {
    pub fn glyph2_indices<'a>(s: &'a str) -> impl Iterator<Item = Glyph2> + 'a {
        Glyph2Iterator {
            byte_count: s.len(),
            charit: s.char_indices(),
            current_char_index: 0,
            next_charindice_opt: None,
        }
    }
}
