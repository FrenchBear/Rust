// textautodecode
// Read a text file, automatically detecting encoding
//
// 2025-05-02   PV      First version

#![allow(unused)]

use std::path::Path;

use tempfile as _;
use encoding_rs as _;

pub use textautodecode::*;

fn main() {
    println!("TextAutoDecode lib version: {}\n", TextAutoDecode::version());

    // let file = Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf8.txt");
    let file = Path::new(r"C:\Temp\f1.txt");
    let file = Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf16be.txt");
    let file = Path::new(r"C:\Development\GitVSTS\WPF\FW4.8\Learning\WPF24 TaskDialog\TaskDialog\TaskDialogInterop.cs");

    let tad = TextAutoDecode::read_text_file(file)
        .expect("Error decoding file");

    println!("File: {}", file.display());
    println!("Encoding: {:?}", tad.encoding);
    if let Some(txt) = tad.text {
        // println!("{}", txt);
    }
}
