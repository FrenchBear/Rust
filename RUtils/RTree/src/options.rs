// rtree - Module options
// Options processing
//
// 2025-04-05   PV      First version
// 2025-06-25   PV      Option -h renamex -a, and ctually parsed...

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
    pub showall: bool,
    pub verbose: bool,
}

impl Options {
    fn header() {
        println!(
            "{APP_NAME} {APP_VERSION}\n\
            Visual directory structure in Rust"
        );
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄] [-⦃a⦄] [-⦃v⦄] [⟨dir⟩]

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃-a⦄       ¬Show all directories, including hidden directories and directories starting with a dot
⦃-v⦄       ¬Verbose output
⟨dir⟩      ¬Starting directory";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    pub fn new() -> Result<Options, Box<dyn Error>> {
        let mut args: Vec<String> = std::env::args().collect();
        if args.len() > 1 && (args[1] == "?" || args[1].to_lowercase() == "help") {
                Self::usage();
                return Err("".into());
            }

        let mut options = Options { ..Default::default() };
        let mut opts = getopt::Parser::new(&args, "h?av");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('a', None) => {
                        options.showall = true;
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

            options.source = Some( arg.clone());
        }

        Ok(options)
    }
}
