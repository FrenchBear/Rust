// runiq: Rust version of uniq
//
// 2025-10-31	PV      First version

// Standard library imports
use std::io;
use std::process;
use std::time::Instant;

// External crates imports
use indexmap::IndexMap;

// -----------------------------------
// Submodules

mod options;
mod tests;

use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// ==============================================================================================
// Main

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

    // First, build a hashpap from stdin
    let lines = build_map(io::stdin().lines(), options.ignore_case);

    // Get final iterator
    let it = final_iterator(&lines, options.output);

    // Do output
    for item in it {
        println!("{}", item);
    }

    let duration = start.elapsed();

    if options.verbose {
        println!("Processed in {:.3}s", duration.as_secs_f64());
    }
}

fn final_iterator<'a>(lines: &'a IndexMap<String, Vec<String>>, output: Output) -> Box<dyn Iterator<Item = &'a String> + 'a> {
    match output {
        Output::Unique => {
            Box::new(lines.iter().map(|(_, v)| &v[0]))
        }
        Output::Repeated => {
            Box::new(lines.iter().filter(|(_, v)| v.len() > 1).map(|(_, v)| &v[0]))
        }
        Output::AllRepeated => {
            Box::new(
                lines.iter()
                    .filter(|(_, v)| v.len() > 1)
                    .flat_map(|(_, v)| v.iter()),
            )
        }
    }
}

fn build_map<I: IntoIterator<Item = Result<String, std::io::Error>>>(it: I, ignore_case: bool) -> IndexMap<String, Vec<String>> {
    let mut lines = IndexMap::<String, Vec<String>>::new();

    for item in it {
        match item {
            Ok(line) => {
                let key = if ignore_case { line.to_lowercase() } else { line.clone() };
                let entry = lines.entry(key).or_default();
                entry.push(line);
            }
            Err(e) => println!("*** Reading error: {}", e),
        }
    }
    lines
}
