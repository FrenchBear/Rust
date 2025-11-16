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
// 2025-10-22   PV      2.1.2 Clippy review
// 2025-10-22   PV      2.2.0 Option -dir show Windows files attributes
// 2025-10-22   PV      2.3.0 Support of links (with MyGlob 2.0)
// 2025-10-23   PV      2.3.1 Handle correctly links to non-existent targets; no_glob_filtering option -ngf
// 2025-10-24   PV      2.3.2 Fixed MyGlob bug C:\**\thumbs.db
// 2025-10-25   PV      2.3.3 ActionDir separated from ActionPrint
// 2025-10-27   PV      2.4.0 Generic filters
// 2025-10-27   PV      2.4.1 Added {} final for -exec/-execg if there is no {} in command
// 2025-10-30   PV      2.5.0 Refactored CommandToRun and related methods to a separate source file for sharing
// 2025-11-15   PV      2.6.0 Option -w to make actions -exec/-execg synchronous (wait for command to terminate)

// Notes:
// - Finding denormalized paths is handled by rcheckfiles and checknnn, no need for a third version :-)

// ToDo:
// - Accent insensitive search (actually maybe not useful, but everything does it)
// - Option -rename with a simplified sed syntax

//#![allow(unused)]

// Standard library imports
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;

// External crates imports
use logging::{LogWriter, log, logln, logwriter_none};
use myglob::{MyGlobMatch, MyGlobSearch};
use windows as _;

// -----------------------------------
// Submodules

mod filters;
mod actions;
mod options;
mod fa_streams;
mod command_to_run;

mod tests;

// -----------------------------------
// Modules

use options::*;
use command_to_run::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// -----------------------------------
// Traits

trait Action: Debug {
    fn name(&self) -> String;
    fn action(&mut self, lw: &mut LogWriter, path: &Path, options: &Options);
    fn conclusion(&mut self, lw: &mut LogWriter, options: &Options);
}

trait Filter: Debug {
    fn name(&self) -> &'static str;
    fn filter(&mut self, lw: &mut LogWriter, path: &Path, verbose: bool) -> bool;
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
            .max_depth(options.maxdepth)
            .case_sensitive(options.case_sensitive)
            .set_link_mode(options.link_mode);
        if options.no_glob_filtering {
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

    let mut filters = Vec::<Box<dyn Filter>>::new();
    for &filter_name in options.filters_names.iter() {
        match filter_name {
            "empty" => filters.push(Box::new(filters::FilterEmpty::new())),
            "ads" => filters.push(Box::new(filters::FilterADS::new(false))),
            "adsx" => filters.push(Box::new(filters::FilterADS::new(true))),
            _ => panic!("{APP_NAME}: Internal error, unknown filter_name {filter_name}"),
        }
    }


    let mut actions = Vec::<Box<dyn Action>>::new();
    for action_name in options.actions_names.iter() {
        match *action_name {
            "print" => {
                if options.actions_names.contains("dir") {
                    logln(&mut writer, "*** Both actions print and dir used, action print ignored.");
                } else {
                    actions.push(Box::new(actions::ActionPrint::new()));
                }
            }
            "yaml" => actions.push(Box::new(actions::ActionYaml::new())),
            "dir" => actions.push(Box::new(actions::ActionDir::new())),
            "delete" => actions.push(Box::new(actions::ActionDelete::new(options.recycle))),
            "rmdir" => actions.push(Box::new(actions::ActionRmdir::new(options.recycle))),
            "nop" => {}
            _ => panic!("{APP_NAME}: Internal error, unknown action_name {action_name}"),
        }
    }
    for ctr in options.exec_commands.iter() {
        actions.push(Box::new(actions::ActionExec::new(ctr)));
    }
    for ctr in options.execg_commands.iter() {
        actions.push(Box::new(actions::ActionExecg::new(ctr)));
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

        if !filters.is_empty() {
            logln(&mut writer, "\nFilter(s): ");
            for ba in filters.iter() {  
                logln(&mut writer, format!("- {}", (**ba).name()).as_str());
            }
        }
        logln(&mut writer, "");
    }

    let mut files_count = 0;
    let mut dirs_count = 0;
    let mut errs_count = 0;
    for gs in sources.iter() {
        for ma in gs.1.explore_iter() {
            match ma {
                MyGlobMatch::File(pb) => {
                    if !options.search_files {
                        continue;
                    }

                    let mut include = true;
                    for f in filters.iter_mut() {
                        if !(**f).filter(&mut writer, &pb, options.verbose) {
                            include = false;
                            break;
                        }
                    }
                    if include {
                        files_count += 1;
                        for ba in actions.iter_mut() {
                            (**ba).action(&mut writer, &pb, &options);
                        }
                    }
                }

                MyGlobMatch::Dir(pb) => {
                    if !options.search_dirs {
                        continue;
                    }

                    let mut include = true;
                    for f in filters.iter_mut() {
                        if !(**f).filter(&mut writer, &pb, options.verbose) {
                            include = false;
                            break;
                        }
                    }
                    if include {
                        dirs_count += 1;
                        for ba in actions.iter_mut() {
                            (**ba).action(&mut writer, &pb, &options);
                        }
                    }
                }

                MyGlobMatch::Error(err) => {
                    errs_count += 1;
                    if options.verbose {
                        logln(&mut writer, format!("{APP_NAME}: MyGlobMatch error {}", err).as_str());
                    }
                }
            }
        }
    }

    // Call conclusions
    for ba in actions.iter_mut() {
        (**ba).conclusion(&mut writer, &options);
    }

    let duration = start.elapsed();

    fn s(n: i32) -> &'static str {
        if n > 1 { "s" } else { "" }
    }

    if options.verbose {
        let mut msg = String::new();

        if files_count + dirs_count == 0 {
            msg.push_str("No match");
        } else {
            if options.search_files {
                if !msg.is_empty() {
                    msg.push_str(", ");
                }
                msg.push_str(format!("{files_count} file{}", s(files_count)).as_str());
            }
            if options.search_dirs {
                if !msg.is_empty() {
                    msg.push_str(", ");
                }
                msg.push_str(format!("{dirs_count} dir{}", s(dirs_count)).as_str());
            }
        }

        if errs_count > 0 {
            if !msg.is_empty() {
                msg.push_str(", ");
            }
            msg.push_str(format!("{errs_count} error{}", s(errs_count)).as_str());
        }
        logln(&mut writer, format!("\n{msg} found in {:.3}s", duration.as_secs_f64()).as_str());
    }
}
