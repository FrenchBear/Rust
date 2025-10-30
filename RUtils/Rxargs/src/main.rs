// rxargs: Rust version of wc
//
// 2025-10-30	PV      First version

#![allow(unused)]

// Standard library imports
use std::io::BufRead;
use std::process;
use std::time::Instant;
use std::{io, path::Path};

// External crates imports
use textautodecode::{TextAutoDecode, TextFileEncoding};

// -----------------------------------
// Submodules

mod command_to_run;
mod options;
pub mod tests;

use command_to_run::*;
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
    line_count: usize,
    lines: Vec<String>,
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

    // If option -1, just accumulate args, otherwise prepare and run command
    // For now, just accumulate args with option -1 ang execute at the end, but maybe later when option -s max_chars is implemented,
    // execute command as soon as args buffer is full without waiting for the end

    if let Some(ref f) = options.input_file {
        process_file(Path::new(f), &options, &mut b);
    } else {
        // If no source has been provided, use stdin

        if options.verbose {
            println!("Reading from stdin");
        }
        loop {
            let line = io::stdin().lines().next();
            if line.is_none() {
                break;
            }
            match line.unwrap() {
                Ok(s) => {
                    process_line(&s, &options, &mut b);
                }
                Err(e) => {
                    if options.verbose {
                        eprintln!("*** Error reading from stdin: {}", e);
                    }
                }
            }
        }
    }

    let duration = start.elapsed();

    if options.verbose {
        println!("{} lines(s) processed in {:.3}s", b.line_count, duration.as_secs_f64());
    }
}

fn process_file(path: &Path, options: &Options, b: &mut DataBag) -> io::Result<()> {
    if options.verbose {
        println!("Reading arguments from file {}", path.display());
    }

    // Delegate text file decoding to TextAutoDecode for simplicity
    let tad = TextAutoDecode::read_text_file(path)?;
    if tad.encoding == TextFileEncoding::NotText {
        return Err(io::Error::other(format!("{APP_NAME}: {} is not a text file", path.display())));
    }

    for line in tad.text.unwrap().lines() {
        process_line(line, options, b);
    }

    Ok(())
}

fn process_line(line: &str, options: &Options, b: &mut DataBag) {
    // By convention, we skip empty lines
    if line.is_empty() {
        return;
    }

    b.line_count += 1;

    if options.group_args {
        b.lines.push(line.into());
        return;
    }

    let ql = quoted_string(line);
    let pp = Path::new(&ql);
    match options.ctr.exec1(pp, false) {
        Ok(s) => {
            if options.verbose {
                println!("{}", s);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
