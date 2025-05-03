// textautodecode
// Read a text file, automatically detecting encoding
//
// 2025-05-02   PV      First version

#![allow(unused)]

use std::path::Path;

use encoding_rs as _;
pub use textautodecode::*;

fn main() {
    println!("TextAutoDecode lib version: {}\n", TextAutoDecode::version());

    let file = Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf8.txt");
    let tad = TextAutoDecode::read_text_file(file)
        .expect("Error decoding file");

    println!("File: {}", file.display());
    println!("Encoding: {:?}", tad.encoding);
    println!("Content:\n{}", tad.text.unwrap());
}
