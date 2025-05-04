// rgrep options.rs
// Process options parsing
//
// 2025-05-04   PV      Moved to a separate module; use MyMarkup for formatting

use super::*;

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
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄] [⦃-i⦄] [⦃-w⦄] [⦃-F⦄] [⦃-r⦄] [⦃-v⦄] [⦃-c⦄] [⦃-l⦄] pattern [source...]
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄       ¬Show advanced usage notes
⦃-i⦄       ¬Ignore case during search
⦃-w⦄       ¬Whole word search
⦃-F⦄       ¬Fixed string search (no regexp interpretation), also for patterns starting with - ? or help
⦃-r⦄       ¬Use autorecurse, see advanced help
⦃-c⦄       ¬Suppress normal output, show count of matching lines for each file
⦃-l⦄       ¬Suppress normal output, show matching file names only
⦃-v⦄       ¬Verbose output
pattern  ¬Regular expression to search
source   ¬File or folder where to search, glob syntax supported. Without source, search stdin";
        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn extended_usage() {
        Options::header();
        let width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80usize
        };
        let text =
"Copyright ©2025 Pierre Violent

⟪⌊Advanced usage notes⌋⟫

Options ⦃-c⦄ (show count of matching lines) and ⦃-l⦄ (show matching file names only) can be used together to show matching lines count only for matching files.
Put special characters such as ⟦.⟧, ⟦*⟧ or ⟦?⟧ between brackets such as ⟦[.]⟧, ⟦[*]⟧ or ⟦[?]⟧ to search them as is.
To search for ⟦[⟧ or ⟦]⟧, use ⟦[\\[]⟧ or ⟦[\\]]⟧.
To search for a string containing double quotes, surround string by double quotes, and double individual double quotes inside. To search for ⟦\"msg\"⟧: rgrep ⟦\"\"\"msg\"\"\"⟧ ⟦C:\\Sources\\**\\*.rs⟧
To search for the string help, use option ⦃-F⦄: rgrep ⦃-F⦄ help ⟦C:\\Sources\\**\\*.rs⟧";

        MyMarkup::render_markup(text);
        MyMarkup::render_markup(MyGlobSearch::glob_syntax());
    }

    fn format_text(text: &str, width: usize) -> String {
        let mut s = String::new();
        for line in text.split('\n') {
            if !s.is_empty() {
                s.push('\n');
            }
            s.push_str(Self::format_line(line, width).as_str());
        }
        s
    }

    fn format_line(line: &str, width: usize) -> String {
        let mut result = String::new();
        let mut current_line_length = 0;

        let left_margin = if line.starts_with('•') { "  " } else { "" };

        for word in line.split_whitespace() {
            let word_length = word.len();

            if current_line_length + word_length < width {
                if !result.is_empty() {
                    result.push(' ');
                    current_line_length += 1; // Add space
                }
                result.push_str(word);
                current_line_length += word_length;
            } else {
                if !result.is_empty() {
                    result.push('\n');
                    current_line_length = if !left_margin.is_empty() {
                        result.push_str(left_margin);
                        2
                    } else {
                        0
                    };
                }
                result.push_str(word);
                current_line_length += word_length;
            }
        }
        result
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
        let mut opts = getopt::Parser::new(&args, "h?12iwFrvcl");

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
