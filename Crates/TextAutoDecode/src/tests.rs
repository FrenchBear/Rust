// tests.rs - textautodecode tests
//
// 2025-05-02   PV

// ToDo: add invalid files and make sure they're correctly detected as non-text

#![cfg(test)]

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
    let t = get_fmt(Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf16bbeom.txt"));
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
