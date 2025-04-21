// l39_alphanum_filter
// Learning Rust, Filter a string to keep only letters and digits, doubling digits
//
// 2025-04-08   PV      First version

//#![allow(unused)]

use regex::Regex;
use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;

fn main() {
    test("Il était 1 petit navire");
    test("Où ça? La!");
    test("Do (ré) [mi] f'a s0l αβψδ");
    test(
        " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
    );
}

fn test(s: &str) {
    println!("{} -> «{}»", s, filter_alphanum(s));
}

/// Only keep letters and digits, converted to ASCII lowercase, doubling digits
fn filter_alphanum(s: &str) -> String {
    static DIGIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)").unwrap());
    let t = s
        .chars()
        .nfd()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>();
    DIGIT.replace_all(t.as_str(), "$1$1").to_string()
}
