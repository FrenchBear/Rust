// rxargs - Module options
// Options processing
//
// 2025-10-30   PV      First version

// Application imports
use crate::*;

// Standard library imports
use std::{arch::x86_64::CpuidResult, error::Error};

// External crates imports
use getopt::Opt;
use mymarkup::MyMarkup;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub ctr: CommandToRun,
    pub input_file: Option<String>,
    pub group_args: bool,
    pub verbose: bool,
}

impl Options {
    fn header() {
        println!(
            "{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}"
        );
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄|⦃-??⦄] [-⦃1⦄] [-⦃a⦄ ⟨file⟩] [-⦃v⦄] ⟨command⟩

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄   ¬Show this message
⦃??⦄|⦃-??⦄    ¬Show advanced usage notes
⦃-1⦄        ¬Group arguments and execute one instance per group of arguments length <= 7800 characters
[-⦃a⦄ ⟨file⟩] ¬Read arguments from ⟨file⟩ instead of standard input
⦃-v⦄        ¬Verbose output, print the command line on the standard error output before executing it and show final stats
⟨command⟩   ¬Command to execute, {} is replaced by auto-quoted arguments (or added at the end without {})";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        println!("Copyright ©2025 Pierre Violent");
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- TextAutoDecode: {}", TextAutoDecode::version());
        println!("- getopt: {}", env!("DEP_GETOPT_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

Command starts at the first argument that does not start with - so a command name cannot start with -
When reading arguments from a file using -⦃a⦄ option, text format is automally detected and non-text files are rejected.
When reading stdin, it's supposed to be valid ASCII or UTF-8 text, other text encoding and non-text input are not detected and rejected: garbage in, garbage out!";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
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

        let mut opts = getopt::Parser::new(&args, "h?1va:");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('1', None) => {
                        options.group_args = true;
                    }

                    Opt('a', attr) => options.input_file = attr,

                    Opt('v', None) => {
                        options.verbose = true;
                    }

                    _ => unreachable!(),
                },
            }
        }

        // Process extra arguments, the command itself
        let mut placeholder_found = false;

        let mut ctrargs: Vec<String> = Vec::new();
        for arg in args.split_off(opts.index()) {
            // For now we just support {}, but in the future, maybe variants (basename, to lowercase, ...)
            // Will be updated when needed
            if arg.contains("{}") {
                placeholder_found = true;
            }
            ctrargs.push(arg);
        }

        if !placeholder_found {
            if options.group_args && ctrargs.is_empty() {
                return Err("With option -g, command to execute is required".into());
            }
            ctrargs.push("{}".into());
        }

        options.ctr.command = ctrargs[0].clone();
        options.ctr.args = ctrargs[1..].to_vec();

        Ok(options)
    }
}
