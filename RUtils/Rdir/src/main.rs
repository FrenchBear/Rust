// rdir: Show detailed file info
//
// 2025-10-24	PV      First version
// 2025-10-24	PV      1.0.1 Cur treams names at first \0; process prefix \\?\UNC\ correctly

#![allow(unused)]

// Standard library imports
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::time::Instant;

// External imports
use myglob::{MyGlobMatch, MyGlobSearch};
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
    files_count: u32,
    dirs_count: u32,
    links_count: u32,
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

    // Convert String sources into MyGlobSearch structs
    let mut sources: Vec<(&String, MyGlobSearch)> = Vec::new();
    for source in options.sources.iter() {
        let resgs = MyGlobSearch::new(source).autorecurse(options.autorecurse).compile();
        match resgs {
            Ok(gs) => sources.push((source, gs)),

            Err(e) => {
                eprintln!("*** Error building MyGlob: {:?}", e);
                process::exit(1);
            }
        }
    }
    if sources.is_empty() {
        eprintln!("*** No source specified. Use {APP_NAME} ? to show usage.");
        process::exit(1);
    }

    let start = Instant::now();

    let mut b = DataBag { ..Default::default() };

    for gs in sources.iter() {
        for ma in gs.1.explore_iter() {
            match ma {
                MyGlobMatch::File(pb) => process_path(&mut b, &pb, &options),

                MyGlobMatch::Dir(pb) => process_path(&mut b, &pb, &options),

                MyGlobMatch::Error(err) => {
                    if options.verbose {
                        eprintln!("{APP_NAME}: MyGlobMatch error {}", err);
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

    if options.verbose {
        let mut msg = Vec::<String>::new();
        if b.files_count > 0 {
            msg.push(format!("{} file{}", b.files_count, s(b.files_count)));
        }
        if b.dirs_count > 0 {
            msg.push(format!("{} {}", b.dirs_count, if b.dirs_count > 1 { "directories" } else { "directory" }));
        };
        if b.links_count > 0 {
            msg.push(format!("{} link{}", b.links_count, s(b.links_count)));
        }
        if msg.is_empty() {
            msg.push("No file, no directory".to_string());
        }

        println!("{} analyzed in {:.3}s", msg.join(", "), duration.as_secs_f64());
    }
}

fn process_path(b: &mut DataBag, path: &Path, options: &Options) {
    if path.is_symlink() {
        b.links_count += 1;
    } else if path.is_dir() {
        b.dirs_count += 1;
    } else if path.is_file() {
        b.files_count += 1;
    } else {
        if options.verbose {
            eprintln!("{}: Unknown type", path.display());
        }
        return;
    }

    print!("Path:           {}", path.display());

    let (kind, kind2) = if path.is_file() {
        if path.is_symlink() {
            ("File symbolic link", "Link")
        } else {
            ("File", "File")
        }
    } else if path.is_dir() {
        if path.is_symlink() {
            ("Directory symbolic link", "Link")
        } else {
            ("Directory", "Directory")
        }
    } else if path.is_symlink() {
        ("Symbolic link (inxistent target)", "Link")
    } else {
        ("Unknown", "Unknown")
    };
    println!("  [{}]", kind);

    match get_names_information(path, options) {
        Ok(n) => {
            let label = format!("{kind2} name:");
            println!("{label:<15} {}", show_invisible_chars(n.filename.as_str()));
            println!("Parent:         {}", show_invisible_chars(n.parent.as_str()));
            if n.original_with_path != n.canonical_fullpath {
                println!("Canonical path: {}", show_invisible_chars(n.canonical_fullpath.as_str())); // For links, get target...
            }
            let mut pr = false;
            if let Some(typ) = n.file_type_description {
                print!("File type:      {}", typ);
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
        Err(e) => eprintln!("*** Error analyzing names info: {}", e),
    }

    match get_size_information(path, options) {
        Ok(si) => {
            if path.is_symlink() && !path.exists() {
                // Do nothing
            } else if path.is_file() {
                let size = get_formatted_size(si.size);
                print!("Apparent size:  {}", size);

                let size_on_disk = get_formatted_size(si.size_on_disk);
                println!("   Size on disk: {}", size_on_disk);
            } else if si.dir_filescount + si.dir_dirscount + si.dir_linkscount == 0 {
                if !path.is_symlink() || options.show_link_target_info {
                    println!("Dir counts:     Empty directory");
                }
            } else {
                print!("Dir counts:     ");
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
        Err(e) => eprintln!("*** Error analyzing size info: {}", e),
    }

    match get_dates_information(path, options) {
        Ok(d) => {
            println!(
                "Dates:          Creation: {}  Modification: {}  Access: {}",
                d.created_local.format("%d/%m/%Y %H:%M:%S"),
                d.modified_local.format("%d/%m/%Y %H:%M:%S"),
                d.accessed_local.format("%d/%m/%Y %H:%M:%S")
            );
        }
        Err(e) => eprintln!("*** Error analyzing dates info: {}", e),
    }

    match get_attributes_information(path, options) {
        Ok(ai) => {
            let mut at: Vec<&str> = Vec::new();
            if ai.normal {
                at.push("Normal (no attributes)");
            }
            if ai.archive {
                at.push("Archive");
            }
            if ai.readonly {
                at.push("Readonly");
            }
            if ai.hidden {
                at.push("Hidden");
            }
            if ai.system {
                at.push("System");
            }
            if ai.directory {
                at.push("Directory");
            }
            if ai.tempoary {
                at.push("Tempoary");
            }
            if ai.sparse_file {
                at.push("Sparse file");
            }
            if ai.reparse_point {
                at.push("Reparse point");
            }
            if ai.compressed {
                at.push("Compressed");
            }
            if ai.offline {
                at.push("Offline");
            }
            if ai.not_content_indexed {
                at.push("Not content indexed");
            }
            if ai.encrypted {
                at.push("Encrypted");
            }
            if ai.integrity_stream {
                at.push("Integrity stream");
            }
            if ai.isvirtual {
                at.push("IsVirtual");
            }
            if ai.no_scrub_data {
                at.push("No scrub data");
            }
            if ai.pinned {
                at.push("Pinned");
            }
            if ai.unpinned {
                at.push("Unpinned");
            }
            if ai.recall_on_open {
                at.push("Recall on open");
            }
            if ai.recall_on_data_access {
                at.push("Recall on data access (STUB)");
            }
            println!("Attributes:     {}", at.join(", "));
        }
        Err(e) => eprintln!("*** Error analyzing attributes info: {}", e),
    }

    match get_reparsepoints_information(path, options) {
        Ok(r) => {
            if r.kind != ReparseType::No_reparse {
                println!("Reparse point:  {:#?}: {}", r.kind, r.detail);
            }
        }
        Err(e) => eprintln!("*** Error analyzing reparse info: {}", e),
    }

    match get_hardlinks_information(path, options) {
        Ok(h) => {
            if h.hardlinks_count > 1 {
                println!("Hardlink count: {}", h.hardlinks_count);
            }
        }
        Err(e) => eprintln!("*** Error analyzing hardlinks info: {}", e),
    }

    match get_streams_information(path, options) {
        Ok(s) => {
            if !s.streams.is_empty() {
                print!("Alt Streams:    ");
                let mut line1: bool = true;
                for stream in s.streams {
                    let size = get_formatted_size(stream.size);
                    if !line1 {
                        print!("                ");
                    } else {
                        line1 = false;
                    }
                    println!("{}  [{}]", stream.name, size);
                }
            }
        }
        Err(e) => eprintln!("*** Error analyzing streams info: {}", e),
    }

    println!();
}

fn s(n: u32) -> &'static str {
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
        res += format!(" ({})", size_scaled).as_str();
    }
    res
}
