// checkutf8: Check that files have utf-8 encoding
//
// 2025-03-16	PV      First version

// https://docs.rs/glob/latest/glob/index.html
// https://docs.rs/walkdir/latest/walkdir/
// https://docs.rs/ignore/latest/ignore/index.html

// https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html

#![allow(unused_imports, dead_code)]

// standard library imports
use std::fs::{self, File};
use std::io::{self, BufReader, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::os::windows::prelude::*;

// external crates imports
// use getopt::Opt;
use encoding_rs::{Encoding, UTF_8, UTF_16LE, WINDOWS_1252};
use glob::{MatchOptions, glob_with};
use ignore::Walk;
use walkdir::WalkDir;

fn main() {
    //test_walkdir();
    test_ignore();
}

fn test_ignore() {
    for result in Walk::new(r"C:\Development\GitVSTS") {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => {
                let ft = entry.file_type().unwrap(); // file_type() returns Option<FileType>
                if ft.is_dir() {
                    //println!("Dir: {}", entry.path().display());
                } else if ft.is_file() {
                    let filename = entry.file_name().to_string_lossy();
                    if filename.ends_with(".cs") {
                        check_file(entry.path());
                        //println!("File: {}", entry.path().display());
                    }
                } else if ft.is_symlink() {
                    //println!("SymLink: {}", entry.path().display());
                }
            }
            Err(_e) => {
                //println!("Err: {}", _e);
            }
        }
    }
}

// Detect Windows hidden files
pub fn is_hidden(file_path: &std::path::PathBuf) -> std::io::Result<bool> {
    let metadata = fs::metadata(file_path)?;
    let attributes = metadata.file_attributes();

    if (attributes & 0x2) > 0 { Ok(true) } else { Ok(false) }
}

fn test_walkdir() {
    // for entry in WalkDir::new(r"C:\Development").into_iter().filter_map(|e| e.ok()) {
    //     println!("{}", entry.path().display());
    // }

    for result in WalkDir::new(r"C:\Development") {
        match result {
            Ok(entry) => {
                let ft = entry.file_type();
                if ft.is_dir() {
                    println!("Dir: {}", entry.path().display());
                } else if ft.is_file() {
                    println!("File: {}", entry.path().display());
                } else if ft.is_symlink() {
                    println!("SymLink: {}", entry.path().display());
                }
            }
            Err(e) => {
                println!("Err: {}", e);
            }
        }
    }
}

fn zzmain() {
    //let source = r"C:\Development\GitVSTS\UIApps\Net9\**\*.cs";
    let source = r"C:\Development\**\*.cs";

    let mo = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut count = 0;
    match glob_with(source, mo) {
        Ok(paths) => {
            for entry in paths {
                match entry {
                    Ok(pb) => {
                        count += 1;
                        check_file(&pb);
                    }
                    Err(_) => {
                        // Ignore silently
                    }
                };
            }
        }
        Err(err) => {
            println!("rsgrep: pattern error {}", err);
            count = -1;
        }
    }

    if count == 0 {
        println!("warning: no file found matching {}", source);
    }
}

fn check_file(path: &Path) {
    let res = check_utf8(path);
    if res.is_err() {
        println!("{}", path.display());
    }
}

//pub fn check_utf8(path: &Path) -> Result<bool, io::Error> {
pub fn check_utf8(path: &Path) -> std::io::Result<bool> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let (_, used_encoding, had_errors) = UTF_8.decode(&buffer);
    if !had_errors && used_encoding == UTF_8 {
        return Ok(true);
    }

    // If none of the encodings worked without errors, return an error.
    Err(io::Error::new(io::ErrorKind::InvalidData, "File does not appear to be UTF-8"))
}
