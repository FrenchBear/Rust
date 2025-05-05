// RNormalizeDates: module options
//
// 2025-04-14   PV      Extracted to a separate file
// 2025-05-04   PV      Use MyMarkup crate to format usage and extended help

// Application imports
use crate::*;

// Standard library imports
use std::error::Error;

// External crates imports
use mymarkup::MyMarkup;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub sources: Vec<String>,
    pub segment: usize,
    pub final_pause: bool,
    pub no_action: bool,
    pub verbose: bool,
}

impl Options {
    fn header() {
        println!(
            "{APP_NAME} {APP_VERSION}\n\
            Rust version of NormalizeDates, Normalizes dates in filenames, replace 'January 2020' by '2020-01'"
        );
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄] [⦃-n⦄] [⦃-p⦄] [⦃-v⦄] [⦃-s #⦄] ⟨source⟩...

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄       ¬Show advanced usage notes
⦃-n⦄       ¬Do not actually rename (no action)
⦃-p⦄       ¬Final pause
⦃-v⦄       ¬Verbose output
⦃-s #⦄     ¬Only process segment # (starting at 1) delimited by ' - '
⟨source⟩   ¬folder containing PDF files (and recurse) or simple file, default: ⟦C:\\Downloads\\A_Trier\\!A_Trier_Revues\\*.pdf⟧";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        let text = "Copyright ©2025 Pierre Violent

⟪⌊Advanced usage notes⌋⟫

Without argument, default folder is C:\\Downloads\\A_Trier\\!A_Trier_Revues\\**\\*.pdf";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
        MyMarkup::render_markup(MyGlobSearch::glob_syntax());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    pub fn new() -> Result<Options, Box<dyn Error>> {
        let mut args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            if args[1].to_lowercase() == "help" {
                Self::usage();
                return Err("".into());
            }

            if args[1] == "??" || args[1] == "-??" {
                Self::extended_usage();
                return Err("".into());
            }
        }

        let mut options = Options { ..Default::default() };
        let mut opts = getopt::Parser::new(&args, "h?npvs:");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('n', None) => {
                        options.no_action = true;
                    }

                    Opt('p', None) => {
                        options.final_pause = true;
                    }

                    Opt('v', None) => {
                        options.verbose = true;
                    }

                    Opt('s', Some(arg)) => {
                        if options.segment > 0 {
                            return Err("Option -s # can only be used once".into());
                        }
                        let segres = arg.parse::<usize>();
                        if let Ok(s) = segres {
                            if (1..=5).contains(&s) {
                                options.segment = s;
                                continue;
                            }
                        }
                        return Err("Option -s requires a numerical argument in [1..5]".into());
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

        if options.sources.is_empty() {
            options.sources.push(r"C:\Downloads\A_Trier\!A_Trier_Revues\*.pdf".into());
        }

        Ok(options)
    }
}
