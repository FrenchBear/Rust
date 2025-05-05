// Logging library
// My own logging support
//
// 2025-04-03   PV      Moved to separate file
// 2025-04-30   PV      Use colored instead of termcolor
// 2025-05-05   PV      Moved to a crate and added support for MacOS and Linux
//
//#[allow(unused)]

// External crates imports

// -----------------------------------
// Submodules

// Standard library imports
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

// External crates imports
use chrono::{DateTime, Local};
use colored::*;

mod tests;

// -----------------------------------
// Globals

const LIB_VERSION: &str = "1.0.0";

pub fn version() -> &'static str {
    LIB_VERSION
}

// -----------------------------------

pub type LogWriter = Option<BufWriter<File>>;

pub fn logln(lw: &mut LogWriter, msg: &str) {
    if msg.starts_with("***") {
        println!("{}", msg.red().bold());
    } else {
        println!("{}", msg);
    }
    if let Some(bw) = lw {
        let _ = writeln!(bw, "{}", msg);
    }
}

#[allow(unused)]
pub fn log(lw: &mut LogWriter, msg: &str) {
    print!("{}", msg);
    if let Some(bw) = lw {
        let _ = write!(bw, "{}", msg);
    }
}

// Create a new logging file, and if verbose is true, write app name+version on first line
pub fn new(app_name: &str, app_version: &str, verbose: bool) -> LogWriter {
    let now: DateTime<Local> = Local::now();
    let formatted_now = now.format("%Y-%m-%d-%H.%M.%S");

    let tmp_folder = if cfg!(target_os = "windows") {
        PathBuf::from("C:\\Temp")
    } else if cfg!(target_os = "linux") {
        expand_tilde( r"~/temp")
    } else if cfg!(target_os = "macos") {
        expand_tilde ( r"~/Temp")
    } else {
        eprintln!("{app_name}: OS not recognized when creating LogWriter, no log created.");
        return None;
    };
    if !tmp_folder.is_dir() {
        eprintln!("{app_name}: Can't find temp folder {}, no log created.", tmp_folder.display());
        return None;
    }
    let dir_sep = if cfg!(target_os = "windows") { '\\' } else { '/' };
    let logpath = format!("{}{dir_sep}{app_name}-{formatted_now}.log", tmp_folder.display());
    let file = File::create(logpath.clone());
    if file.is_err() {
        logln(
            &mut None,
            format!("{app_name}: Error creating log file {logpath}, no log created: {:?}", file.err()).as_str(),
        );
        return None;
    }
    let mut writer = Some(BufWriter::new(file.unwrap()));
    if verbose {
        logln(&mut writer, &format!("{app_name} {app_version}"));
    }

    writer
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(mut stripped) = path.strip_prefix("~") {
        if let Some(home) = dirs::home_dir() {
            if stripped.starts_with('/') || stripped.starts_with('\\') {
                stripped = &stripped[1..];
            }
            return home.join(stripped.trim_start_matches('/'));
        }
    }
    PathBuf::from(path)
}
