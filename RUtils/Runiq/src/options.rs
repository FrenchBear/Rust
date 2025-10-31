// runiq - Module options
// Options processing
//
// 2025-10-31   PV      First version

// Application imports
use crate::*;

// Standard library imports
use std::error::Error;

// External crates imports
use getopt::Opt;
use mymarkup::MyMarkup;

// Program main output
#[derive(Debug, Default)]
pub enum Output {
    #[default]
    Unique, // Show unique lines (default)
    Repeated,    // Show repeated lines (one copy only)
    AllRepeated, // Show repeated lines (all lines)
}

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub ignore_case: bool,
    pub output: Output,
    pub verbose: bool,
}

impl Options {
    fn header() {
        println!("{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}");
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄|⦃-??⦄] [-i] [-⦃u⦄|-⦃d⦄-⦃D⦄] [-⦃v⦄]

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄|⦃-??⦄   ¬Show advanced usage notes
⦃-i⦄       ¬Ignore case (default: case sensitive)
⦃-u⦄       ¬Output: unique lines only
⦃-d⦄       ¬Output: duplicate lines only, one copy only
⦃-D⦄       ¬Output: duplicate lines only, all copies
⦃-v⦄       ¬Verbose output";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        println!("Copyright ©2025 Pierre Violent");
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- getopt: {}", env!("DEP_GETOPT_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

Case insensitive uses to_lowercase() before comparison, this may not be totally correct with some Unicode characters.
There is no attempt to normalize or denormalize strings before comparison
Empty lines are not filtered out (an option could be added for that in the future)
Lines are not stripped (besides end-of-line characters), spaces at the end of a line are significant";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    pub fn new() -> Result<Options, Box<dyn Error>> {
        let args: Vec<String> = std::env::args().collect();
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
        let mut opts = getopt::Parser::new(&args, "h?iudDv");

        let mut optput_selected = false;
        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('i', None) => {
                        options.ignore_case = true;
                    }

                    Opt('u', None) => {
                        if optput_selected {
                            return Err("Onpy one option among -u, -d or -D can be selected".into());
                        }
                        options.output = Output::Unique;
                        optput_selected = true;
                    }

                    Opt('d', None) => {
                        if optput_selected {
                            return Err("Onpy one option among -u, -d or -D can be selected".into());
                        }
                        options.output = Output::Repeated;
                        optput_selected = true;
                    }

                    Opt('D', None) => {
                        if optput_selected {
                            return Err("Onpy one option among -u, -d or -D can be selected".into());
                        }
                        options.output = Output::AllRepeated;
                        optput_selected = true;
                    }

                    Opt('v', None) => {
                        options.verbose = true;
                    }

                    _ => unreachable!(),
                },
            }
        }

        if opts.index() < args.len() {
            let arg = &args[opts.index()];
            return Err(format!("Invalid/unsupported option {}", arg).into());
        }

        Ok(options)
    }
}
