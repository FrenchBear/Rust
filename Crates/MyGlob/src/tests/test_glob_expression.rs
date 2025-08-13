// MyGlob tests - test_glob_expression
// Unit tests for glob_to_segments
//
// 2025-04-09   PV
// 2025-04-23   PV      Added search_error tests
// 2025-07-13   PV      Tests with chinese characters

#![cfg(test)]
use crate::*;

#[test]
fn break1() {
    //let res = MyGlobBuilder::glob_to_segments(r"C:\fichier.dll").unwrap();
}

#[test]
fn glob_ending_with_recurse() {
    // Special case, when a glob pattern ends with **, then \* is automatically added
    let res = MyGlobBuilder::glob_to_segments("**\\").unwrap();
    assert_eq!(res.len(), 2);
    match &res[0] {
        Segment::Recurse => {}
        _ => panic!(),
    }
    match &res[1] {
        Segment::Filter(re) => assert_eq!(re.as_str(), "(?i)^.*$"),
        _ => panic!(),
    }
}

#[test]
fn relative_glob() {
    // glob_to_segments parameter must end with \\
    let res = MyGlobBuilder::glob_to_segments("*\\target\\").unwrap();
    assert_eq!(res.len(), 2);
    match &res[0] {
        Segment::Filter(_) => {}
        _ => panic!(),
    }
    match &res[1] {
        Segment::Constant(k) => assert_eq!(k, "target"),
        _ => panic!(),
    }
}

#[test]
fn test_get_root() {
    tgr("", ".", "*");
    tgr("*", ".", "*");
    tgr("C:", "C:", "");
    tgr("C:\\", "C:\\", "");
    tgr("file.ext", "file.ext", "");
    tgr("C:file.ext", "C:file.ext", "");
    tgr("C:\\file.ext", "C:\\file.ext", "");
    tgr("path\\file.ext", "path\\file.ext", "");
    tgr("path\\*.jpg", "path\\", "*.jpg");
    tgr("path\\**\\*.jpg", "path\\", "**\\*.jpg");
    tgr("C:path\\file.ext", "C:path\\file.ext", "");
    tgr("C:\\path\\file.ext", "C:\\path\\file.ext", "");
    tgr("\\\\server\\share", "\\\\server\\share", "");
    tgr("\\\\server\\share\\", "\\\\server\\share\\", "");
    tgr("\\\\server\\share\\file.txt", "\\\\server\\share\\file.txt", "");
    tgr("\\\\server\\share\\path\\file.txt", "\\\\server\\share\\path\\file.txt", "");
    tgr("\\\\server\\share\\*.jpg", "\\\\server\\share\\", "*.jpg");
    tgr("\\\\server\\share\\path\\*.jpg", "\\\\server\\share\\path\\", "*.jpg");
    tgr("\\\\server\\share\\**\\*.jpg", "\\\\server\\share\\", "**\\*.jpg");
}

fn tgr(pat: &str, root: &str, rem: &str) {
    let (r, s) = MyGlobBuilder::get_root(pat);
    assert_eq!(r, root);
    assert_eq!(s, rem);
}
