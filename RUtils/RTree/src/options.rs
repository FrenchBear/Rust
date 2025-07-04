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
        println!(
            "{APP_NAME} {APP_VERSION}\n\
            Visual directory structure in Rust"
        );
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄] [-⦃a⦄|-⦃A⦄] [⦃-d⦄ ⟨max_depth⟩] [-⦃v⦄] [⟨dir⟩]

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄      ¬Show this message
⦃-a⦄           ¬Show hidden directories and directories starting with a dot
⦃-A⦄           ¬Show system+hidden directories and directories starting with a dollar sign
⦃-d⦄ ⟨max_depth⟩ ¬Limits recursion to max_depth folders, default is 0 meaning no limitation
⦃-v⦄           ¬Verbose output
⟨dir⟩          ¬Starting directory";

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
                            return Err(format!("maxdepth argument must be an integer >= 0").into());
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
