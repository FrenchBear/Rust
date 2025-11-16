// rcat - Module options
// Options processing
//
// 2025-10-24   PV      First version
// 2025-11-16   PV      Options -a+/-a-, -d

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
    pub number_lines: bool, // Unused for now
    pub autorecurse: bool,
    pub debug: bool,
    pub verbose: bool,
}

impl Options {
    fn header() {
        println!("{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}");
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄|⦃-??⦄] [-⦃n⦄] [-⦃v⦄] [-⦃d⦄] [⦃-a+⦄|⦃-a-⦄] [⟨source⟩...]

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄|⦃-??⦄   ¬Show advanced usage notes
⦃-n⦄       ¬Number all output lines (not implemented yet)
⦃-v⦄       ¬Verbose output
⦃-a+⦄|⦃-a-⦄  ¬Enable (default) or disable glob autorecurse mode (see extended usage)
⟨source⟩   ¬Files or directories to read (globbing supported). Without source, read stdin";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        println!("Copyright ©2025 Pierre Violent");
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        println!("- MyGlob: {}", MyGlobSearch::version());
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- getopt: {}", env!("DEP_GETOPT_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

⌊Advanced options⌋:
⦃-d⦄       ¬Debug mode, show internal dev informations";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
        println!();
        MyMarkup::render_markup(MyGlobSearch::glob_syntax());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    pub fn new() -> Result<Options, Box<dyn Error>> {
        let mut args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            if args[1] == "?" || args[1].to_lowercase() == "help" {
                Self::usage();
                return Err("".into());
            }

            if args[1] == "??" || args[1] == "-??" {
                Self::extended_usage();
                return Err("".into());
            }
        }

        let mut options = Options { ..Default::default() };
        let mut opts = getopt::Parser::new(&args, "h?nva:");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('n', None) => {
                        options.number_lines = true;
                    }

                    Opt('v', None) => {
                        options.verbose = true;
                    }

                    Opt('d', None) => {
                        options.debug = true;
                    }

                    Opt('a', arg) => {
                        if let Some(opt) = arg {
                            if opt == "+" {
                                options.autorecurse = true;
                            } else if opt == "-" {
                                options.autorecurse = false;
                            } else {
                                return Err(format!("Invalid option {}", opt).into());
                            }
                        }
                        options.number_lines = true;
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
