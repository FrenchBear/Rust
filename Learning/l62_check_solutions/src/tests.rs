// check_solutions tests
//
// 2025-05-15   PV

#![cfg(test)]

use super::*;

#[test]
fn test_split_on_comma() {
    let s =
        "\"097 VB Sort Comics\", \"080-099\\097 VB Sort Comics, Rename dir\\097 VB Sort Comics.vbproj\", \"{2A3C68B6-ECA9-4FD6-A10B-9BD2E13CB006}";
    let v = split_on_comma(s);
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], "\"097 VB Sort Comics\"");
    assert_eq!(v[1], "\"080-099\\097 VB Sort Comics, Rename dir\\097 VB Sort Comics.vbproj\"");
    assert_eq!(v[2], "\"{2A3C68B6-ECA9-4FD6-A10B-9BD2E13CB006}");
}
