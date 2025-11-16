// rcat: Rust version of cat
//
// 2025-10-24	PV      First version
// 2025-10-31	PV      1.0.1 fn s(n)
// 2025-11-16	PV      1.1   Use MyGlob
// 2025-11-16	PV      2.0   Use getopts instead of getopt to parse options; Use MyGlobCLOptions to process MyGlob options

// ToDo: implement a set of standard options to control glob library, not limited to a+/a-
// ToDo: option to limit to text inputs and control output text encoding
// ToDo: linuw allows - to represent stdin, as in "cat f - g": output f's contents, then standard input, then g's contents

#![allow(unused)]

// Standard library imports
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;
use std::time::Instant;

// External crates imports
use myglob::{MyGlobCLOptions, MyGlobMatch, MyGlobSearch};

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
}

fn main() {
    // Process options
    let options = AppOptions::new().unwrap_or_else(|err| {
        let msg = format!("{}", err);
        if msg.is_empty() {
            process::exit(0);
        }
        eprintln!("{APP_NAME}: Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("options: {:#?}\n", options);

    let start = Instant::now();

    // Convert String sources into MyGlobSearch structs
    let mut sources: Vec<(&String, MyGlobSearch)> = Vec::new();
    let mut err_found = false;
    for source in options.sources.iter() {
        let mut builder = MyGlobSearch::new(source).apply_command_line_options(&options.mgclo);
        let resgs = builder.compile();
        match resgs {
            Ok(gs) => {
                if options.debug {
                    println!("dbg: {} -> {:?}", source, gs.segments);
                }
                sources.push((source, gs));
            }
            Err(e) => {
                err_found = true;
                println!("*** Error building MyGlob: {:?}", e);
            }
        }
    }

    // No valid input, just exit, don't process stdin
    if err_found && sources.is_empty() {
        process::exit(1);
    }

    let mut b = DataBag { ..Default::default() };

    // If no source has been provided, use stdin
    if options.sources.is_empty() {
        if options.verbose > 0 {
            // Use eprintln for verbose messages to not mix with stdout content
            eprintln!("Reading from stdin");
        }

        let mut buffer = Vec::new();
        if let Err(e) = io::stdin().read_to_end(&mut buffer) {
            eprintln!("{APP_NAME}: error reading from stdin: {}", e);
            process::exit(1);
        }

        if let Err(e) = io::stdout().write_all(&buffer) {
            // Don't check for verbose here, as this error is critical.
            eprintln!("{APP_NAME}: error writing to stdout: {}", e);
            // Exit since we can't write to stdout anymore.
            process::exit(1);
        }
    } else {
        for gs in sources.iter() {
            for ma in gs.1.explore_iter() {
                match ma {
                    MyGlobMatch::File(pb) => {
                        process_file(&mut b, &pb, &options);
                    }

                    MyGlobMatch::Dir(_) => {}

                    MyGlobMatch::Error(err) => {
                        if options.verbose > 0 {
                            println!("{APP_NAME}: MyGlobMatch error {}", err);
                        }
                    }
                }
            }
        }
    }

    let duration = start.elapsed();

    if b.files_count > 1 {
        let mut name = String::from("total");
        if b.files_count > 1 {
            name += format!(" ({} files)", b.files_count).as_str();
        }
    }

    if options.verbose > 0 {
        println!("{} file{} searched in {:.3}s", b.files_count, s(b.files_count), duration.as_secs_f64());
    }
}

// Helper for plurals
fn s(n: usize) -> &'static str {
    if n > 1 { "s" } else { "" }
}

/// First step processing a file, read text content from path and call process_text.
fn process_file(b: &mut DataBag, path: &Path, options: &AppOptions) {
    match std::fs::read(path) {
        Ok(bytes) => {
            if let Err(e) = io::stdout().write_all(&bytes) {
                if options.verbose > 0 {
                    eprintln!("{APP_NAME}: error writing to stdout: {}", e);
                }
                // Exit since we can't write to stdout anymore
                process::exit(1);
            }
            b.files_count += 1;
        }
        Err(e) => {
            eprintln!("{APP_NAME}: error reading file {}: {}", path.display(), e);
        }
    }
}
