// Simple test for Glyph2 enumerator
// Learning rust
//
// 2024-12-14   PV

mod glyph2;

use glyph2::Glyph2;

fn main() {
    let s = "Ae\u{0301}𝄞â̧̅🐗🐻‍❄️👨🏾‍❤️‍💋‍👨🏻";

    for gr in Glyph2::glyph2_indices(s) {
        let ss = &s[gr.byte_range.clone()];
        println!("{:?}\t{:?}\t{}", gr.byte_range, gr.char_range, ss);
    }

    println!("{:?}", get_glyph_from_byteindex(s, 0));
    println!("{:?}", get_glyph_from_byteindex(s, 1));
    println!("{:?}", get_glyph_from_byteindex(s, 4));
    println!("{:?}", get_glyph_from_byteindex(s, 8));
    println!("{:?}", get_glyph_from_byteindex(s, 15));
    println!("{:?}", get_glyph_from_byteindex(s, 19));
    println!("{:?}", get_glyph_from_byteindex(s, 32));

    println!("{:?}", get_glyph_from_byteindex("ABC", 1));
    println!("{:?}", get_glyph_from_byteindex("🐗", 1));        // Should panic
}

pub fn get_glyph_from_byteindex(s: &str, byte_index: usize) -> Glyph2 {
    get_glyphresult_from_byteindex(s, byte_index, true).unwrap()
}

pub fn get_glyphoption_from_byteindex(s: &str, byte_index: usize) -> Option<Glyph2> {
    get_glyphresult_from_byteindex(s, byte_index, false)
}

// Private base function
fn get_glyphresult_from_byteindex(s: &str, byte_index: usize, should_panic: bool) -> Option<Glyph2> {
    if byte_index >= s.len() {
        if should_panic {
            panic!("index out of bounds: the len is {} but the index is {}", s.len(), byte_index);
        } else {
            return None;
        }
    }

    //let mut lmax: usize = 0;
    for g in Glyph2::glyph2_indices(s) {
        if byte_index == *g.byte_range.start() {
            return Some(g);
        }
        if byte_index <= *g.byte_range.end() {
            if should_panic {
                panic!(
                    "byte index {} is not a glyph boundary; it is inside '{}' (bytes {}..={})",
                    byte_index,
                    &s[g.byte_range.clone()],
                    *g.byte_range.start(),
                    *g.byte_range.end()
                );
            }
            return None;
        }
        //lmax = *g.byte_range.end() + 1;
    }
    None    // Actually we should never get here
}