// rdir: Show detailed file info
//
// 2025-10-24	PV      First version

#![allow(unused)]

// Standard library imports
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::time::Instant;

// External imports
use num_format::{Locale, ToFormattedString};

// -----------------------------------
// Submodules

mod fa_size;
mod fa_dates;
mod fa_attributes;
mod fa_reparsepoints;
mod fa_hardlinks;
mod fa_streams;
mod options;

use fa_size::*;
use fa_dates::*;
use fa_attributes::*;
use fa_reparsepoints::*;
use fa_hardlinks::*;
use fa_streams::*;
use options::*;

// -----------------------------------
// Tests

pub mod tests;

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
        if pb.is_file() {
            process_file(&mut b, &pb, &options);
        } else if pb.is_dir() {
            process_directory(&mut b, &pb, &options);
        } else if pb.is_symlink() {
            println!("{} is a symbolic link with invalid target", source);
        } else {
            println!("{}: Not found", source);
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
        println!("{} files(s) analyzed in {:.3}s", b.files_count, duration.as_secs_f64());
    }
}

fn process_directory(b: &mut DataBag, path: &Path, options: &Options) {
    println!("Process directory {}: ToDo", path.display());
}

fn process_file(b: &mut DataBag, path: &Path, options: &Options) {
    b.files_count += 1;

    println!("\n-----------------");
    println!("File: {}", path.display());
    let original_path = path.to_string_lossy().replace(r"\\?\", "");
    let absolute_path = path.canonicalize().unwrap().to_string_lossy().replace(r"\\?\", "");
    if absolute_path != original_path {
        println!("Absolute path: {}", absolute_path); // For links, get target...
    }

    match get_size_information(path, &options) {
        Ok(s) => {
            let size_formatted = s.size.to_formatted_string(&Locale::fr); // Use French locale for now. Later we will find the user locale.
            println!("Size: {}B", size_formatted);
        }
        Err(e) => println!("Error analyzing size info: {}", e),
    }

    match get_dates_information(path, &options) {
        Ok(d) => {
            println!(
                "Dates:  Creation: {}  Modification: {}  Access: {}",
                d.created_local.format("%d/%m/%Y %H:%M:%S"),
                d.modified_local.format("%d/%m/%Y %H:%M:%S"),
                d.accessed_local.format("%d/%m/%Y %H:%M:%S")
            );
        }
        Err(e) => println!("Error analyzing dates info: {}", e),
    }

    match get_attributes_information(path, &options) {
        Ok(ai) => {
            print!("Attributes: ");
            if ai.archive {
                print!("archive ");
            }
            if ai.readonly {
                print!("readonly ");
            }
            if ai.hidden {
                print!("hidden ");
            }
            if ai.system {
                print!("system ");
            }
            if ai.directory {
                print!("directory ");
            }
            if ai.tempoary {
                print!("tempoary ");
            }
            if ai.sparse_file {
                print!("sparse_file ");
            }
            if ai.reparse_point {
                print!("reparse_point ");
            }
            if ai.compressed {
                print!("compressed ");
            }
            if ai.offline {
                print!("offline ");
            }
            if ai.not_content_indexed {
                print!("not_content_indexed ");
            }
            if ai.encrypted {
                print!("encrypted ");
            }
            if ai.integrity_stream {
                print!("integrity_stream ");
            }
            if ai.isvirtual {
                print!("isvirtual ");
            }
            if ai.no_scrub_data {
                print!("no_scrub_data ");
            }
            if ai.pinned {
                print!("pinned ");
            }
            if ai.unpinned {
                print!("unpinned ");
            }
            if ai.recall_on_open {
                print!("recall_on_open ");
            }
            if ai.recall_on_data_access {
                print!("recall_on_data_access (STUB)");
            }
            println!()
        }
        Err(e) => println!("Error analyzing attributes info: {}", e),
    }

    match get_reparsepoints_information(path, &options) {
        Ok(r) => {
            if r.kind != ReparseType::NO_REPARSE {
                println!("Reparse point: {:#?}: {}", r.kind, r.detail);
            }
        }
        Err(e) => println!("Error analyzing reparse info: {}", e),
    }

        match get_hardlinks_information(path, &options) {
        Ok(h) => {
            if h.hardlinks_count > 1 {
                println!("Hard links count: {}", h.hardlinks_count);
            }
        }
        Err(e) => println!("Error analyzing reparse info: {}", e),
    }

    match get_streams_information(path, &options) {
        Ok(s) => {
            if !s.streams.is_empty() {
                println!("Alternate Data Streams:");
                for stream in s.streams {
                    let size_formatted = stream.size.to_formatted_string(&Locale::fr);
                    println!("  {} ({}B)", stream.name, size_formatted);
                }
            }
        }
        Err(e) => println!("Error analyzing streams info: {}", e),
    }
}
