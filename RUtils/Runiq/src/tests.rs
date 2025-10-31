// runiq tests
//
// 2025-10-31   PV

#[cfg(test)]

use crate::*;

#[allow(unused_macros)]     // Without that, I got a warning in VSCode that indexmap is unused...
macro_rules! indexmap {
    ($( $key:expr => $val:expr ),* $(,)?) => {{
        let mut map = IndexMap::new();
        {};
        $( map.insert($key.to_string(), $val); )*
        map
    }};
}

#[test]
fn test_build_map_1() {
    let data = vec!["One", "Two", "One", "two"].into_iter().map(|s| Ok(s.to_string()));
    let map = build_map(data, false);

    let expected = indexmap! {
        "One".to_string() => vec![
            "One".to_string(),
            "One".to_string(),
        ],
        "Two".to_string() => vec![
            "Two".to_string(),
        ],
        "two".to_string() => vec![
            "two".to_string(),
        ],
    };

    assert_eq!(map, expected);
}

// Ignoring case
#[test]
fn test_build_map_2() {
    let data = vec!["One", "Two", "One", "two"].into_iter().map(|s| Ok(s.to_string()));
    let map = build_map(data, true);

    let expected = indexmap! {
        "one".to_string() => vec![
            "One".to_string(),
            "One".to_string(),
        ],
        "two".to_string() => vec![
            "Two".to_string(),
            "two".to_string(),
        ],
    };

    assert_eq!(map, expected);
}

// ----

#[test]
fn test_result_unique_case_sensitive() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"].into_iter().map(|s| Ok(s.to_string()));
    let map = build_map(data, false);
    let res = final_iterator(&map, Output::Unique).into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let expected: Vec<String> = vec! {
        "One".to_string(),
        "Two".to_string(),
        "two".to_string(),
        "Three".to_string(),
        "three".to_string(),
    };

    assert_eq!(res, expected);
}

#[test]
fn test_result_unique_ignore_case() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"].into_iter().map(|s| Ok(s.to_string()));
    let map = build_map(data, true);
    let res = final_iterator(&map, Output::Unique).into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let expected: Vec<String> = vec! {
        "One".to_string(),
        "Two".to_string(),
        "Three".to_string(),
    };

    assert_eq!(res, expected);
}

// ----

#[test]
fn test_result_repeated_case_sensitive() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"].into_iter().map(|s| Ok(s.to_string()));
    let map = build_map(data, false);
    let res = final_iterator(&map, Output::Repeated).into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let expected: Vec<String> = vec! {
        "Three".to_string(),
    };

    assert_eq!(res, expected);
}

#[test]
fn test_result_repeated_ignore_case() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"].into_iter().map(|s| Ok(s.to_string()));
    let map = build_map(data, true);
    let res = final_iterator(&map, Output::Repeated).into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let expected: Vec<String> = vec! {
        "Two".to_string(),
        "Three".to_string(),
    };

    assert_eq!(res, expected);
}

// ----

#[test]
fn test_result_allrepeated_case_sensitive() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"].into_iter().map(|s| Ok(s.to_string()));
    let map = build_map(data, false);
    let res = final_iterator(&map, Output::AllRepeated).into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let expected: Vec<String> = vec! {
        "Three".to_string(),
        "Three".to_string(),
    };

    assert_eq!(res, expected);
}

#[test]
fn test_result_allrepeated_ignore_case() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"].into_iter().map(|s| Ok(s.to_string()));
    let map = build_map(data, true);
    let res = final_iterator(&map, Output::AllRepeated).into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let expected: Vec<String> = vec! {
        "Two".to_string(),
        "two".to_string(),
        "Three".to_string(),
        "Three".to_string(),
        "three".to_string(),
    };

    assert_eq!(res, expected);
}
