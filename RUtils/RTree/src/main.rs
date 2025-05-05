// rtree
// Print visual directory structure in Rust
//
// 2025-05-05   PV      First version (from Gemini)

#![allow(unused)]

// Standard library imports
use std::fs;
use std::io;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

// External crates imports

// -----------------------------------
// Submodules

mod options;

use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = "rtree";
const APP_VERSION: &str = "1.0.0";

// ==============================================================================================
// Main

#[derive(Debug, Default)]
struct DataBag {
    dirs_count: usize,
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

    let mut b = DataBag { ..Default::default() };

    let start = Instant::now();
    match options.source {
        Some(start_dir) => {
            do_print(&mut b, &start_dir);
        }
        None => {
            do_print(&mut b, ".");
        }
    }
    let duration = start.elapsed();

    if options.verbose {
        println!("{} directories in {:.3}s", b.dirs_count, duration.as_secs_f64());
    }
}

// Moved to a separate function to use ? operator
fn do_print(b: &mut DataBag, source: &str) -> Result<(), io::Error> {
    let start_dir = &Path::new(source);
    if !start_dir.is_dir() {
        eprintln!("{APP_NAME}: '{}' is not a valid directory.", start_dir.display());
        return Ok(());
    }

    println!("{}", start_dir.to_string_lossy());
    let result = fs::read_dir(start_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<PathBuf>>();

    let num_subdirs = result.len();
    for (i, subdir) in result.iter().enumerate() {
        if !is_well_hidden(subdir) {
            print_tree(b, subdir, "", i == num_subdirs - 1)?;
        }
    }

    Ok(())
}

fn print_tree(b: &mut DataBag, dir: &Path, prefix: &str, is_last: bool) -> io::Result<()> {
    b.dirs_count += 1;
    let entry_prefix = if is_last { "└── " } else { "├── " };
    println!("{}{}{}", prefix, entry_prefix, dir.file_name().unwrap_or_default().to_string_lossy());

    let new_prefix = if is_last {
        prefix.to_string() + "    "
    } else {
        prefix.to_string() + "│   "
    };

    let mut entries: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .filter(|path| !is_well_hidden(path))
        .collect();

    entries.sort(); // Sort alphabetically for consistent output

    let num_entries = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        print_tree(b, entry, &new_prefix, i == num_entries - 1)?;
    }

    Ok(())
}

// We don't include HIDDEN+SYSTEM directories such as $RECYCLE.BIN or System Volume Information
// But just hidden dirs such as .git are included
// But just hidden dirs such as .git are included
fn is_well_hidden(path: &Path) -> bool {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };

    #[cfg(target_os = "windows")]
    {
        let attributes = metadata.file_attributes();
        let is_system = (attributes & 0x00000004) != 0;
        let is_hidden = (attributes & 0x00000002) != 0;
        return is_system & is_hidden;
    }

    false
}
