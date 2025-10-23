// actions.rs, definition of actions
//
// 2025-03-29	PV      First version
// 2025-03-31	PV      Action Print with option detail
// 2025-04-06	PV      Use fs::remove_dir_all instead of fs::remove_dir to delete non-empty directories
// 2025-05-05	PV      Linux compatibility
// 2025-07-12	PV      Bug name inverted (recycle/permanent delete) for action delete
// 2025-10-13   PV      ActionExec, ActionXargs
// 2025-10-17   PV      ActionYaml
// 2025-10-22   PV      to_yaml_single_quoted for Yaml action
// 2025-20-22   PV      Clippy review
// 2025-20-22   PV      PrintAction 'Dir' shows Windows files attributes, and more generally, links
// 2025-20-23   PV      PrintAction 'Dir' processes links, when target of a link does not exist. Show attributes of directories

use super::*;

// Standard library imports
use std::{fs, process::Command};

// External library imports
use chrono::{DateTime, Local, Utc};
use num_format::{Locale, ToFormattedString};

// Retrieve files/dirs attributes on Windows
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

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
    fn name(&self) -> String {
        (if self.detailed_output { "Dir" } else { "Print" }).into()
    }

    fn action(&mut self, lw: &mut LogWriter, path: &Path, _noaction: bool, _verbose: bool) {
        if self.detailed_output {
            action_dir(lw, path, _noaction, _verbose);
        } else {
            action_print(lw, path, _noaction, _verbose);
        }
    }

    fn conclusion(&mut self, _lw: &mut LogWriter, _noaction: bool, _verbose: bool) {}
}

fn action_print(lw: &mut LogWriter, path: &Path, _noaction: bool, _verbose: bool) {
    if path.is_file() {
        // Includes links to existing files
        logln(lw, path.display().to_string().as_str());
    } else if path.is_dir() {
        // Includes links to existing directories
        let dir_sep = if cfg!(target_os = "windows") { '\\' } else { '/' };
        logln(lw, format!("{}{}", path.display(), dir_sep).as_str());
    } else if path.is_symlink() {
        logln(lw, format!("{}{}", path.display(), '?').as_str());
    } else {
        logln(lw, format!("*** Error neither dir not file {}", path.display()).as_str());
    }
}

fn action_dir(lw: &mut LogWriter, path: &Path, _noaction: bool, _verbose: bool) {
    let link_string = if path.is_symlink() {
        let target_path = fs::read_link(path).unwrap();
        let t = target_path.to_string_lossy().replace(r"\\?\", "");
        format!(" -> {}", t)
    } else {
        String::new()
    };

    // Handle the case of a link when target is not accessible such as
    // C:\Users\Pierr\.julia\packages\FilePathsBase\NV2We\docs\src\index.md, a <SYMLINK> io inexistent file ../../README.md
    if path.is_symlink() && !path.is_dir() && !path.is_file() {
        // Since target does not exist, we retrieve link metadata
        let meta = fs::symlink_metadata(path).unwrap();
        let modified_time = meta.modified().unwrap(); // Get last modified time
        let datetime_utc: DateTime<Utc> = DateTime::<Utc>::from(modified_time);
        let datetime_local = datetime_utc.with_timezone(&Local);
        let formatted_time = datetime_local.format("%d/%m/%Y %H:%M:%S");

        logln(
            lw,
            format!(
                "{:>19}   {:<15}       {} {link_string}  [not found]",
                formatted_time,
                "<LINK>",
                path.display()
            )
            .as_str(),
        );
        return;
    }

    // Last modified time formatting and attributes
    let meta = match path.metadata() {
        Ok(meta) => meta,
        Err(e) => {
            logln(lw, format!("*** Error retrieving metadata for {}: {}", path.display(), e).as_str());
            return;
        }
    };

    let modified_time = meta.modified().unwrap(); // Get last modified time
    let datetime_utc: DateTime<Utc> = DateTime::<Utc>::from(modified_time);
    let datetime_local = datetime_utc.with_timezone(&Local);
    let formatted_time = datetime_local.format("%d/%m/%Y %H:%M:%S");

    // Get Windows basic attributes
    let mut attributes_string = String::new();
    #[cfg(target_os = "windows")]
    {
        // Getting metadata of link itself, not link target
        if let Ok(metadata) = fs::symlink_metadata(path) {
            let attributes = metadata.file_attributes();

            const FILE_ATTRIBUTE_READONLY: u32 = 0x00000001;
            const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;
            const FILE_ATTRIBUTE_SYSTEM: u32 = 0x00000004;

            attributes_string.push_str(if (attributes & FILE_ATTRIBUTE_SYSTEM) != 0 { "S" } else { "." });
            attributes_string.push_str(if (attributes & FILE_ATTRIBUTE_HIDDEN) != 0 { "H" } else { "." });
            attributes_string.push_str(if (attributes & FILE_ATTRIBUTE_READONLY) != 0 { "R" } else { "." });
        }
    }

    if path.is_file() {
        // Includes links to existing files
        let file_size = meta.len();
        let formatted_size = if path.is_symlink() {
            "<FILE LINK>    ".into()
        } else {
            file_size.to_formatted_string(&Locale::fr) //Use French locale for now. Later we will find the user locale.
        };

        logln(
            lw,
            format!(
                "{:>19}   {:>15}  {attributes_string}  {} {link_string}",
                formatted_time,
                formatted_size,
                path.display()
            )
            .as_str(),
        );
    } else if path.is_dir() {
        // Includes links to existing directories
        let dir_sep = if cfg!(target_os = "windows") { '\\' } else { '/' };
        let tag = if path.is_symlink() { "<DIR LINK>" } else { "<DIR>" };
        logln(
            lw,
            format!(
                "{:>19}   {:<15}  {attributes_string}  {}{dir_sep} {link_string}",
                formatted_time,
                tag,
                path.display()
            )
            .as_str(),
        );
    } else {
        logln(lw, format!("*** Error neither dir not file {}", path.display()).as_str());
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
    fn name(&self) -> String {
        (if self.recycle {
            "Delete files (use recycle bin for local files, permanently for remote files)"
        } else {
            "Delete files (permanently)"
        })
        .into()
    }

    fn action(&mut self, lw: &mut LogWriter, path: &Path, noaction: bool, verbose: bool) {
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

    fn conclusion(&mut self, _lw: &mut LogWriter, _noaction: bool, _verbose: bool) {}
}

fn quoted_path(path: &Path) -> String {
    quoted_string(&path.display().to_string())
}

fn quoted_string(string: &String) -> String {
    if string.contains(' ') {
        format!("\"{}\"", string)
    } else {
        string.into()
    }
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
    fn name(&self) -> String {
        (if self.recycle {
            "Delete directories (use recycle bin for local files, permanently for remote files)"
        } else {
            "Delete directories (permanent)"
        })
        .into()
    }

    fn action(&mut self, writer: &mut LogWriter, path: &Path, noaction: bool, verbose: bool) {
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

    fn conclusion(&mut self, _lw: &mut LogWriter, _noaction: bool, _verbose: bool) {}
}

// ===============================================================
// Exec action

#[derive(Debug)]
pub struct ActionExec {
    ctr: CommandToRun,
}

impl ActionExec {
    pub fn new(ctr: &CommandToRun) -> Self {
        ActionExec { ctr: (*ctr).clone() }
    }
}

impl Action for ActionExec {
    fn name(&self) -> String {
        format!("Exec «{}» {}", self.ctr.command, self.ctr.args.join(" "))
    }

    fn action(&mut self, lw: &mut LogWriter, path: &Path, _noaction: bool, _verbose: bool) {
        let pp = quoted_path(path);
        let mut args = self.ctr.args.clone();
        for refarg in args.iter_mut() {
            if refarg.contains("{}") {
                *refarg = refarg.replace("{}", &pp);
            }
        }

        logln(lw, format!("exec {} {}", quoted_string(&self.ctr.command), args.join(" ")).as_str());
        if !_noaction {
            let _status = Command::new(self.ctr.command.as_str()).args(&args).spawn();
            //println!("Status: {:#?}", _status);
        }
    }

    fn conclusion(&mut self, _lw: &mut LogWriter, _noaction: bool, _verbose: bool) {}
}

// ===============================================================
// Xargs action

#[derive(Debug)]
pub struct ActionXargs {
    ctr: CommandToRun,
    paths: Vec<String>,
}

impl ActionXargs {
    pub fn new(ctr: &CommandToRun) -> Self {
        ActionXargs {
            ctr: (*ctr).clone(),
            paths: Vec::new(),
        }
    }
}

impl Action for ActionXargs {
    fn name(&self) -> String {
        format!("Xargs «{}» {}", self.ctr.command, self.ctr.args.join(" "))
    }

    fn action(&mut self, _lw: &mut LogWriter, path: &Path, _noaction: bool, _verbose: bool) {
        self.paths.push(quoted_string(&path.display().to_string()));
    }

    fn conclusion(&mut self, lw: &mut LogWriter, _noaction: bool, _verbose: bool) {
        let mut args = Vec::new();
        for arg in self.ctr.args.iter() {
            if arg.contains("{}") {
                for pp in self.paths.iter() {
                    args.push(arg.replace("{}", pp));
                }
            } else {
                args.push(arg.clone());
            }
        }

        logln(lw, format!("xargs {} {}", quoted_string(&self.ctr.command), args.join(" ")).as_str());
        if !_noaction {
            let _status = Command::new(self.ctr.command.as_str()).args(&args).spawn();
        }
    }
}

// ===============================================================
// Yaml action

#[derive(Debug)]
pub struct ActionYaml {}

impl ActionYaml {
    pub fn new() -> Self {
        ActionYaml {}
    }
}

impl Action for ActionYaml {
    fn name(&self) -> String {
        "Yaml".into()
    }

    fn action(&mut self, lw: &mut LogWriter, path: &Path, _noaction: bool, _verbose: bool) {
        if path.is_file() {
            logln(lw, "- typ: file");
        } else {
            logln(lw, "- typ: dir");
        }
        let qp = to_yaml_single_quoted(path.as_os_str().to_str().unwrap());
        logln(lw, &format!("  old: {}", qp));
        logln(lw, &format!("  new: {}\n", qp));
    }

    fn conclusion(&mut self, _lw: &mut LogWriter, _noaction: bool, _verbose: bool) {}
}

/// Wraps a string in single quotes for safe inclusion in a YAML file.
///
/// In YAML, single-quoted strings handle most special characters literally,
/// including '#' and '\'. The only character that must be escaped is the
/// single quote itself, which is done by doubling it (e.g., 'It''s').
///
/// This function always returns a quoted string, which is always valid.
///
/// You must use quotes (like this function provides) if your string:
/// - Contains a # (comment character)
/// - Contains a colon followed by a space (: )
/// - Contains a hyphen followed by a space (- ) at the beginning
/// - Contains a single quote (')
/// - Starts or ends with whitespace
/// - Is empty
/// - Is the word true, false, yes, no, on, off, null, or ~
/// - Looks like a number (e.g., 123, 45.6)
///
/// Writing a function checking all these cases would be very complex
fn to_yaml_single_quoted(s: &str) -> String {
    // 1. Escape any single quotes by replacing them with two single quotes.
    let escaped = s.replace('\'', "''");

    // 2. Wrap the escaped string in single quotes.
    format!("'{}'", escaped)
}
