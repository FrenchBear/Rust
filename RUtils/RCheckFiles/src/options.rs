// rcheckfiles options module
// Processing command line arguments
//
// 2025-10-15	PV      Refactoring, separated options module. Added extended options
// 2025-10-21	PV      Filtering on problem types
// 2025-10-21	PV      Specific type dex for double extension
// 2025-20-22   PV      Clippy review
// 2025-20-24   PV      Problem das for dashes confusables, and mex for mixed scripts

// Application imports
use crate::*;

// Standard library imports
use std::error::Error;
use std::fs;

// External crates imports
use mymarkup::MyMarkup;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub sources: Vec<String>,
    pub fixit: bool,
    pub yaml_output: bool,
    pub yaml_file: String,
    pub count_extensions: bool,
    pub report_types: HashSet<String>,
}

/// Checks if a path exists and is a file.
/// Returns `true` only if the path points to an existing file.
/// Returns `false` for directories, symlinks, or if the path doesn't exist.
pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_file()).unwrap_or(false)
}

/// Checks if a path exists and is a directory.
/// Returns `true` only if the path points to an existing directory.
/// Returns `false` for files, symlinks, or if the path doesn't exist.
pub fn dir_exists(path: &str) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_dir()).unwrap_or(false)
}

impl Options {
    fn header() {
        println!("{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}");
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄|⦃-??⦄] [⦃-p type[,type]...⦄] [⦃-f⦄] [⦃-y⦄] [⦃-F⦄ ⟨yamlfile⟩] [⦃-e⦄] ⟨source⟩...
⦃?⦄|⦃-?⦄|⦃-h⦄     ¬Show this message
⦃??⦄|⦃-??⦄      ¬Show advanced usage notes
⦃-p type[,type]...⦄ ¬Only report specific types. type: nnn|bra|apo|spc|car|sp2|lig|sba|ewd|dex
⦃-f⦄          ¬Automatic problems fixing
⦃-y⦄          ¬Yaml output
⦃-F⦄ ⟨yamlfile⟩ ¬Rename files using old/new fields of provided yaml file
⦃-e⦄          ¬Count extensions
⟨source⟩      ¬File or directory to analyze (note: glob pattern is not supported)

⌊Types⌋⟫:
nnn   ¬Non-normalized names
bra   ¬Bracket issue
spc   ¬Incorrect space
apo   ¬Incorrect apostrophe
das   ¬Incorrect dash
car   ¬Maybe incorrect char
sp2   ¬Double space
lig   ¬Ligatures
sba   ¬Space before/after
ewd   ¬Ends with dots
dex   ¬Double extension
mix   ¬Mixed scripts";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        println!("Copyright ©2025 Pierre Violent");
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- Logging: {}", logging::version());
        println!("- getopt: {}", env!("DEP_GETOPT_VERSION"));
        println!("- regex: {}", env!("DEP_REGEX_VERSION"));
        println!("- serde: {}", env!("DEP_SERDE_VERSION"));
        println!("- serde_yaml: {}", env!("DEP_SERDE_YAML_VERSION"));
        println!("- unicode-normalization: {}", env!("DEP_UNICODE_NORMALIZATION_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

Option ⦃-y⦄ generates yaml output, including extra non-yaml header and footer. If output is redirected to a file to be edited and later processed with -F option, don't forget to remove non-yaml parts.\n";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    pub fn new() -> Result<Options, Box<dyn Error>> {
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
        let mut opts = getopt::Parser::new(&args, "h?p:fyF:e");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('p', problems) => {
                        if problems.is_none() {
                            return Err("Option -p requires a list of problems as an argument".into());
                        }
                        for problem in problems.unwrap().split(',') {
                            let pb = problem.trim().to_lowercase();
                            if pb != "nnn"
                                && pb != "bra"
                                && pb != "spc"
                                && pb != "apo"
                                && pb != "das"
                                && pb != "car"
                                && pb != "sp2"
                                && pb != "lig"
                                && pb != "sba"
                                && pb != "ewd"
                                && pb != "dex"
                                && pb != "mix"
                            {
                                return Err(
                                    format!("Invalid problem type {}, must be one of nnn|bra|spc|apo|das|car|sp2|lig|sba|ewd|dex|mix", problem).into(),
                                );
                            }
                            if !options.report_types.contains(&pb) {
                                options.report_types.insert(pb);
                            }
                        }
                    }

                    Opt('f', None) => {
                        options.fixit = true;
                    }

                    Opt('e', None) => {
                        options.count_extensions = true;
                    }

                    Opt('y', None) => {
                        options.yaml_output = true;
                    }

                    Opt('F', yamlfile) => {
                        if yamlfile.is_none() {
                            return Err("Option -f requires about yaml file as an argument".into());
                        }
                        options.yaml_file = yamlfile.unwrap();
                        if !file_exists(&options.yaml_file) {
                            return Err(format!("Can't find yaml file {}", options.yaml_file).into());
                        }
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

        // Validate options
        let count_true = (options.yaml_output as u8) + (options.fixit as u8) + (!options.yaml_file.is_empty() as u8);
        if count_true > 1 {
            return Err("Options -y, -f and -F are exclusive".into());
        }
        if options.count_extensions && !options.yaml_file.is_empty() {
            return Err("Options -F and -e are exclusive".into());
        }

        if options.yaml_file.is_empty() {
            if options.sources.is_empty() {
                return Err("Without option -F, at least one source is required".into());
            }
        } else if !options.sources.is_empty() {
            return Err("With option -F, no source is allowed".into());
        }

        Ok(options)
    }
}
