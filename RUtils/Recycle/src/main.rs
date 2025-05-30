// recycle: Delete files and directories to trash
//
// 2025-04-03	PV      First version
// 2025-04-17	PV      1.1.0 Fixed logic errors (return statement misplaced in embedded is blocks)
// 2025-05-05   PV      1.1.2 Use MyMarkup crate to format usage and extended help
// 2025-05-05	PV      1.1.3 Logging crate

//#![allow(unused)]

// Standard library imports
use std::error::Error;
use std::path::Path;
use std::process;
use std::time::Instant;

// External crates imports
use getopt::Opt;
use myglob::{MyGlobMatch, MyGlobSearch};
use mymarkup::MyMarkup;
use logging::*;

// -----------------------------------
// Submodules

mod drive_type;
mod reparse;
mod tests;

use drive_type::*;
use reparse::*;

// -----------------------------------
// Globals

const APP_NAME: &str = "recycle";
const APP_VERSION: &str = "1.1.3";

// ==============================================================================================
// Options processing

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    sources: Vec<String>,
    no_action: bool,
    verbose: bool,
    silent: bool,
}

impl Options {
    fn header() {
        println!(
            "{APP_NAME} {APP_VERSION}\n\
            Delete files and directories to trash"
        );
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄] [⦃-v⦄] [⦃-s⦄] [⦃-n⦄] ⟨source⟩...

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄       ¬Show advanced usage notes
⦃-v⦄       ¬Verbose output
⦃-s⦄       ¬Silent mode, silently ignore files/dirs not found
⦃-n⦄       ¬No action (nothing deleted)
⟨source⟩   ¬File or directory to delete, or file glob pattern";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        let text =
"Copyright ©2025 Pierre Violent

⟪⌊Advanced usage notes⌋⟫

Only local files (local drive or attached USB drive) support trash.
Network files can't be deleted to recycle bin, so they can't be removed with this command (contrary to PDEL that will remove remote files permanently).";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
        MyMarkup::render_markup(MyGlobSearch::glob_syntax());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    fn new() -> Result<Options, Box<dyn Error>> {
        let mut args: Vec<String> = std::env::args().collect();
        if args.len() > 1 && args[1].to_lowercase() == "help" {
            Self::usage();
            return Err("".into());
        }

        if args[1] == "??" || args[1] == "-??" {
            Self::extended_usage();
            return Err("".into());
        }

        let mut options = Options { ..Default::default() };
        let mut opts = getopt::Parser::new(&args, "h?vsn");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('v', None) => {
                        options.verbose = true;
                    }

                    Opt('s', None) => {
                        options.silent = true;
                    }

                    Opt('n', None) => {
                        options.no_action = true;
                    }

                    _ => unreachable!(),
                },
            }
        }

        // Check for extra argument
        for arg in args.split_off(opts.index()) {
            if arg == "?" || arg == "help" {
                Self::usage();
                return Err("".into());
            }

            if arg.starts_with("-") {
                return Err(format!("Invalid/unsupported option {}", arg).into());
            }

            options.sources.push(arg);
        }

        Ok(options)
    }
}

// -----------------------------------
// Main

fn main() {
    // Process options
    let options = Options::new().unwrap_or_else(|err| {
        let msg = format!("{}", err);
        if msg.is_empty() {
            process::exit(0);
        }
        logln(&mut None, format!("*** {APP_NAME}: Problem parsing arguments: {}", err).as_str());
        process::exit(1);
    });

    // Prepare log writer
    let mut writer = logging::new(APP_NAME, APP_VERSION, options.verbose);

    let mut files_count = 0;
    let mut dirs_count = 0;

    let start = Instant::now();

    for source in options.sources.iter() {
        let p = Path::new(&source);
        if p.is_file() {
            recycle_file(&mut writer, p, &mut files_count, &options);
        } else if p.is_dir() {
            // Check that it's not a reparse point
            recycle_dir(&mut writer, p, &mut dirs_count, &options);
        } else {
            let gsres = MyGlobSearch::build(source);
            let gs = match gsres {
                Ok(gs) => gs,
                Err(_) => {
                    if !options.silent {
                        logln(
                            &mut writer,
                            format!("*** Source {source} is neither a file nor a dir nor a valid glob, ignored").as_str(),
                        );
                    }
                    continue;
                }
            };

            if gs.is_constant() {
                if !options.silent {
                    logln(&mut writer, format!("*** Source {source} is neither a file nor a dir, ignored").as_str());
                }
                continue;
            }

            for ma in gs.explore_iter() {
                let problem = match ma {
                    MyGlobMatch::File(pb) => recycle_file(&mut writer, &pb, &mut files_count, &options),
                    MyGlobMatch::Dir(pb) => recycle_dir(&mut writer, &pb, &mut dirs_count, &options),
                    MyGlobMatch::Error(e) => {
                        if !options.silent {
                            logln(&mut writer, format!("*** Error {e}").as_str());
                        }
                        false
                    }
                };

                if problem && !options.silent {
                    logln(&mut writer, format!("*** Exploration of glob {source} is stopped").as_str());
                }
            }
        }
    }

    if options.verbose {
        let duration = start.elapsed();

        fn s(n: i32) -> &'static str {
            if n > 1 { "s" } else { "" }
        }

        if files_count > 0 {
            log(&mut writer, &format!("{files_count} file{} recycled", s(files_count)));
            if options.no_action {
                log(&mut writer, " (noaction)");
            }
            logln(&mut writer, "");
        }
        if dirs_count > 0 {
            log(&mut writer, &format!(", {files_count} dir{} recycled", s(dirs_count)));
            if options.no_action {
                log(&mut writer, " (noaction)");
            }
            logln(&mut writer, "");
        }

        logln(&mut writer, &format!("Total duration: {:.3}s", duration.as_secs_f64()));
    }
}

fn recycle_dir(writer: &mut LogWriter, path: &Path, dirs_count: &mut i32, options: &Options) -> bool {
    if options.no_action {
        if options.verbose {
            logln(writer, format!("RD /S {}", quoted_path(path)).as_str());
        }
        *dirs_count += 1;
        return false;
    }

    // We don't recycle dirs located on a remote drive
    if let Ok(dt) = drive_type(path) {
        if dt == DriveType::DRIVE_REMOTE {
            if !options.silent {
                logln(
                    writer,
                    format!("*** Can't recycle dir {} located on a remote drive", quoted_path(path)).as_str(),
                );
            }
            return true; // Block glob processing, since all other dirs are on remote drive
        }
    }

    if let Ok(rt) = reparse_type(path) {
        if rt == ReparseType::Junction || rt == ReparseType::SymLink || rt == ReparseType::Stub {
            if !options.silent {
                logln(
                    writer,
                    format!(
                        "*** Can't recycle dir {} reparse point {}",
                        if rt == ReparseType::Junction {
                            "JUNCTION"
                        } else if rt == ReparseType::SymLink {
                            "SYMLINK"
                        } else {
                            "STUB"
                        },
                        quoted_path(path)
                    )
                    .as_str(),
                );
            }
            return false; // Don't block glob processing
        }
    }

    match trash::delete(path) {
        Ok(_) => {
            if options.verbose {
                logln(writer, format!("RD /S {}", quoted_path(path)).as_str());
            }
            *dirs_count += 1;
        }

        Err(e) => {
            if !options.silent {
                logln(
                    writer,
                    format!("*** Error deleting file (trash::delete) {}: {}", quoted_path(path), e).as_str(),
                );
            }
        }
    }
    false // No problem
}

fn recycle_file(writer: &mut LogWriter, path: &Path, files_count: &mut i32, options: &Options) -> bool {
    if options.no_action {
        if options.verbose {
            logln(writer, format!("DEL {}", quoted_path(path)).as_str());
        }
        *files_count += 1;
        return false;
    }

    // We don't recycle files located on a remote drive
    if let Ok(dt) = drive_type(path) {
        if dt == DriveType::DRIVE_REMOTE {
            if !options.silent {
                logln(
                    writer,
                    format!("*** Can't recycle file {} located on a remote drive", quoted_path(path)).as_str(),
                );
            }
            return true; // Block glob processing, since all other files are on remote drive
        }
    }

    // SYMLINK files can safely be sent to trash, no need to block them
    // OneDrive Stubs are deleted locally AND on OneDrive, they're not copied to local recycle.bin, but in
    // outlook recycle.bin.  For security, let refuse to delete them by default
    if let Ok(rt) = reparse_type(path) {
        if rt == ReparseType::Stub {
            if !options.silent {
                logln(writer, format!("*** Can't recycle file stub {}", quoted_path(path)).as_str());
            }
            return false; // Don't block glob processing
        }
    }

    match trash::delete(path) {
        Ok(_) => {
            if options.verbose {
                println!("DEL {}", quoted_path(path));
            }
            *files_count += 1;
        }

        Err(e) => {
            if !options.silent {
                logln(
                    writer,
                    format!("*** Error deleting file (trash::delete) {}: {}", quoted_path(path), e).as_str(),
                );
            }
        }
    }
    false // No problem
}

fn quoted_path(path: &Path) -> String {
    let n = path.display().to_string();
    if n.contains(' ') { format!("\"{}\"", n) } else { n }
}
