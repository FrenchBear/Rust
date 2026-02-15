// rtree - Module options
// Options processing
//
// 2025-04-05   PV      First version
// 2025-06-25   PV      Option -h renamex -a, and ctually parsed...
// 2025-07-25   PV      Option -d, option -A

// Application imports
use crate::*;

// Standard library imports
use std::error::Error;

// External crates imports
use getopt::Opt;
use mymarkup::MyMarkup;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub source: Option<String>,
    pub show_hidden: bool,
    pub show_hidden_and_system: bool,
    pub verbose: bool,
    pub maxdepth: u32,
}

impl Options {
    fn header() {
        println!("{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}");
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄|⦃-??⦄] [-⦃a⦄|-⦃A⦄] [⦃-d⦄ ⟨max_depth⟩] [-⦃v⦄] [⟨dir⟩]

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄      ¬Show this message
⦃??⦄|⦃-??⦄       ¬Show advanced usage notes
⦃-a⦄           ¬Show hidden directories and directories starting with a dot
⦃-A⦄           ¬Show system+hidden directories and hidden directories starting with a dollar sign
⦃-d⦄ ⟨max_depth⟩ ¬Limits recursion to max_depth folders, default is 0 meaning no limitation
⦃-v⦄           ¬Verbose output
⟨dir⟩          ¬Starting directory";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        println!("Copyright ©2025-2026 Pierre Violent");
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- getopt: {}", env!("DEP_GETOPT_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

By default, hidden folders are not shown.
Option ⦃-a⦄ shows hidden folders, that is, folders with file attribute H (Windows, Hidden) such as ⟦C:\\ProgramData⟧ or name starting with a . such as ⟦.git⟧.
Option ⦃-A⦄ (Windows only) shows system hidden folders, folders with file attribute H and S (Windows, Hidden+System) such as ⟦C:\\Recovery⟧ or hidden folders having a name starting with a $ such as ⟦C:\\$SysReset⟧.

On Windows, folders are sorted by default using File Explorer sorting rules, use option ⦃-s 2⦄ to sort folders using case folding. On Linux, folders are always sorted using case folding.

When recursion depth is limited using option ⦃-d⦄, \"...\" at the end of the folder means that there are unexplored subfolders.

Regardless of recursion depth limitation, \"... ?\" at the end of a folder means that folder content access is denied, so it is unknown if there are subfolders or not.

Option ⦃-v⦄ show small statistics at the end of tree.";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    pub fn new() -> Result<Options, Box<dyn Error>> {
        let mut args: Vec<String> = std::env::args().collect();

        if args.len() > 1 {
            if args[1] == "?" || args[1] == "-?" || args[1] == "/?" || args[1].to_lowercase() == "help" || args[1].to_lowercase() == "-help" || args[1].to_lowercase() == "/help" {
                Self::usage();
                return Err("".into());
            }

            if args[1] == "??" || args[1] == "-??" || args[1] == "/??" || args[1].to_lowercase() == "--help" {
                Self::extended_usage();
                return Err("".into());
            }
        }

        let mut options = Options { ..Default::default() };
        let mut opts = getopt::Parser::new(&args, "h?aAvd:");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('a', None) => {
                        options.show_hidden = true;
                    }

                    Opt('d', Some(arg)) => match arg.parse::<u32>() {
                        Ok(n) => {
                            options.maxdepth = n;
                        }
                        Err(_) => {
                            return Err("maxdepth argument must be an integer >= 0".into());
                        }
                    },

                    Opt('A', None) => {
                        options.show_hidden_and_system = true;
                    }

                    Opt('v', None) => {
                        options.verbose = true;
                    }

                    _ => unreachable!(),
                },
            }
        }

        // Check for extra argument
        for arg in args.split_off(opts.index()) {
            if arg.starts_with("-") {
                return Err(format!("Invalid/unsupported option {}", arg).into());
            }

            if options.source.is_some() {
                return Err(format!("Invalid argument {}, only one starting directory can be specified.", arg).into());
            }

            options.source = Some(arg.clone());
        }

        // show_hidden_and_system implies show_hidden
        if options.show_hidden_and_system {
            options.show_hidden = true
        }

        Ok(options)
    }
}
