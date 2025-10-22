// rfind: A Rust version of find/XFind/Search
//
// 2025-03-29	PV      First version
// 2025-03-31	PV      1.1.0 Action Dir
// 2025-04-03	PV      1.2.0 Core reorganization, logging module
// 2025-04-06	PV      1.3.0 Use fs::remove_dir_all instead of fs::remove_dir to delete non-empty directories
// 2025-04-12	PV      1.4.0 Option -empty
// 2025-04-13	PV      1.4.1 Use MyGlobSearch autorecurse
// 2025-04-13	PV      1.4.2 Option -noa[utorecurse]
// 2025-04-22	PV      1.5.0 Options -a+|-, -r+|-, options module
// 2025-05-03	PV      1.6.0 Option -name
// 2025-05-03	PV      1.6.0 Option -name
// 2025-05-04   PV      1.6.1 Use MyMarkup for extended help formatting.
// 2025-05-05	PV      1.7.0 Logging crate and Linux compatibility
// 2025-07-11   PV      1.7.1 Get info from Cargo.toml and use build.rs
// 2025-07-12	PV      1.7.2 Bug name inverted (recycle/permanent delete) for action delete
// 2025-07-12	PV      1.7.2 Bug name inverted (recycle/permanent delete) for action delete
// 2025-08-11 	PV 		1.8.1 Fixed is_file_empty bug
// 2025-09-06 	PV 		1.9.0 Option -maxdepth n
// 2025-09-08 	PV 		1.9.1 Use MyGlobSearch 1.9.0 with breadth-first search for a more natural order
// 2025-09-15 	PV 		1.10.0 Do not write log file by default, use option -log for that. Option -dbg for debugging; logwriter_none
// 2025-10-13 	PV 		2.0.0 Option -exec cmd ;
// 2025-10-13 	PV 		2.0.1 Option -xargs cmd ;
// 2025-10-17   PV      2.1.0 Options -yaml and -cs
// 2025-10-22   PV      2.1.1 to_yaml_single_quoted for ActionYaml to avoid problems with filenames containing special yaml values/characters
// 2025-20-22   PV      Clippy review
// 2025-20-22   PV      2.2.0 option -dir show Windows files attributes

// Notes:
// - Finding denormalized paths is handled by rcheckfiles and checknnn, no need for a third version :-)
// - This program uses MyGlob for enumeration, with standard filters, so $RECYCLE.BIN, .git and System Volume
//   Information are automatically filtered out. Maybe add an option -noglobfilter to optionally deactivate this filtering,
//   until then, use USE_MYGLOB_DEFAULT_EXCLUSIONS constant

// ToDo:
// - Accent insensitive search (actually maybe not useful, but everything does it)
// - Option -rename with a simplified sed syntax

//#![allow(unused)]

// Standard library imports
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

// External crates imports
use logging::{LogWriter, log, logln, logwriter_none};
use myglob::{MyGlobMatch, MyGlobSearch};
use windows as _;

// -----------------------------------
// Submodules

mod actions;
mod options;

use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const USE_MYGLOB_DEFAULT_EXCLUSIONS: bool = true;


// -----------------------------------
// Traits

trait Action: Debug {
    fn name(&self) -> String;
    fn action(&mut self, lw: &mut LogWriter, path: &Path, noaction: bool, verbose: bool);
    fn conclusion(&mut self, lw: &mut LogWriter, noaction: bool, verbose: bool);
}

// ==============================================================================================
// Main

fn main() {
    // Process options
    let mut options = Options::new().unwrap_or_else(|err| {
        let msg = format!("{}", err);
        if msg.is_empty() {
            process::exit(0);
        }
        logln(
            &mut logwriter_none(),
            format!("*** {APP_NAME}: Problem parsing arguments: {}", err).as_str(),
        );
        process::exit(1);
    });

    // Prepare log writer
    let mut writer = if options.log {
        logging::new(APP_NAME, APP_VERSION, options.verbose)
    } else {
        logwriter_none()
    };

    let start = Instant::now();

    // Adjust sources if option -name is used (for compatibility with XFind/Search)
    // In this case, appends \**\name to each source that is a valid directory
    if !options.names.is_empty() {
        let name = if options.names.len() == 1 {
            options.names.first().unwrap().clone()
        } else {
            String::from("{") + &options.names.join(",") + "}"
        };

        for source in &mut options.sources {
            let p = Path::new(&source);
            if let Ok(m) = p.metadata()
                && m.is_dir()
            {
                let dir_sep = if cfg!(target_os = "windows") { '\\' } else { '/' };

                if !(source.ends_with('/') || source.ends_with('\\')) {
                    (*source).push(dir_sep);
                    (*source).push(dir_sep);
                }
                (*source).push_str("**");
                (*source).push(dir_sep);
                (*source).push(dir_sep);
                *source += name.as_str();
            }
        }
    }

    // Convert String sources into MyGlobSearch structs
    let mut sources: Vec<(&String, MyGlobSearch)> = Vec::new();
    for source in options.sources.iter() {
        let mut builder = MyGlobSearch::new(source)
            .autorecurse(options.autorecurse)
            .maxdepth(options.maxdepth)
            .case_sensitive(options.case_sensitive);
        if !USE_MYGLOB_DEFAULT_EXCLUSIONS {
            builder = builder.clear_ignore_dirs();
        }
            
        let resgs = builder.compile();
        match resgs {
            Ok(gs) => {
                if options.debug {
                    logln(&mut writer, format!("dbg: {} -> {:?}", source, gs.segments).as_str());
                }
                sources.push((source, gs));
            }
            Err(e) => {
                logln(&mut writer, format!("*** Error building MyGlob: {:?}", e).as_str());
            }
        }
    }
    if sources.is_empty() {
        logln(&mut writer, format!("*** No source specified. Use {APP_NAME} ? to show usage.").as_str());
        process::exit(1);
    }

    if options.verbose {
        log(&mut writer, "\nSources(s): ");
        if options.search_dirs && options.search_files {
            logln(&mut writer, "(search for files and directories)");
        } else if options.search_dirs {
            logln(&mut writer, "(search for directories)");
        } else {
            logln(&mut writer, "(search for files)");
        }

        for source in sources.iter() {
            logln(&mut writer, format!("- {}", source.0).as_str());
        }
    }

    let mut actions = Vec::<Box<dyn Action>>::new();
    for action_name in options.actions_names.iter() {
        match *action_name {
            "print" => {
                if options.actions_names.contains("dir") {
                    logln(&mut writer, "*** Both actions print and dir used, action print ignored.");
                } else {
                    actions.push(Box::new(actions::ActionPrint::new(false)))
                }
            }
            "yaml" => actions.push(Box::new(actions::ActionYaml::new())),
            "dir" => actions.push(Box::new(actions::ActionPrint::new(true))),
            "delete" => actions.push(Box::new(actions::ActionDelete::new(options.recycle))),
            "rmdir" => actions.push(Box::new(actions::ActionRmdir::new(options.recycle))),
            "nop" => {}
            _ => panic!("{APP_NAME}: Internal error, unknown action_name {action_name}"),
        }
    }
    for ctr in options.exec_commands.iter() {
        actions.push(Box::new(actions::ActionExec::new(ctr)));
    }
    for ctr in options.xargs_commands.iter() {
        actions.push(Box::new(actions::ActionXargs::new(ctr)));
    }

    if options.verbose {
        log(&mut writer, "\nAction(s): ");
        if options.noaction {
            logln(&mut writer, "(no action will be actually performed)");
        } else {
            logln(&mut writer, "");
        }
        for ba in actions.iter() {
            logln(&mut writer, format!("- {}", (**ba).name()).as_str());
        }
        logln(&mut writer, "");
        if options.isempty {
            logln(&mut writer, "Only search for empty files or directories");
        }
    }

    let mut files_count = 0;
    let mut dirs_count = 0;
    for gs in sources.iter() {
        for ma in gs.1.explore_iter() {
            match ma {
                MyGlobMatch::File(pb) => {
                    if options.search_files && (!options.isempty || is_file_empty(&pb)) {
                        files_count += 1;
                        for ba in actions.iter_mut() {
                            (**ba).action(&mut writer, &pb, options.noaction, options.verbose);
                        }
                    }
                }

                MyGlobMatch::Dir(pb) => {
                    if options.search_dirs && (!options.isempty || !is_dir_empty(&pb)) {
                        dirs_count += 1;
                        for ba in actions.iter_mut() {
                            (**ba).action(&mut writer, &pb, options.noaction, options.verbose);
                        }
                    }
                }

                MyGlobMatch::Error(err) => {
                    if options.verbose {
                        logln(&mut writer, format!("{APP_NAME}: MyGlobMatch error {}", err).as_str());
                    }
                }
            }
        }
    }

    // Call conclusions
    for ba in actions.iter_mut() {
        (**ba).conclusion(&mut writer, options.noaction, options.verbose);
    }

    let duration = start.elapsed();

    if options.verbose {
        if files_count + dirs_count > 0 {
            logln(&mut writer, "");
        }
        if options.search_files {
            log(&mut writer, format!("{files_count} files(s)").as_str());
        }
        if options.search_dirs {
            if options.search_files {
                log(&mut writer, ", ");
            }
            log(&mut writer, format!("{dirs_count} dir(s)").as_str());
        }
        logln(&mut writer, format!(" found in {:.3}s", duration.as_secs_f64()).as_str());
    }
}

fn is_file_empty(path: &PathBuf) -> bool {
    fs::metadata(path).unwrap().len() == 0
}

fn is_dir_empty(path: &PathBuf) -> bool {
    match fs::read_dir(path) {
        Ok(mut p) => p.next().is_some(),
        Err(_) => false,
    }
}
