// glyph.rs
// Learning Rust, implementation of a Glyph
//
// 2024-12-14   PV      First version, inefficient, only supporing simple combining diacriticals, but working!
//
// ToDo:
// - Do not store a Vec<char>, a char_indices() iterator is probably enough, unless we return a &[char]
// - Return other than string, either &[char] or &[u8], or alternately byte_index_first/byte_index_last, and/or char_index_first/char_index_last

#[derive(Debug, PartialEq)]
pub struct Glyph {
    pub chars: String,
}

struct GlyphIterator {
    chars: Vec<char>,
    current_char_index: usize,
    current_glyph_index: usize,
}

// Iterator returns (char_index, string)
impl Iterator for GlyphIterator {
    type Item = (usize, Glyph);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_char_index >= self.chars.len() {
            return None;
        }

        let ix_start = self.current_char_index;
        let mut ix_end = self.current_char_index;

        // For now, just include COMBINING DIACRITICAL MARKS U+0300..0.036F
        while ix_end+1 < self.chars.len() && (self.chars[ix_end + 1] as u32 >= 0x0300 && self.chars[ix_end + 1] as u32 <= 0x036F) {
            ix_end += 1;
        }

        let result: Option<Self::Item> = Some((
            self.current_char_index,
            Glyph {
                chars: self.chars.get(ix_start..=ix_end).unwrap().iter().collect(),
            },
        ));

        self.current_char_index = ix_end + 1;
        self.current_glyph_index += 1;

        result
    }
}

impl Glyph {
    pub fn new(chars: &str) -> Self {
        Glyph { chars: String::from(chars) }
    }

    pub fn glyph_indices(s: &str) -> impl Iterator<Item = (usize, Glyph)> {
        let it = GlyphIterator {
            chars: s.chars().collect(),
            current_char_index: 0,
            current_glyph_index: 0,
        };

        it
    }
}
