// tests.rs - textautodecode tests
//
// 2025-05-02   PV
// 2025-05-06   PV      Tests with non-text content
// 2025-06-24   PV      Added check_utf8 checks for truncated UTF-8 sequence at the end of a 1000 bytes buffer

#![cfg(test)]

use std::io::Write;
use std::io::Error;
use tempfile::Builder;

use crate::*;

fn get_fmt(p: &Path) -> TextAutoDecode {
    let r = TextAutoDecode::read_text_file(p);
    match r {
        Ok(tad) => tad,
        Err(e) => panic!("{}", e),
    }
}


#[test]
fn test_empty() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-empty.txt"));
    assert_eq!(t.encoding, TextFileEncoding::Empty);
    assert!(t.text.unwrap().is_empty());
}

#[test]
fn test_ascii() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-ascii.txt"));
    assert_eq!(t.encoding, TextFileEncoding::ASCII);
    assert!(t.text.unwrap().starts_with("juliette sophie brigitte geraldine"));
}

#[test]
fn test_utf8() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf8.txt"));
    assert_eq!(t.encoding, TextFileEncoding::UTF8);
    assert!(t.text.unwrap().starts_with("juliette sophie brigitte géraldine"));
}

#[test]
fn test_utf8bom() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf8bom.txt"));
    assert_eq!(t.encoding, TextFileEncoding::UTF8BOM);
    assert!(t.text.unwrap().starts_with("juliette sophie brigitte géraldine"));
}

#[test]
fn test_utf16lebom() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf16lebom.txt"));
    assert_eq!(t.encoding, TextFileEncoding::UTF16LEBOM);
    assert!(t.text.unwrap().starts_with("juliette sophie brigitte géraldine"));
}

#[test]
fn test_utf16le() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf16le.txt"));
    assert_eq!(t.encoding, TextFileEncoding::UTF16LE);
    assert!(t.text.unwrap().starts_with("juliette sophie brigitte géraldine"));
}

#[test]
fn test_utf16bbeom() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf16bebom.txt"));
    assert_eq!(t.encoding, TextFileEncoding::UTF16BEBOM);
    assert!(t.text.unwrap().starts_with("juliette sophie brigitte géraldine"));
}

#[test]
fn test_utf16be() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf16be.txt"));
    assert_eq!(t.encoding, TextFileEncoding::UTF16BE);
    assert!(t.text.unwrap().starts_with("juliette sophie brigitte géraldine"));
}

#[test]
fn test_oem850() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-oem850.txt"));
    assert_eq!(t.encoding, TextFileEncoding::EightBit);
    // Can't actually use text since it's OEM850 read witn a WINDOWS1252 decoder
}

#[test]
fn test_1252() {
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-1252.txt"));
    assert_eq!(t.encoding, TextFileEncoding::EightBit);
    assert!(t.text.unwrap().starts_with("juliette sophie brigitte géraldine"));
}

#[test]
fn test_utf16_too_short() -> Result<(), io::Error> {
    let model: [u8; 8] = [
        // <= 20 bytes, won't be recognized as a valid UTF-16 file
        0x41, 0x00, // A
        b'\n', 0x00, // Unix EOL
        0x61, 0x00, // a
        b'\n', 0x00, // Unix EOL
    ];

    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&model)?;
    let t = get_fmt(temp_file.path());
    assert_eq!(t.encoding, TextFileEncoding::NotText);
    Ok(())
}

#[test]
fn test_utf8_with_binary_char() -> Result<(), io::Error> {
    let model: [u8; 9] = [
        0xEF, 0xBB, 0xBF,  // UTF-8 BOM
        0x41, 0x42, 0x43,  // ABC
        0x07,              // BEL: Not a valid text char!
        0x0D, 0x0A,        // CR, LF
    ];
    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&model)?;
    let t = get_fmt(temp_file.path());
    assert_eq!(t.encoding, TextFileEncoding::NotText);
    Ok(())
}

#[test]
fn test_utf8_truncate_1() {
    let model: [u8; 1000] = [b'A'; 1000];
    let res = TextAutoDecode::check_utf8(&model, 1000);
    assert!(res.is_some());
}

#[test]
fn test_utf8_truncate_2() {
    let mut model: [u8; 1000] = [b'A'; 1000];
    model[999] = 0b11000000;    // byte 1 of a sequence of 2 bytes
    let res = TextAutoDecode::check_utf8(&model, 1000);
    assert!(res.is_some());
}

#[test]
fn test_utf8_truncate_3() {
    let mut model: [u8; 1000] = [b'A'; 1000];
    model[998] = 0b11100000;    // byte 1 of a sequence of 3 bytes
    model[999] = 0b10000000;    // Folowing byte
    let res = TextAutoDecode::check_utf8(&model, 1000);
    assert!(res.is_some());
}

#[test]
fn test_utf8_truncate_4() {
    let mut model: [u8; 1000] = [b'A'; 1000];
    model[998] = 0b11110000;    // byte 1 of a sequence of 4 bytes
    model[999] = 0b10000000;    // Folowing byte
    let res = TextAutoDecode::check_utf8(&model, 1000);
    assert!(res.is_some());
}

#[test]
fn test_utf8_truncate_5() {
    let mut model: [u8; 1000] = [b'A'; 1000];
    model[996] = 0b10000000;    // Folowing byte, 4 following bytes is not allowed
    model[997] = 0b10000000;    // Folowing byte
    model[998] = 0b10000000;    // Folowing byte
    model[999] = 0b10000000;    // Folowing byte
    let res = TextAutoDecode::check_utf8(&model, 1000);
    assert!(!res.is_some());
}

#[test]
fn test_utf8_truncate_6() {
    let mut model: [u8; 1000] = [b'A'; 1000];
    let mut i=0;
    // Fill with BOAR U+1F417 = F0 9F 90 97
    while i<1000 {
        model[i] = 0xF0;
        model[i+1] = 0x9F;
        model[i+2] = 0x90;
        model[i+3] = 0x97;
        i+=4
    }
    let res = TextAutoDecode::check_utf8(&model, 1000);
    assert!(res.is_some());
}
