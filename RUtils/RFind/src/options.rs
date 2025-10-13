// rfind - module options
// Process command line options
//
// 2025-04-22   PV      Moved to a separate file
// 2025-05-03	PV      Option -name
// 2025-05-04   PV      Use MyMarkup for formatting
// 2025-05-05   PV      is_option for Linux compatibility
// 2025-07-13 	PV 		Option -nop
// 2025-09-06 	PV 		Option -maxdepth n
// 2025-09-15 	PV 		Option -dbg for debugging and -log to write log file
// 2025-10-13 	PV 		Option -exec, -xargs and struct CommandToRun

// Application imports
use crate::*;

// Standard library imports
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;

// External crates imports
use mymarkup::MyMarkup;

#[derive(Debug, Default, Clone)]
pub struct CommandToRun {
    pub command: String,
    pub args: Vec<String>,
}

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub sources: Vec<String>,
    pub actions_names: HashSet<&'static str>,
    pub exec_commands: Vec<CommandToRun>,
    pub xargs_commands: Vec<CommandToRun>,
    pub search_files: bool,
    pub search_dirs: bool,
    pub names: Vec<String>,
    pub maxdepth: usize,
    pub isempty: bool,
    pub recycle: bool,
    pub autorecurse: bool,
    pub noaction: bool,
    pub verbose: bool,
    pub debug: bool,
    pub log: bool,
}

impl Options {
    fn header() {
        println!(
            "{APP_NAME} {APP_VERSION}\n\
            {APP_DESCRIPTION}"
        );
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄] [⦃-v⦄] [⦃-n⦄] [⦃-f⦄|⦃-type f⦄|⦃-d⦄|⦃-type d⦄] [⦃-e⦄|⦃-empty⦄] [⦃-r+⦄|⦃-r-⦄] [⦃-a+⦄|⦃-a-⦄] [⟨action⟩...] [⦃-name⦄ ⟨name⟩] [⦃-maxdepth⦄ ⟨n⟩] ⟨source⟩...

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄          ¬Show this message
⦃??⦄               ¬Show advanced usage notes
⦃-v⦄               ¬Verbose output
⦃-n⦄               ¬No action: display actions, but don't execute them
⦃-f⦄|⦃-type f⦄       ¬Search for files
⦃-d⦄|⦃-type d⦄       ¬Search for directories
⦃-e⦄|⦃-empty⦄        ¬Only find empty files or directories
⦃-r+⦄|⦃-r-⦄          ¬Delete to recycle bin (default) or delete forever; Recycle bin is not allowed on network sources
⦃-a+⦄|⦃-a-⦄          ¬Enable (default) or disable glob autorecurse mode (see extended usage)
⦃-name⦄ ⟨name⟩       ¬Append ⟦**/⟧⟨name⟩ to each source directory (compatibility with XFind/Search)
⦃-maxdepth⦄ ⟨n⟩      ¬Limit the recursion depth of ** segments, 1=One directory only, ... Default=0 is unlimited depth

⟨source⟩           ¬File or directory to search

⌊Actions⌋:
⦃-print⦄           ¬Default, print matching files names and dir names
⦃-dir⦄             ¬Variant of ⦃-print⦄, with last modification date and size
⦃-nop[rint]⦄       ¬Do nothing, useful to replace default action ⦃-print⦄ to count files and folders with option ⦃-v⦄
⦃-delete⦄          ¬Delete matching files
⦃-rmdir⦄           ¬Delete matching directories, whether empty or not
⦃-exec⦄ ⟨cmd⟩ [⦃;⦄]    ¬Execute command ⟨cmd⟩ for each path found, {} replaced by the path. A single semicolon marks the end of the command
⦃-xargs⦄ ⟨cmd⟩ [⦃;⦄]   ¬Execute command ⟨cmd⟩ at the end, {} replaced by all the paths found. A single semicolon marks the end of the command";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        println!("Copyright ©2025 Pierre Violent");
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        println!("- MyGlob: {}", MyGlobSearch::version());
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- Logging: {}", logging::version());
        println!("- trash: {}", env!("DEP_TRASH_VERSION"));
        println!("- chrono: {}", env!("DEP_CHRONO_VERSION"));
        println!("- num-format: {}", env!("DEP_NUM_FORMAT_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

⌊Advanced options⌋:
⦃-dbg⦄       ¬Debug mode, show internal dev informations
⦃-log⦄       ¬Write log file in temp folder

⌊Compatibility with XFind⌋:
- ¬Option ⦃-norecycle⦄ can be used instead of ⦃-r-⦄ to indicate to delete forever.
- ¬Option ⦃-name⦄ can be used to indicate a specific file name or pattern to search.";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
        println!();
        MyMarkup::render_markup(MyGlobSearch::glob_syntax());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    pub fn new() -> Result<Options, Box<dyn Error>> {
        let args: Vec<String> = std::env::args().collect();

        let mut options = Options {
            autorecurse: true,
            recycle: true,
            ..Default::default()
        };

        // Works with Windows and Linux
        fn is_option(arg: &str) -> bool {
            #[cfg(target_os = "windows")]
            {
                if arg.starts_with('/') {
                    return true;
                }
            }
            arg.starts_with('-')
        }

        // Since we have non-standard long options, don't use getopt for options processing but a manual loop
        let mut args_iter = args.iter();
        args_iter.next(); // Skip application executable
        while let Some(arg) = args_iter.next() {
            if is_option(arg) {
                // Options are case insensitive
                let arglc = arg[1..].to_lowercase();

                match &arglc[..] {
                    "?" | "h" | "help" | "-help" => {
                        Self::usage();
                        return Err("".into());
                    }

                    "??" => {
                        Self::extended_usage();
                        return Err("".into());
                    }

                    "v" => options.verbose = true,
                    "log" => options.log = true,
                    "dbg" => options.debug = true,
                    "n" => options.noaction = true,

                    "f" => options.search_files = true,
                    "d" => options.search_dirs = true,
                    "type" => {
                        if let Some(search_type) = args_iter.next() {
                            match search_type.to_lowercase().as_str() {
                                "f" => options.search_files = true,
                                "d" => options.search_dirs = true,
                                _ => return Err(format!("Invalid argument {search_type} for pption -type, valid arguments are f or d").into()),
                            }
                        } else {
                            return Err("Option -type requires an argument f or d".into());
                        }
                    }

                    "name" => {
                        if let Some(name) = args_iter.next() {
                            options.names.push(name.clone());
                        } else {
                            return Err("Option -name requires an argument".into());
                        }
                    }

                    "maxdepth" => {
                        if let Some(name) = args_iter.next() {
                            if name.parse::<usize>().is_err() {
                                return Err("Option -maxdepth requires a numeric argument".into());
                            }
                            options.maxdepth = name.parse::<usize>().unwrap();
                        } else {
                            return Err("Option -maxdepth requires an argument".into());
                        }
                    }

                    "e" | "empty" => options.isempty = true,

                    "r+" | "recycle" => options.recycle = true,
                    "r-" | "norecycle" => options.recycle = false,

                    "a+" => options.autorecurse = true,
                    "a-" => options.autorecurse = false,

                    "print" => {
                        options.actions_names.insert("print");
                    }
                    "dir" => {
                        options.actions_names.insert("dir");
                    }
                    "nop" | "noprint" => {
                        options.actions_names.insert("nop");
                    }
                    "rm" | "del" | "delete" => {
                        options.actions_names.insert("delete");
                    }
                    "rd" | "rmdir" => {
                        options.actions_names.insert("rmdir");
                    }

                    "exec" | "xargs" => {
                        let mut args: Vec<String> = Vec::new();
                        while let Some(arg) = args_iter.next() {
                            if arg == ";" {
                                break;
                            }
                            args.push(arg.clone());
                        }
                        if args.len() == 0 {
                            return Err(format!("Option -{arglc} requires an argument").into());
                        }

                        // For now, simplified analysis
                        let ctr = CommandToRun {
                            command: args[0].clone(),
                            args: args[1..].to_vec(),
                        };
                        if arglc == "exec" {
                            options.exec_commands.push(ctr);
                        } else {
                            options.xargs_commands.push(ctr);
                        }
                    }

                    _ => {
                        return Err(format!("Invalid/unsupported option {}", arg).into());
                    }
                }
            } else {
                // Non-option, some values are special
                match &arg.to_lowercase()[..] {
                    "?" | "h" | "help" => {
                        Self::usage();
                        return Err("".into());
                    }

                    "??" => {
                        Self::extended_usage();
                        return Err("".into());
                    }

                    // Everything else is considered as a source (a glob pattern), it will be validated later
                    _ => options.sources.push(arg.clone()),
                }
            }
        }

        // If neither filtering files or dirs has been requested, then we search for both
        if !options.search_dirs && !options.search_files {
            options.search_dirs = true;
            options.search_files = true;
        }

        // If no action is specified, then print action is default
        if options.actions_names.is_empty() && options.exec_commands.is_empty() && options.xargs_commands.is_empty() {
            options.actions_names.insert("print");
        }

        Ok(options)
    }
}
