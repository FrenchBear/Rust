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

//#![allow(unused)]

// standard library imports
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;
use std::fs;

// external crates imports
use myglob::{MyGlobMatch, MyGlobSearch};

// -----------------------------------
// Submodules

mod options;
mod actions;
mod logging;

use options::*;
use logging::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = "rfind";
const APP_VERSION: &str = "1.5.1";

// -----------------------------------
// Traits

trait Action: Debug {
    fn action(&self, lw: &mut LogWriter, path: &Path, noaction: bool, verbose: bool);
    fn name(&self) -> &'static str;
}

// ==============================================================================================
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
    let mut writer = logging::new(options.verbose);

    let start = Instant::now();

    // Convert String sources into MyGlobSearch structs
    let mut sources: Vec<(&String, MyGlobSearch)> = Vec::new();
    for source in options.sources.iter() {
        let resgs = MyGlobSearch::new(source).autorecurse(options.autorecurse).compile();
        match resgs {
            Ok(gs) => sources.push((source, gs)),
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
            "dir" => actions.push(Box::new(actions::ActionPrint::new(true))),
            "delete" => actions.push(Box::new(actions::ActionDelete::new(options.recycle))),
            "rmdir" => actions.push(Box::new(actions::ActionRmdir::new(options.recycle))),
            _ => panic!("{APP_NAME}: Internal error, unknown action_name {action_name}"),
        }
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
            logln(&mut writer, "Only search for empty files or folders");
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
                        for ba in actions.iter() {
                            (**ba).action(&mut writer, &pb, options.noaction, options.verbose);
                        }
                    }
                }

                MyGlobMatch::Dir(pb) => {
                    if options.search_dirs && (!options.isempty || !is_dir_empty(&pb)) {
                        dirs_count += 1;
                        for ba in actions.iter() {
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
    fs::metadata(path).unwrap().len()>0
}

fn is_dir_empty(path: &PathBuf) -> bool {
    match fs::read_dir(path) {
        Ok(mut p) => p.next().is_some(),
        Err(_) => false,
    }
}