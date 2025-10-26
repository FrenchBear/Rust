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
use numfmt::{Formatter, Precision, Scales};

// -----------------------------------
// Submodules

mod fa_attributes;
mod fa_dates;
mod fa_hardlinks;
mod fa_names;
mod fa_reparsepoints;
mod fa_size;
mod fa_streams;
mod options;

use fa_attributes::*;
use fa_dates::*;
use fa_hardlinks::*;
use fa_names::*;
use fa_reparsepoints::*;
use fa_size::*;
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

// fn main() {
//     // Create a small test file
//     let path = Path::new(r"S:\MultiLinks\inexistent_target.txt");
//     println!("Path: {}", path.display());
//     let filename = path.file_name().unwrap().to_str().unwrap().to_string();
//     println!("Filename: {}", filename);
//     let ext = path.extension().unwrap().to_str().unwrap().to_string();
//     println!("Extension: {}", ext);

//     let can = canonicalize_link(path).unwrap();
//     println!("Canonical path: {}", can.display());

//     let original_with_path = path.to_string_lossy().replace(r"\\?\", "");
//     println!("Original with path: {}", original_with_path);
//     let canonical_fullpath = path.canonicalize().unwrap().to_string_lossy().replace(r"\\?\", "");
//     println!("Canonical fullpath: {}", canonical_fullpath);
// }

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
            process_path(&mut b, &pb, &options);
        } else if pb.is_dir() {
            process_path(&mut b, &pb, &options);
        } else if pb.is_symlink() {
            //println!("{} is a symbolic link with invalid target", source);
            process_path(&mut b, &pb, &options);
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

fn process_path(b: &mut DataBag, path: &Path, options: &Options) {
    b.files_count += 1;

    println!("\n-----------------");
    print!("Path: {}", path.display());

    let (kind, kind2) = if path.is_file() {
        if path.is_symlink() { ("File symbolic link", "Link") } else { ("File", "File") }
    } else if path.is_dir() {
        if path.is_symlink() { ("Directory symbolic link", "Link") } else { ("Directory", "Directory") }
    } else if path.is_symlink() {
        ("Symbolic link (inxistent target)", "Link")
    } else {
        ("Unknown", "Unknown")
    };
    println!("  [{}]", kind);

    match get_names_information(path, &options) {
        Ok(n) => {
            println!("{kind2} name: {}", show_invisible_chars(n.filename.as_str()));
            println!("Parent: {}", show_invisible_chars(n.parent.as_str()));
            if n.original_with_path != n.canonical_fullpath {
                println!("Canonical path: {}", show_invisible_chars(n.canonical_fullpath.as_str())); // For links, get target...
            }
            let mut pr = false;
            if let Some(typ) = n.file_type_description {
                print!("File type: {}", typ);
                pr = true;
            }
            if let Some(app) = n.opens_with {
                print!("  Opens with: {}", app);
                pr = true;
            }
            if pr {
                println!();
            }
        }
        Err(e) => println!("Error analyzing names info: {}", e),
    }

    match get_size_information(path, &options) {
        Ok(si) => {
            if path.is_file() {
                let size = get_formatted_size(si.size);
                print!("Apparent size: {}", size);

                let size_on_disk = get_formatted_size(si.size_on_disk);
                println!("   Size on disk: {}", size_on_disk);
            } else {
                if si.dir_filescount + si.dir_dirscount + si.dir_linkscount == 0 {
                    print!("Empty directory");
                } else {
                    print!("Dir counts: ");
                    if si.dir_filescount > 0 {
                        print!("{} file{} ", si.dir_filescount, s(si.dir_filescount));
                    }
                    if si.dir_dirscount > 0 {
                        if si.dir_dirscount == 1 {
                            print!("{} directory ", si.dir_dirscount);
                        } else {
                            print!("{} directories ", si.dir_dirscount);
                        }
                    }
                    if si.dir_linkscount > 0 {
                        print!("{} link{} ", si.dir_linkscount, s(si.dir_linkscount));
                    }
                    println!();
                }
            }
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
            if ai.normal {
                print!("normal (no attributes) ");
            }
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
        Err(e) => println!("Error analyzing hardlinks info: {}", e),
    }

    match get_streams_information(path, &options) {
        Ok(s) => {
            if !s.streams.is_empty() {
                println!("Alternate Data Streams:");
                for stream in s.streams {
                    let size = get_formatted_size(stream.size);
                    println!("  {}  [{}]", stream.name, size);
                }
            }
        }
        Err(e) => println!("Error analyzing streams info: {}", e),
    }
}

fn s(n: i32) -> &'static str {
    if n > 1 { "s" } else { "" }
}

fn show_invisible_chars(s: &str) -> String {
    let s = format!("{:?}", s).replace(r"\\", r"\");
    strip_quotes(&s).to_string()
}

/// Removes the surrounding double quotes from a string slice, if they exist.
/// If the string starts and ends with a `"` character, a slice without those
/// characters is returned. Otherwise, the original string slice is returned.
fn strip_quotes(s: &str) -> &str {
    s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).unwrap_or(s)
}

fn get_formatted_size(size: u64) -> String {
    // numfmt formatter
    let mut fmt_bytes = Formatter::new()
        .scales(Scales::none())
        .separator(' ')
        .unwrap()
        .precision(Precision::Decimals(0))
        .suffix("\u{00A0}B")
        .unwrap();
    let mut res = fmt_bytes.fmt2(size).replace(' ', "\u{00A0}");

    if size >= 1024 {
        let mut fmt_scaled = Formatter::new()
            .scales(Scales::binary())
            .separator(' ')
            .unwrap()
            .precision(Precision::Significance(3))
            .suffix("B")
            .unwrap();
        let size_scaled = fmt_scaled.fmt2(size).replace(' ', "\u{00A0}");
        res = res + format!(" ({})", size_scaled).as_str();
    }
    res
}
