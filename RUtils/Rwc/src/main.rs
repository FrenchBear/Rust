// rwc: Rust version of wc
//
// 2025-04-21	PV      First version
// 2025-04-22	PV      1.1.0 Always show bytes; option -a+|- to control autorecurse
// 2025-05-02   PV      1.2.0 Use crate textautodecode instead of decode_encoding module. Also use file length instead of string bytes count to include BOM size
// 2025-05-04   PV      1.2.1 Use MyMarkup crate to format usage and extended help
// 2025-05-05   PV      1.2.2 Linux compatibility; Ignore files larger than 1GB
// 2025-07-10   PV      1.2.3 Get information from Cargo.toml, and use build script build.rs
// 2025-10-31   PV      1.2.4 fn s(n)

//#![allow(unused)]

// Standard library imports
use std::io;
use std::path::Path;
use std::process;
use std::time::Instant;

// External crates imports
use myglob::{MyGlobMatch, MyGlobSearch};
use textautodecode::{TextAutoDecode, TextFileEncoding};

// -----------------------------------
// Submodules

mod options;
pub mod tests;

use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// ==============================================================================================
// Main

#[derive(Debug, Default)]
struct DataBag {
    files_count: usize,
    lines_count: usize,
    words_count: usize,
    chars_count: usize,
    bytes_count: usize,
}

fn main() {
    // Process options
    let options = Options::new().unwrap_or_else(|err| {
        let msg = format!("{}", err);
        if msg.is_empty() {
            process::exit(0);
        }
        eprintln!("{APP_NAME}: Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let start = Instant::now();

    let mut b = DataBag { ..Default::default() };

    for source in options.sources.iter() {
        let resgs = MyGlobSearch::new(source).autorecurse(options.autorecurse).compile();
        match resgs {
            Ok(gs) => {
                for ma in gs.explore_iter() {
                    match ma {
                        MyGlobMatch::File(pb) => {
                            process_file(&mut b, &pb, &options);
                        }

                        // We ignore matching directories in rwc, we only look for files
                        MyGlobMatch::Dir(_) => {}

                        MyGlobMatch::Error(err) => {
                            if options.verbose {
                                eprintln!("{APP_NAME}: error {}", err);
                            }
                        }
                    }
                }
            }

            Err(e) => {
                eprintln!("{APP_NAME}: Error building MyGlob: {:?}", e);
            }
        }
    }

    // If no source has been provided, use stdin
    if options.sources.is_empty() {
        if options.verbose {
            println!("Reading from stdin");
        }
        let s = io::read_to_string(io::stdin()).unwrap();
        process_text(&mut b, s.as_str(), "(stdin)", &options, s.len());
    }
    let duration = start.elapsed();

    if b.files_count > 1 || options.show_only_total {
        let mut name = String::from("total");
        if b.files_count > 1 {
            name += format!(" ({} files)", b.files_count).as_str();
        }
        print_line(b.lines_count, b.words_count, b.chars_count, b.bytes_count, name.as_str());
    }

    if options.verbose {
        println!("{} file{} searched in {:.3}s", b.files_count, s(b.files_count), duration.as_secs_f64());
    }
}

fn s(n: usize) -> &'static str {
    if n > 1 { "s" } else { "" }
}

fn print_line(lines_count: usize, words_count: usize, chars_count: usize, bytes_count: usize, filename: &str) {
    println!("{:7} {:7} {:8} {:8}  {}", lines_count, words_count, chars_count, bytes_count, filename);
}

/// First step processing a file, read text content from path and call process_text.
fn process_file(b: &mut DataBag, path: &Path, options: &Options) {
    let res = TextAutoDecode::read_text_file(path);
    match res {
        Ok(tad) => {
            if tad.encoding == TextFileEncoding::NotText {
                // Non-text files are ignored
                if options.verbose {
                    println!("{APP_NAME}: ignored non-text file {}", path.display());
                }
            } else {
                let filesize = path.metadata().unwrap().len();
                // Anything above 1GB is ignored
                if filesize >= 1024u64 * 1024u64 * 1024u64 {
                    if options.verbose {
                        println!("{APP_NAME}: ignored very large file {}, size: {}", path.display(), filesize);
                    }
                } else {
                    let filename = path.display().to_string();
                    process_text(b, tad.text.unwrap().as_str(), filename.as_str(), options, filesize as usize);
                }
            }
        }
        Err(e) => {
            eprintln!("*** Error reading file {}: {}", path.display(), e);
        }
    }
}

/// Core rwc process, compute counts for a string
fn process_text(b: &mut DataBag, txt: &str, filename: &str, options: &Options, filesize: usize) {
    let mut lines = 0;
    let mut words = 0;
    let chars = txt.chars().count();

    for line in txt.lines() {
        lines += 1;
        // Don't want to use Unicode-aware split_whitespace() because of too many fancy spaces
        // split_ascii_whitespace() is Ok, it includes space, tab, LF, CR and FF, but just space and tab are enough
        for word in line.trim().split([' ', '\t']) {
            if !word.is_empty() {
                words += 1;
            }
        }
    }

    if !options.show_only_total {
        print_line(lines, words, chars, filesize, filename);
    }

    b.files_count += 1;
    b.lines_count += lines;
    b.words_count += words;
    b.chars_count += chars;
    b.bytes_count += filesize;
}
