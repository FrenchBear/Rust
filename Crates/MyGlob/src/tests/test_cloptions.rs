// MyGlob tests - test_cloptions
// Unit tests for MyGlobCLOptions
//
// 2025-11-16   PV

#![cfg(test)]
use crate::*;

#[test]
fn test_cl_options_1() {
    let mut mgclo = MyGlobCLOptions::new();
    assert!(mgclo.process_options("a+,cs,l2,md 3,ngf").is_ok()) ;
    assert_eq!(mgclo.autorecurse, true);
    assert_eq!(mgclo.case_sensitive, true);
    assert_eq!(mgclo.link_mode, 2);
    assert_eq!(mgclo.max_depth, 3);
    assert_eq!(mgclo.no_glob_filtering, true);
    assert!(mgclo.filters.is_empty());
}

#[test]
fn test_cl_options_2() {
    let mut mgclo = MyGlobCLOptions::new();
    assert!(mgclo.process_options("cs").is_ok());
    assert!(mgclo.process_options("fbin,f obj").is_ok());
    assert_eq!(mgclo.case_sensitive, true);
    assert_eq!(mgclo.filters.len(), 2);
    assert_eq!(mgclo.filters[0], "bin");
    assert_eq!(mgclo.filters[1], "obj");
}
