// RNormalizeDates: module options
//
// 2025-04-14   PV      Extracted to a separate file

use std::error::Error;

use super::*;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub sources: Vec<String>,
    pub final_pause: bool,
    pub segment: u8,
    pub no_action: bool,
    pub verbose: bool,
}

impl Options {
    fn header() {
        eprintln!(
            "{APP_NAME} {APP_VERSION}\n\
            Rust version of NormalizeDates, Normalizes dates in filenames, replace 'January 2020' by '2020-01'"
        );
    }

    fn usage() {
        Options::header();
        eprintln!(
            "\nUsage: {APP_NAME} [?|-?|-h|??] [-n] [-p] [-s #] [-v] source...
?|-?|-h  Show this message
??       Show advanced usage notes
-n       Do not actually rename (no action)
-p       Final pause
-v       Verbose output
-s #     Only process segment # (starting at 1) delimited by ' - '
source   folder containing PDF files (and recurse) or simple file"
        );
    }

    fn extended_usage() {
        Options::header();
        let width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80usize
        };
        let text =
"Copyright ©2025 Pierre Violent\n
Advanced usage notes\n--------------------\n
Options -c (show count of matching lines) and -l (show matching file names only) can be used together to show matching lines count only for matching files.\n
Glob supports recursive search without using option, for instance, C:\\Development\\GitVSTS\\**\\Net[7-9]\\**\\*.cs\n
Only UTF-8, UTF-16 LE and Windows 1252 text files are currently supported, but automatic format detection using heuristics may not be always correct. Other formats are silently ignored.\n
Glob crate pattern nules (option -1):
•   ? matches any single character.
•   * matches any (possibly empty) sequence of characters.
•   ** matches the current directory and arbitrary subdirectories. To match files in arbitrary subdiretories, use **\\*. This sequence must form a single path component, so both **a and b** are invalid and will result in an error.
•   [...] matches any character inside the brackets. Character sequences can also specify ranges of characters, as ordered by Unicode, so e.g. [0-9] specifies any character between 0 and 9 inclusive. An unclosed bracket is invalid.
•   [!...] is the negation of [...], i.e. it matches any characters not in the brackets.
•   The metacharacters ?, *, [, ] can be matched by using brackets (e.g. [?]). When a ] occurs immediately following [ or [! then it is interpreted as being part of, rather then ending, the character set, so ] and NOT ] can be matched by []] and [!]] respectively. The - character can be specified inside a character sequence pattern by placing it at the start or the end, e.g. [abc-].\n
MyGlob care rule patters (option -2, default): Include all above patterns, plus:
•   {choice1,choice2...}  match any of the comma-separated choices between braces. Can be nested, and include ?, * and character classes.
•   Character classes [ ] accept regex syntax, see https://docs.rs/regex/latest/regex/#character-classes for character classes and escape sequences supported.";

        println!("{}", Self::format_text(text, width));
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

            if current_line_length + word_length + 1 <= width {
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
                        let segres = arg.parse::<u8>();
                        if let Ok(s) = segres {
                            if (1..5).contains(&s) {
                                options.segment = s;
                                continue;
                            }
                        }
                        return Err("Option -s requires a numerical argument in 1..5".into());
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
            options.sources.push(r"C:\Users\Pierr\Downloads\A_Trier\!A_Trier_Revues".into());
        }

        Ok(options)
    }
}
