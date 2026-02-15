// recycle options module
// Processing command line arguments
//
// 2025-10-22   PV      Extracted from main.rs; Added dependencies info in extended help

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
    pub sources: Vec<String>,
    pub no_action: bool,
    pub verbose: bool,
    pub silent: bool,
}

impl Options {
    fn header() {
        println!("{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}");
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
        println!("Copyright ©2025-2026 Pierre Violent");
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        //println!("- MyGlob: {}", MyGlobSearch::version());
        println!("- MyGlob: {}", MyGlobSearch::version());
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- Logging: {}", logging::version());
        println!("- getopt: {}", env!("DEP_GETOPT_VERSION"));
        println!("- trash: {}", env!("DEP_TRASH_VERSION"));
        println!("- windows: {}", env!("DEP_WINDOWS_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

Only local files (local drive or attached USB drive) support trash.
Network files can't be deleted to recycle bin, so they can't be removed with this command (contrary to PDEL that will remove remote files permanently).\n";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
        MyMarkup::render_markup(MyGlobSearch::glob_syntax());
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
            if arg.starts_with("-") {
                return Err(format!("Invalid/unsupported option {}", arg).into());
            }

            options.sources.push(arg);
        }

        Ok(options)
    }
}
