// l67_format_f64
// Compare three libraries to format f64 numbers with thousand separators and three decimals
//
// 2025-10-26   PV

// I've wasted more than hour trying to use num_format for that, both Gemini and ChatGPT providing continuously invalid,
// uncompilable code, using inexistent methods... without telling that num_format is designed for format integers, not
// float numbers. And crate home page doesn't mention this either.
// On the plus side, it supports French locale out-of-the-box, it's really simple to use.
// Probably a good choice for integer-only formatting.
// Last maintained 3 years ago, 80K downloads a week.
//
// format_num crate is simple to use, but decimal separator and thousand grouping separator are hardcoded, so we use
// string.replace to convert to French Locale format.
// Potentially interesting, trying to emulate Python 3's format specification mini-language (PEP3101) with some minor
// implementation details changes.
// Note that integers are formatted after conversion to f64 (using Into<f64> trait)
// It doesn't look maintained, 1 version 0.1.0 produced 5 years ago, and only 150-200 downloads a week.
//
// Finally numfmt crate looks powerful with configurable decimal separator, thousand separator, scaling, prefix and suffix,
// but it's not possible to use \u{00A0} as a thousand separator since its utf-8 length is >1, and rounding is incorrect:
// setting precision truncates the value at the requested precision without rounding.
// There is a parser to customize format using a string rather than calling functions to build a formatter, simpler
// and faster than format_num format string, but with 
// An interesting option to format file sizes using scaling and suffix (there is an example on crate Readme).
// Last maintained 3 months ago, 1.5K downloads a week.

//#![allow(unused)]

// External imports
use num_format::{Locale, ToFormattedString};

use format_num::format_num;

use numfmt::{Formatter, Precision, Scales};


fn main() {
    // num_format crate, primarily designed for formatting integers
    let n = 131285602;
    let n_formatted = n.to_formatted_string(&Locale::fr); // Use French locale for now. Later we will find the user locale.
    println!("num_format: {}", n_formatted);
    println!();

    // format_num crate
    let f: f64 = 1234567.891734;
    let formatted_string = format_num!(",.3f", f).replace(',', "\u{00A0}").replace('.', ",");
    println!("format_num f64: {}", formatted_string);
    let formatted_string = format_num!(",d", n).replace(',', "\u{00A0}");
    println!("format_num i32: {}", formatted_string);
    println!();

    // numfmt crate
    let mut formatter = Formatter::new()
        .comma(true)
        .separator(' ').unwrap() // Use space for thousand separator   [should use non-breaking space]
        .precision(Precision::Decimals(3)) // Set 3 decimal places
        .scales(Scales::none()); // Don't use scaling (like K, M, G)
    let formatted_string = formatter.fmt2(f).replace(' ', "\u{00A0}");
    println!("numfmt f64: {}", formatted_string);

    // Parse the format string:
    // ,    Use ',' (comma) as the decimal separator
    // 3    Set 3 decimal places
    // n    No scaling
    // /(space)  Use ' ' (space) as the thousands separator (must be 1 utf-8 long, can't use '\u{00A0}')
    let mut formatter: Formatter = "[,3n/ ]".parse().unwrap();
    let formatted_string = formatter.fmt2(f).replace(' ', "\u{00A0}");
    println!("numfmt f64 alt: {}", formatted_string);
    
    let mut f = Formatter::new()
        .separator(' ').unwrap()
        .precision(Precision::Decimals(0));
    let s = f.fmt2(n).replace(' ', "\u{00A0}");
    println!("numfmt i32: {}", s);
}
