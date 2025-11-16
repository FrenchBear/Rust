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
// 2025-10-17   PV      Option -yaml
// 2025-10-22   PV      Clippy review
// 2025-10-22   PV      links options, reorg usage message
// 2025-10-23   PV      no_glob_filtering
// 2025-10-23   PV      no_glob_filtering
// 2025-10-29   PV      -xargs replaced by -execg
// 2025-10-29   PV      Added {} final for -exec/-execg if there is no {} in command
// 2025-11-15   PV      -w to make actions -exec/-execg synchronous
// 2025-11-16   PV      Grouped all MyGlob options into mgclo: GlobCLOptions; Use MyGlob to parse these options

// Application imports
use crate::*;

// Standard library imports
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;

// External crates imports
use mymarkup::MyMarkup;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub sources: Vec<String>,
    pub actions_names: HashSet<&'static str>,
    pub filters_names: HashSet<&'static str>,
    pub exec_commands: Vec<CommandToRun>,
    pub execg_commands: Vec<CommandToRun>,
    pub search_files: bool,
    pub search_dirs: bool,
    pub names: Vec<String>,
    pub recycle: bool,
    pub mgclo: MyGlobCLOptions,
    pub noaction: bool,
    pub syncronous_exec: bool,
    pub verbose: bool,
    pub debug: bool,
    pub log: bool,
}

impl Options {
    fn header() {
        println!("{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}");
    }

    fn header_copyright() {
        Self::header();
        println!("Copyright ©2025 Pierre Violent");
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} [⟨option⟩...] [⟨filter⟩...] [⟨action⟩...] ⟨source⟩...

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃help⦄|⦃-help⦄  ¬Show this message
⦃??⦄|⦃-??⦄|⦃--help⦄    ¬Show advanced usage notes
⦃-v⦄               ¬Verbose output
⦃-w⦄               ¬Actions ⦃-exec⦄/⦃-execg⦄ are synchronous (wait for command execution to terminate before continuing), default is asynchronous
⦃-n⦄               ¬No action: display actions, but don't execute them
⦃-r+⦄|⦃-r-⦄          ¬Delete to recycle bin (default) or delete forever; Recycle bin is not allowed on network sources
⦃-glob opt⟩[,⟨opt⟩]⦄… ¬Globbing specific options (see extended help)

⟨source⟩           ¬File or directory to search (glob pattern)

⌊Filters⌋:
⦃-f⦄|⦃-type f⦄       ¬Search for files
⦃-d⦄|⦃-type d⦄       ¬Search for directories
⦃-e⦄|⦃-empty⦄        ¬Only find empty files or directories
⦃-ads⦄             ¬Select files with alternate data streams
⦃-adsx⦄            ¬Select files with alternate data streams of 2KB or more (typically ignore Zone.identification, AFP_Resource, ms-properties...)
⦃-name⦄ ⟨name⟩       ¬Append ⟦/**/⟨name⟩⟧ to each source directory (compatibility with XFind/Search)

⌊Actions⌋:
⦃-print⦄           ¬Default, print matching files names and dir names
⦃-dir⦄             ¬Variant of ⦃-print⦄, with last modification date and size
⦃-nop[rint]⦄       ¬Do nothing, useful to replace default action ⦃-print⦄ to count files and folders with option ⦃-v⦄
⦃-delete⦄          ¬Delete matching files
⦃-rmdir⦄           ¬Delete matching directories, whether empty or not
⦃-exec⦄ ⟨cmd⟩ [⦃;⦄]    ¬Execute command ⟨cmd⟩ for each path found, {} replaced by the path or added at the end. A single semicolon marks the end of the command
⦃-execg⦄ ⟨cmd⟩ [⦃;⦄]   ¬Execute grouped command ⟨cmd⟩ at the end, {} replaced by all the paths found or added at the end. A single semicolon marks the end of the command
⦃-yaml⦄            ¬Generate old/new yaml data for matches, to be edited and used by rcheckfiles -F";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Self::header_copyright();
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
⦃-dbg⦄       ¬Debug mode, show internal dev information
⦃-log⦄       ¬Write log file in temp folder\n\n"
            .to_string()
            + MyGlobCLOptions::options()
            + "
            
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

        // Debug
        // let args = vec![String::from("app.exe"), String::from(r"C:\Temp\T\*"), String::from(r"-execg"),  String::from(r"cmd"),  String::from(r"/c"),  String::from(r"type"),  String::from(r"{}")];

        let mut options = Options {
            recycle: true,
            mgclo: MyGlobCLOptions {
                autorecurse: true,
                link_mode: 1,
                ..Default::default()
            },
            search_files: false,
            search_dirs: false,
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

        // Special options processing
        if args.len() > 1 {
            if args[1] == "?" || args[1] == "-?" || args[1].to_lowercase() == "help" || args[1].to_lowercase() == "-help" {
                Self::usage();
                return Err("".into());
            }

            if args[1] == "??" || args[1] == "-??" || args[1].to_lowercase() == "--help" {
                Self::extended_usage();
                return Err("".into());
            }
        }

        fn dep(old: &str, new: &str) {
            println!("*** Warning: Deprecated option {old}, use -glob {new} instead")
        }

        // Since we have non-standard long options, don't use getopt for options processing but a manual loop
        let mut args_iter = args.iter();
        args_iter.next(); // Skip application executable
        while let Some(arg) = args_iter.next() {
            if is_option(arg) {
                // Options are case insensitive
                let arglc = arg[1..].to_lowercase();

                match &arglc[..] {
                    // "?" | "help" => {
                    //     Self::usage();
                    //     return Err("".into());
                    // }

                    // "??" | "-help" => {
                    //     Self::extended_usage();
                    //     return Err("".into());
                    // }
                    "v" => options.verbose = true,
                    "w" => options.syncronous_exec = true,
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

                    // -- MyGlob options

                    // New compact version
                    "glob" | "-glob" => {
                        if let Some(arg) = args_iter.next() {
                            options.mgclo.process_options(arg);
                        } else {
                            return Err("Option -glob requires an argument".into());
                        }
                    }

                    // Keep legacy glob options, but they are deprecated
                    "a+" => {
                        options.mgclo.autorecurse = true;
                        dep("-a+", "a+");
                    }
                    "a-" => {
                        options.mgclo.autorecurse = false;
                        dep("-a-", "a-");
                    }
                    "l0" => {
                        options.mgclo.link_mode = 0;
                        dep("-l0", "l0");
                    }
                    "l1" => {
                        options.mgclo.link_mode = 1;
                        dep("-l1", "l1");
                    }
                    "l2" => {
                        options.mgclo.link_mode = 2;
                        dep("-l2", "l2");
                    }

                    "maxdepth" => {
                        dep("-maxdepth n", "md n");
                        if let Some(name) = args_iter.next() {
                            if name.parse::<usize>().is_err() {
                                return Err("Option -maxdepth requires a numeric argument".into());
                            }
                            options.mgclo.max_depth = name.parse::<usize>().unwrap();
                        } else {
                            return Err("Option -maxdepth requires an argument".into());
                        }
                    }

                    "cs" | "cs+" => {
                        options.mgclo.case_sensitive = true;
                        dep("-cs", "cs");
                    }
                    "cs-" => {
                        options.mgclo.case_sensitive = false;
                        dep("-ci", "ci");
                    }

                    "ngf" => {
                        options.mgclo.no_glob_filtering = true;
                        dep("-ngf", "ngf");
                    }

                    // --
                    "e" | "empty" => {
                        options.filters_names.insert("empty");
                    }
                    "ads" => {
                        options.filters_names.insert("ads");
                    }
                    "adsx" => {
                        options.filters_names.insert("adsx");
                    }

                    "r+" | "recycle" => options.recycle = true,
                    "r-" | "norecycle" => options.recycle = false,

                    "print" => {
                        options.actions_names.insert("print");
                    }
                    "dir" => {
                        options.actions_names.insert("dir");
                    }
                    "yaml" => {
                        options.actions_names.insert("yaml");
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

                    "exec" | "execg" => {
                        let mut args: Vec<String> = Vec::new();
                        let mut placeholder_found = false;
                        // while let Some(arg) = args_iter.next() {     // Clippy suggested to replace this
                        for arg in args_iter.by_ref() {
                            if arg == ";" {
                                break;
                            }
                            // For now we just support {}, but in the future, maybe variants (basename, to lowercase, ...)
                            // Will be updated when needed
                            if arg.contains("{}") {
                                placeholder_found = true;
                            }
                            args.push(arg.clone());
                        }
                        if !placeholder_found {
                            if arglc == "exec" {
                                args.push("{}".into());
                            } else {
                                return Err("Command to execute is required for option execg".into());
                            }
                        }
                        // Since we add a {} at the end if no {} has been provided, error message is not needed
                        // Command can be empty, meaning, execute the command -- why not?
                        // if args.is_empty() {
                        //     return Err(format!("Option -{arglc} requires an argument").into());
                        // }

                        // For now, simplified analysis
                        let ctr = CommandToRun {
                            command: args[0].clone(),
                            args: args[1..].to_vec(),
                        };
                        if arglc == "exec" {
                            options.exec_commands.push(ctr);
                        } else {
                            options.execg_commands.push(ctr);
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
        if options.actions_names.is_empty() && options.exec_commands.is_empty() && options.execg_commands.is_empty() {
            options.actions_names.insert("print");
        }

        Ok(options)
    }
}
