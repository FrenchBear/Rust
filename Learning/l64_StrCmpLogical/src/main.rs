// l64_StrCmpLogical
// Compare strings using Windows shell comparer
//
// 2025-06-30   PV

#![allow(unused)]

use windows::core::{PCWSTR, HRESULT};
use windows::Win32::UI::Shell::StrCmpLogicalW;
use std::ffi::OsString;
use std::os::windows::prelude::*; // For OsStringExt::encode_wide
use std::cmp::Ordering;

/// Compares two strings using the natural sort algorithm, similar to Windows File Explorer.
/// This is a Windows-specific function.
fn str_cmp_logical_w_rust(s1: &str, s2: &str) -> Ordering {
    // Convert Rust &str to null-terminated wide strings (UTF-16) for Windows API.
    // OsString is convenient for this as it handles platform-specific string encoding.
    let os_str1: OsString = s1.into();
    let wide_s1: Vec<u16> = os_str1.encode_wide().chain(std::iter::once(0)).collect(); // Add null terminator
    let p1 = PCWSTR(wide_s1.as_ptr());

    let os_str2: OsString = s2.into();
    let wide_s2: Vec<u16> = os_str2.encode_wide().chain(std::iter::once(0)).collect(); // Add null terminator
    let p2 = PCWSTR(wide_s2.as_ptr());

    // Call the Windows API function
    // The `windows` crate functions typically return a Result,
    // where Ok(0) means success, and a non-zero value for comparison results.
    // StrCmpLogicalW returns an INT, so we directly use its return value.
    let result = unsafe {
        StrCmpLogicalW(p1, p2)
    };

    // StrCmpLogicalW returns:
    // < 0 if psz1 comes before psz2
    //   0 if psz1 is identical to psz2
    // > 0 if psz1 comes after psz2
    // We map this to Rust's Ordering enum.
    result.cmp(&0)
}

fn main() {
    let mut filenames = vec![
        "file1.txt",
        "file10.txt",
        "file2.txt",
        "another_file.txt",
        "Image_1.jpg",
        "Image_10.jpg",
        "Image_2.jpg",
        "folder A",
        "folder B",
        "folder 1",
        "folder 10",
        "folder 2",
        "archive.zip",
    ];

    println!("Original filenames:");
    for f in &filenames {
        println!("{}", f);
    }

    // Sort using standard lexicographical sort
    let mut lexical_sorted = filenames.clone();
    lexical_sorted.sort(); // sort() uses lexicographical comparison by default for strings
    println!("\nSorted (Lexicographical - standard Rust sort()):");
    for f in &lexical_sorted {
        println!("{}", f);
    }

    // Sort using StrCmpLogicalW in Rust
    let mut natural_sorted = filenames.clone();

    // Use slice::sort_by with our custom comparison function
    natural_sorted.sort_by(|a, b| str_cmp_logical_w_rust(a, b));

    println!("\nSorted (Natural Sort - Windows File Explorer style):");
    for f in &natural_sorted {
        println!("{}", f);
    }
}
