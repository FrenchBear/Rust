// rtree
// Visual directory structure in Rust
//
// 2025-05-05   PV      First version (from Gemini)
// 2025-06-29   PV      Renames (from -h) and parsed correctly option -a, but still don't use it in code
// 2025-06-29   PV      Renames (from -h) and parsed correctly option -a, but still don't use it in code
// 2025-07-04   PV      1.2.0 Option -A, option -d. Process junctions metadata correctly

#![allow(unused)]

// Standard library imports
use std::cmp::Ordering;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::prelude::*;
#[cfg(target_os = "windows")]
use windows::{
    Win32::UI::Shell::StrCmpLogicalW,
    core::{HRESULT, PCWSTR},
}; // For OsStringExt::encode_wide

// External crates imports

// -----------------------------------
// Submodules

mod options;

use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// ==============================================================================================
// Call StrCmpLogicalW

/// Compares two strings using the natural sort algorithm, similar to Windows File Explorer.
/// This is a Windows-specific function.
#[cfg(target_os = "windows")]
fn str_cmp_logical_w_rust(s1: &str, s2: &str) -> Ordering {
    // Convert Rust &str to null-terminated wide strings (UTF-16) for Windows API.
    // OsString is convenient for this as it handles platform-specific string encoding.
    let os_str1: OsString = s1.into();
    let wide_s1: Vec<u16> = os_str1.encode_wide().chain(std::iter::once(0)).collect(); // Add null terminator
    let p1 = PCWSTR(wide_s1.as_ptr());

    let os_str2: OsString = s2.into();
    let wide_s2: Vec<u16> = os_str2.encode_wide().chain(std::iter::once(0)).collect(); // Add null terminator
    let p2 = PCWSTR(wide_s2.as_ptr());

    // Call the Windows API function
    // The `windows` crate functions typically return a Result,
    // where Ok(0) means success, and a non-zero value for comparison results.
    // StrCmpLogicalW returns an INT, so we directly use its return value.
    let result = unsafe { StrCmpLogicalW(p1, p2) };

    // StrCmpLogicalW returns:
    // < 0 if psz1 comes before psz2
    //   0 if psz1 is identical to psz2
    // > 0 if psz1 comes after psz2
    // We map this to Rust's Ordering enum.
    result.cmp(&0)
}

// ==============================================================================================
// Main

#[derive(Debug, Default)]
struct DataBag {
    dirs_count: usize,
    links_count: usize,
}

// fn main() {
//     let (h, s) = is_hidden_or_system_dir(Path::new(r"C:\Users\Pierr\AppData"));
//     println!("h:{h} s:{s}");

//     let (h, s) = is_hidden_or_system_dir(Path::new(r"C:\Users\Pierr\SendTo"));
//     println!("h:{h} s:{s}");
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

    let mut b = DataBag { ..Default::default() };

    let start = Instant::now();
    match options.source.clone() {
        Some(start_dir) => {
            do_print(&mut b, &start_dir, &options);
        }
        None => {
            do_print(&mut b, ".", &options);
        }
    }
    let duration = start.elapsed();

    if options.verbose {
        print!("{} subdirectorie(s)", b.dirs_count);
        if b.links_count > 0 {
            print!(", {} link(s)", b.links_count);
        }
        println!(" in {:.3}s", duration.as_secs_f64());
    }
}

// Moved to a separate function to use ? operator
fn do_print(b: &mut DataBag, source: &str, options: &Options) -> Result<(), io::Error> {
    let start_dir = &Path::new(source);
    if !start_dir.is_dir() {
        eprintln!("{APP_NAME}: '{}' is not a valid directory.", start_dir.display());
        return Ok(());
    }

    println!("{}", start_dir.to_string_lossy());
    let mut result = fs::read_dir(start_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<PathBuf>>();
    #[cfg(not(target_os = "windows"))]
    result.sort(); // Actually, should be case insensitive sort
    #[cfg(target_os = "windows")]
    result.sort_by(|a, b| {
        str_cmp_logical_w_rust(
            a.file_name().unwrap_or_default().to_str().unwrap(),
            b.file_name().unwrap_or_default().to_str().unwrap(),
        )
    });

    let num_subdirs = result.len();
    for (i, subdir) in result.iter().enumerate() {
        let (h, s) = is_hidden_or_system_dir(subdir);
        if s && !options.show_hidden_and_system || h && !options.show_hidden {
            continue;
        }
        print_tree(b, subdir, "", i == num_subdirs - 1, options, (options.maxdepth as i32)-1)?;
    }

    Ok(())
}

fn print_tree(b: &mut DataBag, dir: &Path, prefix: &str, is_last: bool, options: &Options, depth: i32) -> io::Result<()> {
    let entry_prefix = if is_last { "└── " } else { "├── " };
    let new_prefix = if is_last {
        prefix.to_string() + "    "
    } else {
        prefix.to_string() + "│   "
    };

    print!("{}{}{}", prefix, entry_prefix, dir.file_name().unwrap_or_default().to_string_lossy());

    let s1 = "aaa";
    let zz = s1.replace("a", "b");

    if dir.is_symlink() {
        let target_path = fs::read_link(dir)?;
        let t = target_path.to_string_lossy().replace(r"\\?\", "");
        println!(" -> {}", t);
        b.links_count += 1;
        return Ok(());
    }

    b.dirs_count += 1;

    // Some directories can't be read, add ... ? to name to indicate we don't know what's below
    let rd = fs::read_dir(dir);
    if rd.is_err() {
        println!("  ... ?");
        return Ok(());
    }

    let mut entries: Vec<PathBuf> = rd?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();

    #[cfg(not(target_os = "windows"))]
    entries.sort(); // Actually, should be case insensitive sort
    #[cfg(target_os = "windows")]
    entries.sort_by(|a, b| {
        str_cmp_logical_w_rust(
            a.file_name().unwrap_or_default().to_str().unwrap(),
            b.file_name().unwrap_or_default().to_str().unwrap(),
        )
    });

    let num_entries = entries.len();

	if depth == 0 {
		if num_entries > 0 {
			println!(" ...");
		} else {
			println!();
		}
		return Ok(());
	}
	println!();

    for (i, entry) in entries.iter().enumerate() {
        let (h, s) = is_hidden_or_system_dir(&entry);
        if s && !options.show_hidden_and_system || h && !options.show_hidden {
            continue;
        }
        print_tree(b, entry, &new_prefix, i == num_entries - 1, options, depth-1)?;
    }

    Ok(())
}

// We don't include HIDDEN+SYSTEM directories such as $RECYCLE.BIN or System Volume Information
// But just hidden dirs such as .git are included
// But just hidden dirs such as .git are included
fn is_hidden_or_system_dir(path: &Path) -> (bool, bool) {
    let filename = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
    let starts_with_dot = filename.starts_with(".");
    let starts_with_dollar = filename.starts_with("$");

    #[cfg(target_os = "windows")]
    {
        //let metadata = match fs::metadata(path) {
        let metadata = match fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(_) => return (false, false),
        };

        let attributes = metadata.file_attributes();
        let is_system = (attributes & 0x00000004) != 0;
        let is_hidden = (attributes & 0x00000002) != 0;
        return (is_hidden || starts_with_dot, is_system || (is_hidden && starts_with_dollar));
    }

    (starts_with_dot, false)
}
