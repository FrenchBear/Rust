// runiq tests
//
// 2025-10-31   PV

#[cfg(test)]
use crate::*;

#[allow(unused_macros)] // Without that, I got a warning in VSCode that indexmap is unused...
macro_rules! indexmap {
    ($( $key:expr => $val:expr ),* $(,)?) => {{
        let mut map = IndexMap::new();
        {};
        $( map.insert($key.to_string(), $val); )*
        map
    }};
}

// A simple macro_rules! macro to create an owned `String` from a string literal
#[macro_export] // Export the macro to make it available in other modules/crates
macro_rules! S {
    ( $e:expr ) => {
        String::from($e)
    };
}

#[test]
fn test_build_map_1() {
    let data = vec!["One", "Two", "One", "two"].into_iter().map(|s| Ok(s.to_string()));
    let map = build_map(data, false);

    let expected = indexmap! {
        S!("One") => vec![
            S!("One"),
            S!("One"),
        ],
        S!("Two") => vec![
            S!("Two"),
        ],
        S!("two") => vec![
            S!("two"),
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
            S!("One"),
            S!("One"),
        ],
        S!("two") => vec![
            S!("Two"),
            S!("two"),
        ],
    };

    assert_eq!(map, expected);
}

// ----

#[test]
fn test_result_unique_case_sensitive() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"]
        .into_iter()
        .map(|s| Ok(s.to_string()));
    let map = build_map(data, false);
    let res = final_iterator(&map, Output::Unique)
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let expected: Vec<String> = vec![S!("One"), S!("Two"), S!("two"), S!("Three"), S!("three")];

    assert_eq!(res, expected);
}

#[test]
fn test_result_unique_ignore_case() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"]
        .into_iter()
        .map(|s| Ok(s.to_string()));
    let map = build_map(data, true);
    let res = final_iterator(&map, Output::Unique)
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let expected: Vec<String> = vec![S!("One"), S!("Two"), S!("Three")];

    assert_eq!(res, expected);
}

// ----

#[test]
fn test_result_repeated_case_sensitive() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"]
        .into_iter()
        .map(|s| Ok(s.to_string()));
    let map = build_map(data, false);
    let res = final_iterator(&map, Output::Repeated)
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let expected: Vec<String> = vec![S!("Three")];

    assert_eq!(res, expected);
}

#[test]
fn test_result_repeated_ignore_case() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"]
        .into_iter()
        .map(|s| Ok(s.to_string()));
    let map = build_map(data, true);
    let res = final_iterator(&map, Output::Repeated)
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let expected: Vec<String> = vec![S!("Two"), S!("Three")];

    assert_eq!(res, expected);
}

// ----

#[test]
fn test_result_allrepeated_case_sensitive() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"]
        .into_iter()
        .map(|s| Ok(s.to_string()));
    let map = build_map(data, false);
    let res = final_iterator(&map, Output::AllRepeated)
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let expected: Vec<String> = vec![S!("Three"), S!("Three")];

    assert_eq!(res, expected);
}

#[test]
fn test_result_allrepeated_ignore_case() {
    let data = vec!["One", "Two", "two", "Three", "Three", "three"]
        .into_iter()
        .map(|s| Ok(s.to_string()));
    let map = build_map(data, true);
    let res = final_iterator(&map, Output::AllRepeated)
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let expected: Vec<String> = vec![S!("Two"), S!("two"), S!("Three"), S!("Three"), S!("three")];

    assert_eq!(res, expected);
}
