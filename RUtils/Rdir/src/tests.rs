// rdir tests
//
// 2025-10-24   PV

#[cfg(test)]

use crate::*;

#[test]
fn test_strip_quotes() {
    assert_eq!(strip_quotes("C:\\Documents\\Hello.doc"), "C:\\Documents\\Hello.doc");
    assert_eq!(strip_quotes("\"C:\\Documents\\Hello.doc\""), "C:\\Documents\\Hello.doc");
}


#[test]
fn test_show_invisible_chars() {
    assert_eq!(show_invisible_chars("C:\\Documents\\Hello.doc"), "C:\\Documents\\Hello.doc");
}

#[test]
fn test_get_formatted_size() {
    assert_eq!(get_formatted_size(0), "0");
    assert_eq!(get_formatted_size(123), "123\u{00A0}B");
    assert_eq!(get_formatted_size(385_976_527_801), "385\u{00A0}976\u{00A0}527\u{00A0}801\u{00A0}B (359\u{00A0}GiB)");
}
