// actions.rs, definition of actions
//
// 2025-03-29	PV      First version
// 2025-03-31	PV      Action Print with option detail
// 2025-04-06	PV      Use fs::remove_dir_all instead of fs::remove_dir to delete non-empty directories
// 2025-05-05	PV      Linux compatibility
// 2025-07-12	PV      Bug name inverted (recycle/permanent delete) for action delete

use super::*;

use chrono::{DateTime, Local, Utc};
use num_format::{Locale, ToFormattedString};
use std::fs;

use trash::delete;

// ===============================================================
// Print action

#[derive(Debug)]
pub struct ActionPrint {
    detailed_output: bool,
}

impl ActionPrint {
    pub fn new(detailed_output: bool) -> Self {
        ActionPrint { detailed_output }
    }
}

impl Action for ActionPrint {
    fn action(&self, lw: &mut LogWriter, path: &Path, _noaction: bool, _verbose: bool) {
        if path.is_file() {
            if self.detailed_output {
                match path.metadata() {
                    Ok(meta) => {
                        // File size formatting
                        let file_size = meta.len();
                        let formatted_size = file_size.to_formatted_string(&Locale::fr); //Use French locale for now. Later we will find the user locale.

                        // Last modified time formatting
                        let modified_time = meta.modified().unwrap(); // Get last modified time
                        let datetime_utc: DateTime<Utc> = DateTime::<Utc>::from(modified_time);
                        let datetime_local = datetime_utc.with_timezone(&Local);
                        let formatted_time = datetime_local.format("%d/%m/%Y %H:%M:%S");

                        logln(lw, format!("{:>19}   {:>15} {}", formatted_time, formatted_size, path.display()).as_str());
                    }
                    Err(e) => {
                        logln(lw, format!("*** Error retrieving metadata for file {}: {e}", path.display()).as_str());
                    }
                }
            } else {
                logln(lw, path.display().to_string().as_str());
            }
        } else {
            let dir_sep = if cfg!(target_os = "windows") { '\\' } else { '/' };

            if self.detailed_output {
                match path.metadata() {
                    Ok(meta) => {
                        // Last modified time formatting
                        let modified_time = meta.modified().unwrap(); // Get last modified time
                        let datetime_utc: DateTime<Utc> = DateTime::<Utc>::from(modified_time);
                        let datetime_local = datetime_utc.with_timezone(&Local);
                        let formatted_time = datetime_local.format("%d/%m/%Y %H:%M:%S");

                        logln(
                            lw,
                            format!("{:>19}   {:<15} {}{dir_sep}", formatted_time, "<DIR>", path.display()).as_str(),
                        );
                    }
                    Err(e) => {
                        logln(lw, format!("*** Error retrieving metadata for dir {}: {e}", path.display()).as_str());
                    }
                }
            } else {
                let mut msg = path.display().to_string();
                msg.push(dir_sep);
                logln(lw, msg.as_str());
            }
        }
    }

    fn name(&self) -> &'static str {
        if self.detailed_output { "Dir" } else { "Print" }
    }
}

// ===============================================================
// Delete action (remove files)

#[derive(Debug)]
pub struct ActionDelete {
    recycle: bool,
}

impl ActionDelete {
    pub fn new(recycle: bool) -> Self {
        ActionDelete { recycle }
    }
}

impl Action for ActionDelete {
    fn action(&self, lw: &mut LogWriter, path: &Path, noaction: bool, verbose: bool) {
        if path.is_file() {
            let s = quoted_path(path);
            let qp = s.as_str();
            if !self.recycle {
                logln(lw, format!("DEL {}", qp).as_str());
                if !noaction {
                    match fs::remove_file(path) {
                        Ok(_) => {
                            if verbose {
                                logln(lw, format!("File {} deleted successfully.", qp).as_str());
                            }
                        }
                        Err(e) => logln(lw, format!("*** Error deleting file (fs::remove_file) {}: {}", qp, e).as_str()),
                    }
                }
            } else {
                logln(lw, format!("RECYCLE {}", qp).as_str());
                if !noaction {
                    match delete(path) {
                        Ok(_) => {
                            if verbose {
                                logln(lw, format!("File {} deleted successfully.", qp).as_str());
                            }
                        }
                        Err(e) => logln(lw, format!("*** Error deleting file (trash::delete) {}: {}", qp, e).as_str()),
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        if self.recycle {
            "Delete files (use recycle bin for local files, permanently for remote files)"
        } else {
            "Delete files (permanently)"
        }
    }
}

fn quoted_path(path: &Path) -> String {
    let n = path.display().to_string();
    if n.contains(' ') { format!("\"{}\"", n) } else { n }
}

// ===============================================================
// Rmdir action (remove directories)

#[derive(Debug)]
pub struct ActionRmdir {
    recycle: bool,
}

impl ActionRmdir {
    pub fn new(recycle: bool) -> Self {
        ActionRmdir { recycle }
    }
}

impl Action for ActionRmdir {
    fn action(&self, writer: &mut LogWriter, path: &Path, noaction: bool, verbose: bool) {
        if path.is_dir() {
            let s = quoted_path(path);
            let qp = s.as_str();
            if !self.recycle {
                logln(writer, format!("RD /S {}", qp).as_str());
                if !noaction {
                    match fs::remove_dir_all(path) {
                        Ok(_) => {
                            if verbose {
                                logln(writer, format!("Dir {} deleted successfully.", qp).as_str());
                            }
                        }
                        Err(e) => logln(writer, format!("*** Error deleting dir (fs::remove_dir_all) {}: {}", qp, e).as_str()),
                    }
                }
            } else {
                logln(writer, format!("RECYCLE (dir) {}", quoted_path(path)).as_str());
                if !noaction {
                    match delete(path) {
                        Ok(_) => {
                            if verbose {
                                logln(writer, format!("Dir '{}' deleted successfully.", qp).as_str());
                            }
                        }
                        Err(e) => logln(writer, format!("*** Error deleting dir (trash::delete) {}: {}", qp, e).as_str()),
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        if self.recycle {
            "Delete directories (use recycle bin for local files, permanently for remote files)"
        } else {
            "Delete directories (permanent)"
        }
    }
}
