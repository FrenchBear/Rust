// rcat: Rust version of cat
//
// 2025-10-24	PV      First version
// 2025-10-31	PV      1.0.1 fn s(n)

//#![allow(unused)]

// Standard library imports
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;
use std::time::Instant;

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
        let pb = Path::new(source);
        process_file(&mut b, pb, &options);
    }

    // If no source has been provided, use stdin
    if options.sources.is_empty() {
        if options.verbose {
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
    }

    let duration = start.elapsed();

    if b.files_count > 1 {
        let mut name = String::from("total");
        if b.files_count > 1 {
            name += format!(" ({} files)", b.files_count).as_str();
        }
    }

    if options.verbose {
        println!("{} file{} searched in {:.3}s", b.files_count, s(b.files_count), duration.as_secs_f64());
    }
}

// Helper for plurals
fn s(n: usize) -> &'static str {
    if n > 1 { "s" } else { "" }
}

/// First step processing a file, read text content from path and call process_text.
fn process_file(b: &mut DataBag, path: &Path, options: &Options) {
    match std::fs::read(path) {
        Ok(bytes) => {
            if let Err(e) = io::stdout().write_all(&bytes) {
                if options.verbose {
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
