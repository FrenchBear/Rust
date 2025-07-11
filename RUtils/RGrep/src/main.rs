// rgrep: Basic grep project in Rust
//
// 2025-03-13	PV      First version
// 2025-03-16	PV      1.0.1   Extended help, support reading from stdin
// 2025-03-25	PV      1.1.0   Global constants; Ignore $RECYCLE.BIN
// 2025-03-27   PV      1.2.0   Option -2 to use MyGlob crate (experimental)
// 2025-03-28   PV      1.2.1   Option -1 to use glob crate, glob syntax documented in extended help
// 2025-03-29   PV      1.2.2   Option -2 is now default; Rename rgrep
// 2025-04-01   PV      1.3.0   read_text_file_2, faster to detect text encoding
// 2025-04-08   PV      1.4.0   When stdout is redirected, don't use colors (atty crate)
// 2025-04-18   PV      1.4.1   Only check help and ? on first position in command line; more extended help
// 2025-04-18   PV      1.5.0   End of glob crate and options -1/-2
// 2025-05-02   PV      1.6.0   Use crate textautodecode instead of decode_encoding module
// 2025-05-04   PV      1.7.0   Do not crash with patterns as [^abc]. Created Options module. Use MyMarkup for extended help formatting.
// 2025-07-10   PV      1.7.1   Get information from Cargo.toml, and use build script build.rs

//#![allow(unused)]

// Standard library imports
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

// External crates imports
use colored::*;
use getopt::Opt;
use myglob::{MyGlobMatch, MyGlobSearch};
use regex::Regex;
use textautodecode::{TextAutoDecode, TextFileEncoding};

// -----------------------------------
// Submodules

mod options;
mod grepiterator;
mod tests;

use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// -----------------------------------
// Main

fn main() {
    // Process options
    let mut options = Options::new().unwrap_or_else(|err| {
        let msg = format!("{}", err);
        if msg.is_empty() {
            process::exit(0);
        }
        eprintln!("{APP_NAME}: Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let re = build_re(&options);
    if re.is_err() {
        eprintln!("{APP_NAME}: Problem with search pattern: {:?}", re.err().unwrap());
        process::exit(1);
    }
    let re = re.unwrap();

    let start = Instant::now();

    // Building list of files
    // ToDo: It could be better to process file just when it's returned by iterator rather than stored in a Vec and processed later...
    let mut files: Vec<PathBuf> = Vec::new();
    for source in options.sources.iter() {
        let mut count = 0;

        let resgs = MyGlobSearch::new(source).autorecurse(options.autorecurse).compile();
        match resgs {
            Ok(gs) => {
                for ma in gs.explore_iter() {
                    match ma {
                        MyGlobMatch::File(pb) => {
                            count += 1;
                            files.push(pb);
                        }

                        // We ignore matching directories in rgrep, we only look for files
                        MyGlobMatch::Dir(_) => {}

                        MyGlobMatch::Error(err) => {
                            if options.verbose > 0 {
                                eprintln!("{APP_NAME}: error {}", err);
                            }
                        }
                    }
                }
            }

            Err(e) => {
                eprintln!("{APP_NAME}: Error building MyGlob: {:?}", e);
                count = -1; // No need to display "no file found" in this case
            }
        }

        if count == 0 {
            println!("{APP_NAME}: no file found matching {}", source);
        }
    }

    // Finally processing files, if more than 1 file, prefix output with file
    if options.sources.is_empty() {
        if options.verbose > 0 {
            println!("Reading from stdin");
        }
        let s = io::read_to_string(io::stdin()).unwrap();
        process_text(&re, s.as_str(), "(stdin)", &options);
    } else {
        if files.len() > 1 {
            options.show_path = true;
        }
        for pb in &files {
            if options.verbose > 1 {
                println!("Process: {}", pb.display());
            }
            process_path(&re, pb, &options);
        }
    }
    let duration = start.elapsed();

    if options.verbose > 0 {
        if files.is_empty() {
            print!("\nstdin");
        } else {
            print!("\n{} file", files.len());
            if files.len() > 1 {
                print!("s");
            }
        }
        println!(" searched in {:.3}s", duration.as_secs_f64());
    }
}

/// Helper, build Regex according to options (case, fixed string, whole word).<br/>
/// Return an error in case of invalid Regex.
pub fn build_re(options: &Options) -> Result<Regex, regex::Error> {
    let spat = if options.fixed_string {
        regex::escape(options.pattern.as_str())
    } else if options.whole_word {
        format!("\\b{}\\b", options.pattern)
    } else {
        options.pattern.clone()
    };
    let spat = String::from(if options.ignore_case { "(?imR)" } else { "(?mR)" }) + spat.as_str();
    Regex::new(spat.as_str())
}

/// First step processing a file, read text content from path and call process_text.
fn process_path(re: &Regex, path: &Path, options: &Options) {
    let res = TextAutoDecode::read_text_file(path);
    match res {
        Ok(tad) => {
            if tad.encoding == TextFileEncoding::NotText {
                // Non-text files are ignored
                if options.verbose == 1 {
                    println!("{APP_NAME}: ignored non-text file {}", path.display());
                }
            } else {
                let filename = path.display().to_string();
                process_text(re, tad.text.unwrap().as_str(), filename.as_str(), options);
            }
        }
        Err(e) => {
            eprintln!("*** Error reading file {}: {}", path.display(), e);
        }
    }
}

/// Core rgrep process, search for re in txt, read from filename, according to options.
fn process_text(re: &Regex, txt: &str, filename: &str, options: &Options) {
    let mut matchlinecount = 0;

    // Note that this test is actually useless since colored doesn't emit ANSI sequences when stdout is not a tty
    if atty::is(atty::Stream::Stdout) {
        for gi in grepiterator::GrepLineMatches::new(txt, re) {
            matchlinecount += 1;

            if options.out_level == 1 {
                println!("{}", filename);
                return;
            }

            if options.out_level == 0 {
                if options.show_path {
                    print!("{}: ", filename.bright_black());
                }

                let mut p: usize = 0;
                for ma in gi.ranges {
                    if ma.start < gi.line.len() {
                        let e = ma.end;
                        print!("{}{}", &gi.line[p..ma.start], &gi.line[ma].red().bold());
                        p = e;
                    }
                }
                println!("{}", &gi.line[p..]);
            }
        }
    } else {
        for gi in grepiterator::GrepLineMatches::new(txt, re) {
            matchlinecount += 1;

            if options.out_level == 1 {
                println!("{}", filename);
                return;
            }

            if options.out_level == 0 {
                if options.show_path {
                    print!("{}: ", filename);
                }
                println!("{}", gi.line);
            }
        }
    }

    // Note: both options -c and -l (out_level==3) is not supported by Linux version
    if options.out_level == 2 || (options.out_level == 3 && matchlinecount > 0) {
        println!("{}:{}", filename, matchlinecount);
    }
}
