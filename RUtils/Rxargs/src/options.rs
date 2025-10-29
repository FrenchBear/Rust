// rwc - Module options
// Options processing
//
// 2025-04-21   PV      First version
// 2025-05-04   PV      Use MyMarkup crate to format usage and extended help
// 2025-07-10   PV      Use APP_DESCRIPTION variable

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
    pub autorecurse: bool,
    pub show_only_total: bool,
    pub verbose: bool,
}

impl Options {
    fn header() {
        println!(
            "{APP_NAME} {APP_VERSION}\n\
            {APP_DESCRIPTION}"
        );
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄|⦃-??⦄] [⦃-a+⦄|⦃-a-⦄] [-⦃t⦄] [-⦃v⦄] [⟨source⟩...]

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄|⦃-??⦄   ¬Show advanced usage notes
⦃-a+|-a-⦄  ¬Enable (default) or disable glob autorecurse mode (see extended usage)
⦃-t⦄       ¬Only show total line
⦃-v⦄       ¬Verbose output
⟨source⟩   ¬File or directory to search, glob syntax supported (see extended usage). Without source, search stdin.";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        println!("Copyright ©2025 Pierre Violent");
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        println!("- MyGlob: {}", MyGlobSearch::version());
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- TextAutoDecode: {}", TextAutoDecode::version());
        println!("- getopt: {}", env!("DEP_GETOPT_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

The four numerical fields report lines, words, characters and bytes counts. For UTF-8 or UTF-16 encoded files, a character is a Unicode codepoint, so bytes and characters counts may be different. Characters count neither include line terminators, nor BOM if present. Bytes count is the total file size as reported by the operating system, including line terminators and BOM if present.

Words are series of character(s) separated by space(s), spaces are either ASCII 9 (tab) or 32 (regular space).  Unicode \"fancy spaces\" are not considered.

Lines end with ⟦\\r⟧, ⟦\\n⟧ or ⟦\\r\\n⟧. If the last line of the file ends with such termination character, an extra empty line is counted.";

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

        let mut options = Options {
            autorecurse: true,
            ..Default::default()
        };
        let mut opts = getopt::Parser::new(&args, "h?tva:");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('a', attr) => match attr.unwrap().as_str() {
                        "+" => options.autorecurse = true,
                        "-" => options.autorecurse = false,
                        _ => return Err("Only -a+ and -a- (enable/disable autorecurse) are supported".into()),
                    },

                    Opt('t', None) => {
                        options.show_only_total = true;
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

        Ok(options)
    }
}
