// rwc tests
//
// 2025-04-21   PV

#[cfg(test)]

use crate::*;

#[test]
fn text_count_1() {
    let o = Options {show_only_total:true, ..Default::default()};
    let mut b = DataBag { ..Default::default()};
    process_text(&mut b, "Once upon a time\nWas a King and a Prince\nIn a far, far away kingdom.", "(test)", &o);
    assert_eq!(b.files_count, 1);
    assert_eq!(b.lines_count, 3);
    assert_eq!(b.words_count, 16);
    assert_eq!(b.chars_count, 68);
    assert_eq!(b.bytes_count, 68);
}

#[test]
fn text_count_2() {
    let o = Options {show_only_total:true, ..Default::default()};
    let mut b = DataBag { ..Default::default()};
    // Simple emojis, no ZWJ or emoji attributes
    // Starts and ends with a space, three spaces in the middle, that's two words
    process_text(&mut b, " Aé♫山𝄞🐗   🐷🐽🐖 ", "(test)", &o);
    assert_eq!(b.files_count, 1);
    assert_eq!(b.lines_count, 1);
    assert_eq!(b.words_count, 2);
    assert_eq!(b.chars_count, 14);
    assert_eq!(b.bytes_count, 34);
}
