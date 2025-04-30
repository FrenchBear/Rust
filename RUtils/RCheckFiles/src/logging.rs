// logging.rs
// Logging support
//
// 2025-04-03   PV      Moved to separate file
// 2025-04-30   PV      Use colored instead of termcolor

// stdlib
use std::fs::File;
use std::io::{BufWriter, Write};

// external crates imports
use chrono::{DateTime, Local};
use colored::*;

use super::*;

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
pub(crate) fn new(verbose:bool) -> LogWriter {
    let now: DateTime<Local> = Local::now();
    let formatted_now = now.format("%Y-%m-%d-%H.%M.%S");
    let logpath = format!("c:\\temp\\{APP_NAME}-{formatted_now}.log");
    let file = File::create(logpath.clone());
    if file.is_err() {
        logln(&mut None, format!("{APP_NAME}: Error creating log file {logpath}: {:?}", file.err()).as_str());
        process::exit(1);
    }
    let mut writer = Some(BufWriter::new(file.unwrap()));
    if verbose {
        logln(&mut writer, &format!("{APP_NAME} {APP_VERSION}"));
    }

    writer
}
