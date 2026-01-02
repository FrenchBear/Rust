// rdir - Module options
// Options processing
//
// 2025-10-24   PV      First version

// Application imports
use crate::*;

// Standard library imports
use std::error::Error;

// External crates imports
use getopt::Opt;
use myglob::MyGlobSearch;
use mymarkup::MyMarkup;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub sources: Vec<String>,
    pub show_link_target_info: bool,
    pub autorecurse: bool,
    pub verbose: bool,
}

impl Options {
    fn header() {
        println!("{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}");
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄|⦃-??⦄] [-⦃l⦄] [-⦃s⦄] [-⦃v⦄] ⟨source⟩...

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄|⦃-??⦄   ¬Show advanced usage notes
⦃-s⦄       ¬Displays files in specified directory and all subdirectories (glob autorecurse)
⦃-l⦄       ¬Show information of links target instead of link
⦃-v⦄       ¬Verbose output
⟨source⟩   ¬Files or directories to analyze";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        println!("Copyright ©2025 Pierre Violent");
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- MyGlob: {}", MyGlobSearch::version());
        println!("- getopt: {}", env!("DEP_GETOPT_VERSION"));
        println!("- chrono: {}", env!("DEP_CHRONO_VERSION"));
        println!("- numfmt: {}", env!("DEP_NUMFMT_VERSION"));
        println!();
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
        let mut opts = getopt::Parser::new(&args, "h?lsv");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('l', None) => {
                        options.show_link_target_info = true;
                    }

                    Opt('s', None) => {
                        options.autorecurse = true;
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

            options.sources.push(arg);
        }

        // For dev/debug
        //options.sources.push(r"\\wsl.localhost\Ubuntu-22.04\tmp\jl_7aRYRShS5y.pdf".to_string());

        if options.sources.is_empty() {
            return Err("No source provided".into());
        }

        Ok(options)
    }
}
