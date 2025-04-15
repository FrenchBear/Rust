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
            "\nUsage: {APP_NAME} [?|-?|-h|??] [-n] [-p] [-v] [-s #] source...
?|-?|-h  Show this message
??       Show advanced usage notes
-n       Do not actually rename (no action)
-p       Final pause
-v       Verbose output
-s #     Only process segment # (starting at 1) delimited by ' - '    *** NOT IMPLEMENTED YET
source   folder containing PDF files (and recurse) or simple file, default: C:\\Downloads\\A_Trier\\!A_Trier_Revues\\*.pdf"
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

Without argument, default folder is C:\\Downloads\\A_Trier\\!A_Trier_Revues\\**\\*.pdf

Glob pattern rules:
•   ? matches any single character.
•   * matches any (possibly empty) sequence of characters.
•   ** matches the current directory and arbitrary subdirectories. To match files in arbitrary subdiretories, use **\\*. This sequence must form a single path component, so both **a and b** are invalid and will result in an error.
•   [...] matches any character inside the brackets. Character sequences can also specify ranges of characters, as ordered by Unicode, so e.g. [0-9] specifies any character between 0 and 9 inclusive. An unclosed bracket is invalid.
•   [!...] is the negation of [...], i.e. it matches any characters not in the brackets.
•   The metacharacters ?, *, [, ] can be matched by using brackets (e.g. [?]). When a ] occurs immediately following [ or [! then it is interpreted as being part of, rather then ending, the character set, so ] and NOT ] can be matched by []] and [!]] respectively. The - character can be specified inside a character sequence pattern by placing it at the start or the end, e.g. [abc-].
•   {choice1,choice2...}  match any of the comma-separated choices between braces. Can be nested, and include ?, * and character classes.
•   Character classes [ ] accept regex syntax such as [\\d] to match a single digit, see https://docs.rs/regex/latest/regex/#character-classes for character classes and escape sequences supported.

Autorecurse glob pattern transformation is active:
•   Constant pattern (no filter, no **) pointing to a folder: \\**\\* is appended at the end to search all files of all seubfolders.
•   Patterns without ** and ending with a filter: \\** is inserted before final filter to find all matching files of all subfolders.";

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
                                eprint!("*** OPTION -S NOT IMPLEMENTED YET");
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
            options.sources.push(r"C:\Downloads\A_Trier\!A_Trier_Revues\*.pdf".into());
        }

        Ok(options)
    }
}

