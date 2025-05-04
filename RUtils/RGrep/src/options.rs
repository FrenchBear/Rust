// rgrep options.rs
// Process options parsing
//
// 2025-05-04   PV      Moved to a separate module; use MyMarkup for formatting

// Application imports
use crate::*;

// External crates imports
use mymarkup::MyMarkup;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub pattern: String,
    pub sources: Vec<String>,
    pub ignore_case: bool,
    pub whole_word: bool,
    pub fixed_string: bool,
    pub autorecurse: bool,
    pub show_path: bool,
    pub out_level: u8, // 0: normal output, 1: (-l) matching filenames only, 2: (-c) filenames and matching lines count, 3: (-c -l) only matching filenames and matching lines count
    pub verbose: u8,
}

impl Options {
    fn header() {
        println!(
            "{APP_NAME} {APP_VERSION}\n\
            Simplified grep in Rust"
        );
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄] [⦃-i⦄] [⦃-w⦄] [⦃-F⦄] [⦃-a+⦄|⦃-a-⦄] [⦃-v⦄] [⦃-c⦄] [⦃-l⦄] ⟨pattern⟩ [⟨source⟩...]

⌊Options:⌋
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄       ¬Show advanced usage notes
⦃-v⦄       ¬Verbose output
⦃-i⦄       ¬Ignore case during search
⦃-w⦄       ¬Whole word search
⦃-F⦄       ¬Fixed string search (no regexp interpretation), also for patterns starting with - ? or help
⦃-a+⦄|⦃-a-⦄  ¬Enable (default) or disable glob autorecurse mode (see extended usage)
⦃-c⦄       ¬Suppress normal output, show count of matching lines for each file
⦃-l⦄       ¬Suppress normal output, show matching file names only
⟨pattern⟩  ¬Regular expression to search
⟨source⟩   ¬File or folder where to search, glob syntax supported. Without source, search stdin";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        let text =
"Copyright ©2025 Pierre Violent

⟪⌊Advanced usage notes⌋⟫

Options ⦃-c⦄ (show count of matching lines) and ⦃-l⦄ (show matching file names only) can be used together to show matching lines count only for matching files.
Put special characters such as ⟦.⟧, ⟦*⟧ or ⟦?⟧ between brackets such as ⟦[.]⟧, ⟦[*]⟧ or ⟦[?]⟧ to search them as is.
To search for ⟦[⟧ or ⟦]⟧, use ⟦[\\[]⟧ or ⟦[\\]]⟧.
To search for a string containing double quotes, surround string by double quotes, and double individual double quotes inside. To search for ⟦\"msg\"⟧: {APP_NAME} ⟦\"\"\"msg\"\"\"⟧ ⟦C:\\Sources\\**\\*.rs⟧
To search for the string help, use option ⦃-F⦄: {APP_NAME} ⦃-F⦄ help ⟦C:\\Sources\\**\\*.rs⟧";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
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
        let mut opts = getopt::Parser::new(&args, "h?12iwFra:vcl");

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

                    Opt('w', None) => {
                        options.whole_word = true;
                    }

                    Opt('F', None) => {
                        options.fixed_string = true;
                    }

                    Opt('r', None) => {
                        options.autorecurse = true;
                    }
                    Opt('a', attr) => match attr.unwrap().as_str() {
                        "+" => options.autorecurse = true,
                        "-" => options.autorecurse = false,
                        _ => return Err("Only -a+ and -a- (enable/disable autorecurse) are supported".into()),
                    },


                    Opt('l', None) => {
                        options.out_level |= 1;
                    }

                    Opt('c', None) => {
                        options.out_level |= 2;
                    }

                    Opt('v', None) => {
                        options.verbose += 1;
                    }

                    _ => unreachable!(),
                },
            }
        }

        // Check for extra argument
        for arg in args.split_off(opts.index()) {
            // Don't check ? or help other than in first position, otherwise 'rgrep -F help source' will not search for word help

            if arg.starts_with("-") {
                return Err(format!("Invalid/unsupported option {}", arg).into());
            }

            if options.pattern.is_empty() {
                options.pattern = arg;
            } else {
                options.sources.push(arg);
            }
        }

        if options.pattern.is_empty() {
            Self::header();
            eprintln!("\nNo pattern specified.\nUse {APP_NAME} ? to show options or {APP_NAME} ?? for advanced usage notes.");
            return Err("".into());
        }

        Ok(options)
    }
}
