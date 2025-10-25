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
