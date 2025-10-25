// rwc tests
//
// 2025-04-21   PV

#[cfg(test)]

use crate::*;

#[test]
fn test_count_1() {
    let o = Options {show_only_total:true, ..Default::default()};
    let mut b = DataBag { ..Default::default()};
    process_text(&mut b, "Once upon a time\nWas a King and a Prince\nIn a far, far away kingdom.", "(test)", &o, 68);
    assert_eq!(b.files_count, 1);
    assert_eq!(b.lines_count, 3);
    assert_eq!(b.words_count, 16);
    assert_eq!(b.chars_count, 68);
    assert_eq!(b.bytes_count, 68);
}

#[test]
fn test_count_2() {
    let o = Options {show_only_total:true, ..Default::default()};
    let mut b = DataBag { ..Default::default()};
    // Simple emojis, no ZWJ or emoji attributes
    // Starts and ends with a space, three spaces in the middle, that's two words
    process_text(&mut b, " Aé♫山𝄞🐗   🐷🐽🐖 ", "(test)", &o, 34);
    assert_eq!(b.files_count, 1);
    assert_eq!(b.lines_count, 1);
    assert_eq!(b.words_count, 2);
    assert_eq!(b.chars_count, 14);
    assert_eq!(b.bytes_count, 34);
}


#[test]
fn test_file_ascii() {
    let o = Options {show_only_total:true, ..Default::default()};
    let mut b = DataBag { ..Default::default()};
    process_file(&mut b, Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-ascii.txt"), &o);
    assert_eq!(b.files_count, 1);
    assert_eq!(b.lines_count, 9);
    assert_eq!(b.words_count, 143);
    assert_eq!(b.chars_count, 1145);
    assert_eq!(b.bytes_count, 1145);
}

#[test]
fn test_file_utf8() {
    let o = Options {show_only_total:true, ..Default::default()};
    let mut b = DataBag { ..Default::default()};
    process_file(&mut b, Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf8.txt"), &o);
    assert_eq!(b.files_count, 1);
    assert_eq!(b.lines_count, 9);
    assert_eq!(b.words_count, 143);
    assert_eq!(b.chars_count, 1145);
    assert_eq!(b.bytes_count, 1194);
}

#[test]
fn test_file_utf16lebom() {
    let o = Options {show_only_total:true, ..Default::default()};
    let mut b = DataBag { ..Default::default()};
    process_file(&mut b, Path::new(r"C:\DocumentsOD\Doc tech\Encodings\prenoms-utf16lebom.txt"), &o);
    assert_eq!(b.files_count, 1);
    assert_eq!(b.lines_count, 9);
    assert_eq!(b.words_count, 143);
    assert_eq!(b.chars_count, 1145);
    assert_eq!(b.bytes_count, 2292);
}
