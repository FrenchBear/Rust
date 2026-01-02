// rtt - Module options
// Options processing
//
// 2025-04-21   PV      First version
// 2025-05-04   PV      Use MyMarkup crate to format usage and extended help

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
    pub autorecurse: bool,
    pub show_only_warnings: bool,
    pub verbose: bool,
}

impl Options {
    fn header() {
        println!("{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}");
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄|⦃-??⦄] [⦃-a+⦄|⦃-a-⦄] [⦃-w⦄] [⦃-v⦄] [⟨source⟩...]

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄|⦃-??⦄   ¬Show advanced usage notes
⦃-a+⦄|⦃-a-⦄  ¬Enable (default) or disable glob autorecurse mode (see extended usage)
⦃-w⦄       ¬Only show warnings
⦃-v⦄       ¬Verbose output
⟨source⟩   ¬File or directory to search, glob syntax supported. Without source, search stdin.";

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
        println!("- coloredt: {}", env!("DEP_COLORED_VERSION"));
        println!("- tempfile: {}", env!("DEP_TEMPFILE_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

Counts include with and without BOM variants.
8-bit text files are likely Windows 1252/Latin-1/ANSI or OEM 850/OEM 437, there is no detailed analysis.

⌊EOL styles⌋:
- ¬⟪Windows⟫: ⟦\\r\\n⟧
- ¬⟪Unix⟫: ⟦\\n⟧
- ¬⟪Mac⟫: ⟦\\r⟧

⌊Warnings report⌋:
- ¬Empty files
- ¬Source text files (based on extension) that should contain text, but with unrecognized content
- ¬UTF-8 files with BOM
- ¬UTF-16 files without BOM
- ¬Different encodings for a given file type (extension) in a directory
- ¬Mixed EOL styles in a file
- ¬Different EOL styles for a given file type (extension) in a directory";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
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

        let mut options = Options {
            autorecurse: true,
            ..Default::default()
        };
        let mut opts = getopt::Parser::new(&args, "h?fwva:");

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

                    Opt('w', None) => {
                        options.show_only_warnings = true;
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
