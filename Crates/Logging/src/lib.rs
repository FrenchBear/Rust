// Logging library
// My own logging support
//
// 2025-04-03   PV      Moved to separate file
// 2025-04-30   PV      Use colored instead of termcolor
// 2025-05-05   PV      Moved to a crate and added support for MacOS and Linux
// 2025-09-15   PV      1.1: Debugging lines with prefix dbg: are shown in cyan; LogWriter now a struct with path field; logwriter_none()
// 2025-20-22   PV      Clippy review

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

const LIB_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() -> &'static str {
    LIB_VERSION
}

// -----------------------------------

pub struct LogWriter {
    writer_opt: Option<BufWriter<File>>,
    path: Option<PathBuf>,
}

pub fn logwriter_none() -> LogWriter {
    LogWriter {
        writer_opt: None,
        path: None,
    }
}

impl LogWriter {
    pub fn get_path(self) -> Option<PathBuf> {
        self.path.clone()
    }
}

// -----------------------------------

pub fn logln(lw: &mut LogWriter, msg: &str) {
    if msg.starts_with("***") {
        println!("{}", msg.red().bold());
    } else if msg.starts_with("dbg:") {
        println!("{}", msg.cyan());
    } else {
        println!("{}", msg);
    }
    if let Some(bw) = lw.writer_opt.as_mut() {
        let _ = writeln!(bw, "{}", msg);
    }
}

#[allow(unused)]
pub fn log(lw: &mut LogWriter, msg: &str) {
    print!("{}", msg);
    if let Some(bw) = lw.writer_opt.as_mut() {
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
        expand_tilde(r"~/temp")
    } else if cfg!(target_os = "macos") {
        expand_tilde(r"~/Temp")
    } else {
        eprintln!("{app_name}: OS not recognized when creating LogWriter, no log created.");
        return logwriter_none();
    };
    if !tmp_folder.is_dir() {
        eprintln!("{app_name}: Can't find temp folder {}, no log created.", tmp_folder.display());
        return logwriter_none();
    }
    let dir_sep = if cfg!(target_os = "windows") { '\\' } else { '/' };
    let logpath = format!("{}{dir_sep}{app_name}-{formatted_now}.log", tmp_folder.display());
    let file = File::create(logpath.clone());
    if file.is_err() {
        logln(
            &mut logwriter_none(),
            format!("{app_name}: Error creating log file {logpath}, no log created: {:?}", file.err()).as_str(),
        );
        return logwriter_none();
    }
    let writer = Some(BufWriter::new(file.unwrap()));

    let mut lw = LogWriter {
        writer_opt: writer,
        path: Some(PathBuf::from(logpath)),
    };

    if verbose {
        logln(&mut lw, &format!("{app_name} {app_version}"));
    }

    lw
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(mut stripped) = path.strip_prefix("~")
        && let Some(home) = dirs::home_dir()
    {
        if stripped.starts_with('/') || stripped.starts_with('\\') {
            stripped = &stripped[1..];
        }
        return home.join(stripped.trim_start_matches('/'));
    }
    PathBuf::from(path)
}
