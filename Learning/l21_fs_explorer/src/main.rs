// l21_fs_explorer: Various code to test exploring filesystem
//
// 2025-03-16	PV      First version
// 2025-03-23   PV      check_filenames
// 2025-04-21   PV      Clippy optimizations

// https://docs.rs/glob/latest/glob/index.html
// https://docs.rs/walkdir/latest/walkdir/
// https://docs.rs/ignore/latest/ignore/index.html

// https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html

#![allow(unused_imports, dead_code, unused_variables)]

// standard library imports
use std::fs::{self, File};
use std::io::{self, BufReader, ErrorKind, Read, Write};
use std::os::windows::prelude::*;
use std::path::{Path, PathBuf};

// external crates imports
use encoding_rs::{Encoding, UTF_8, UTF_16LE, WINDOWS_1252};
use glob::{MatchOptions, glob_with};
use ignore::{DirEntry, Walk as WalkIgnore, WalkBuilder};
use rust_search::{DirEntry as RSDirEntry, FilterExt, FilterFn, SearchBuilder};
use std::fmt::Display;
use std::time::Instant;
use unicode_categories::UnicodeCategories;
use walkdir::WalkDir as WalkSimple;

fn main() {
    //test_walkdir();
    //test_ignore();
    //check_filenames();

    //test_glob_explore_and_eliminate_unwanted();
    //test_rust_search();
    test_globmatch();
}

fn test_globmatch() {
    let builder = globmatch::Builder::new(r"**/cargo.toml")
        .case_sensitive(false)
        .build(r"C:\Development")
        .unwrap();

    // for p in builder
    //     .into_iter()
    //     // Doesn't exclude files in $RECYCLE.BIN (can filter on path), but since it's done after getting iterator, it's too late
    //     .filter_entry(|p| !globmatch::is_hidden_entry(p))
    // {
    //     // match p {
    //     //     Ok(path) => println!("{}", path.display()),
    //     //     Err(_) => {}
    //     // }
    //     if let Ok(path) = p { println!("{}", path.display()); }
    // }

    // Since a Result<T,E> is iterable on Ok values and we only care about these, we can use flatten
    for p in builder
        .into_iter()
        // Doesn't exclude files in $RECYCLE.BIN (can filter on path), but since it's done after getting iterator, it's too late
        .filter_entry(|p| !globmatch::is_hidden_entry(p))
        .flatten()
    {
        println!("{}", p.display());
    }
}

fn test_rust_search() {
    let fi: FilterFn = |dir| dir.metadata().unwrap().is_file();

    let search: Vec<String> = SearchBuilder::default()
        .location(r"C:\Development")
        .search_input("cargo.toml")
        //.search_input("cargo")
        .ignore_case()
        //.filter(rust_search::filter::FilterType::Custom(fi))          // Nothing works here
        .build()
        .collect();

    for path in search {
        println!("{}", path);
    }
}

fn test_glob_explore_and_eliminate_unwanted() {
    let pattern = r"C:\Development\**\cargo.toml";

    let mo = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    match glob_with(pattern, mo) {
        Ok(paths) => {
            for entry in paths {
                match entry {
                    Ok(pb) => {
                        if is_ignored(&pb) {
                            println!(">>> {}", pb.display());
                        } else {
                            println!("{}", pb.display());
                        }
                    }
                    Err(err) => {
                        println!("*** Entry error {}", err);
                    }
                };
            }
        }
        Err(err) => {
            println!("*** Pattern error {}", err);
        }
    }
}

fn is_ignored(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt; // Import Windows-specific metadata
    if let Ok(metadata) = fs::metadata(path) {
        let attributes = metadata.file_attributes();

        (attributes & 0x2) != 0 || // Hidden attribute
            path.as_os_str().to_str().unwrap().contains("$RECYCLE.BIN")
        //path.file_name().and_then(|s| s.to_str()).unwrap().contains("$RECYCLE.BIN") // recycle bin
    } else {
        false
    }
}

fn check_filenames() {
    let mut cnt = 0;
    let start = Instant::now();

    // cnt += check(r"C:\Development\GitVSTS");
    // cnt += check(r"C:\Development\GitHub");
    // cnt += check_ignore(r"C:\Development");
    // cnt += check_simple(r"\\terazalt\books\Livres");
    // cnt += check_simple(r"\\terazalt\books\BD");
    // cnt += check_simple(r"\\terazalt\books");
    // cnt += check_simple(r"\\terazalt\Photo");
    // cnt += check_simple(r"D:\Pierre\OneDrive\MusicOD");
    cnt += check_simple(r"D:\Pierre\OneDrive\DocumentsOD");

    let duration = start.elapsed();
    println!("\n{} files checked in {:.3}s", cnt, duration.as_secs_f64());
}

// CHeck for non-standard characters in development paths
fn check_ignore(path: &str) -> i32 {
    let mut cnt = 0;
    for result in WalkIgnore::new(path) {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => {
                let ft = entry.file_type().unwrap(); // ignore file_type() returns Option<FileType>
                if ft.is_dir() || ft.is_file() {
                    let filename = entry.file_name().to_string_lossy();
                    check_unicode(&filename, entry.path().display());
                    cnt += 1;
                } else if ft.is_symlink() {
                    //println!("SymLink: {}", entry.path().display());
                }
            }
            Err(_e) => {
                //println!("Err: {}", _e);
            }
        }
    }

    cnt
}

fn check_simple(path: &str) -> i32 {
    let mut cnt = 0;
    for result in WalkSimple::new(path) {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => {
                let ft = entry.file_type();
                if ft.is_dir() || ft.is_file() {
                    let filename = entry.file_name().to_string_lossy();
                    check_unicode(&filename, entry.path().display());
                    cnt += 1;
                } else if ft.is_symlink() {
                    //println!("SymLink: {}", entry.path().display());
                }
            }
            Err(_e) => {
                //println!("Err: {}", _e);
            }
        }
    }

    cnt
}

fn check_unicode(path: &str, fp: impl Display) {
    for c in path.chars() {
        if !(c.is_alphanumeric() || (32..127).contains(&(c as i32)) || (160..256).contains(&(c as i32)) || "€®™©–—…×·•∶⧹⧸⚹†‽¿🎜🎝".contains(c))
        {
            println!("{} -> char {:04X} {}", fp, c as i32, c);
        }
    }
}

fn test_ignore() {
    for result in WalkIgnore::new(r"C:\Development\GitVSTS") {
        // Each item yielded by the iterator is either a directory entry or an
        // error, so either print the path or the error.
        match result {
            Ok(entry) => {
                let ft = entry.file_type().unwrap(); // file_type() returns Option<FileType>
                if ft.is_dir() {
                    //println!("Dir: {}", entry.path().display());
                } else if ft.is_file() {
                    let filename = entry.file_name().to_string_lossy();
                    println!("File: {}", filename);
                    // if filename.ends_with(".cs") {
                    //     check_file(entry.path());
                    //     //println!("File: {}", entry.path().display());
                    // }
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

    for result in WalkSimple::new(r"C:\Development") {
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
