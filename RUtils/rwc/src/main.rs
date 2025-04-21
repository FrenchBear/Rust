// rwc: Rust version of wc
//
// 2025-04-21	PV      First version

// standard library imports
use std::io;
use std::path::Path;
use std::process;
use std::time::Instant;

// external crates imports
use myglob::{MyGlobMatch, MyGlobSearch};

// -----------------------------------
// Submodules

mod decode_encoding;
mod options;
pub mod tests;

use decode_encoding::*;
use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = "rwc";
const APP_VERSION: &str = "1.0.0";

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

                        // We ignore matching directories in rgrep, we only look for files
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
        process_text(&mut b, s.as_str(), "(stdin)", &options);
    }
    let duration = start.elapsed();

    if b.files_count > 1 || options.show_only_total {
        print_line(b.lines_count, b.words_count, b.chars_count, b.bytes_count, "total");
    }

    if options.verbose {
        println!("{} files(s) searched in {:.3}s", b.files_count, duration.as_secs_f64());
    }
}

fn print_line(lines_count: usize, words_count: usize, chars_count: usize, _bytes_count: usize, filename: &str) {
    println!("{:6} {:6} {:6} {}", lines_count, words_count, chars_count, filename);
}

/// First step processing a file, read text content from path and call process_text.
fn process_file(b: &mut DataBag, path: &Path, options: &Options) {
    let res = read_text_file(path);
    match res {
        Ok((Some(s), _)) => {
            let filename = path.display().to_string();
            process_text(b, s.as_str(), filename.as_str(), options);
        }
        Ok((None, _)) => {
            // Non-text files are ignored
            if options.verbose {
                println!("{APP_NAME}: ignored non-text file {}", path.display());
            }
        }
        Err(e) => {
            eprintln!("*** Error reading file {}: {}", path.display(), e);
        }
    }
}

/// Core rgrep process, search for re in txt, read from filename, according to options.
fn process_text(b: &mut DataBag, txt: &str, filename: &str, options: &Options) {
    let mut lines = 0;
    let mut words = 0;
    let chars = txt.chars().count();
    let bytes = txt.len();

    for line in txt.lines() {
        lines += 1;
        for word in line.trim().split([' ', '\t']) {
            if !word.is_empty() {
                words += 1;
            }
        }
    }

    if !options.show_only_total {
        print_line(lines, words, chars, bytes, filename);
    }

    b.files_count += 1;
    b.lines_count += lines;
    b.words_count += words;
    b.chars_count += chars;
    b.bytes_count += bytes;
}
