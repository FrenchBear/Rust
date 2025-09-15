// tests.rs - MyMarkup tests
//
// 2025-09-15   PV      First actual test

#![cfg(test)]

use std::{fs, io};

use crate::*;

#[test]
fn test_logwriter() -> io::Result<()> {
    let mut lw = new("test", "1.2.3", true);
    logln(&mut lw, "Hello");
    logln(&mut lw, "*** Error: message");
    logln(&mut lw, "dbg: Debugging info");

    let pb = lw.get_path().unwrap();
    let file_path = pb.as_path();
    let content = fs::read_to_string(file_path)?;
    assert_eq!(content,"test 1.2.3\nHello\n*** Error: message\ndbg: Debugging info\n");
    fs::remove_file(file_path).unwrap();

    Ok(())
}
