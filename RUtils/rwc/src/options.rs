// rwc - Module options
// Ottions processing
//
// 2025-04-21   PV      First version

// standard library imports
use std::error::Error;

// external crates imports
use getopt::Opt;
use terminal_size::{Width, terminal_size};

// Application imports
use crate::*;

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
        eprintln!(
            "{APP_NAME} {APP_VERSION}\n\
            Word count in rust"
        );
    }

    fn usage() {
        Options::header();
        eprintln!(
            "\nUsage: {APP_NAME} [?|-?|-h|??] [-a+|-a-] [-t] [-v] [source...]
?|-?|-h  Show this message
??       Show advanced usage notes
-a+|-a-  Enable (default) or disable glob autorecurse mode (see extended usage)
-t       Only show total line
-v       Verbose output
source   File or folder where to search, glob syntax supported. Without source, search stdin."
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
Advanced usage notes\n--------------------

The four numerical fields report lines, words, characters and bytes counts. For utf-8 or utf-16 encoded files, a character is an Unicode codepoint, so bytes and characters counts may be different.

Words are series of character(s) separated by space(s), spaces are either ASCII 9 (tab) or 32 (regular space).  Unicode \"fancy spaces\" are not considered.

Lines end with \\r, \\n or \\r\\n. If the last line of the file ends with such termination character, an extra empty line is counted.

Glob pattern rules:
•   ? matches any single character.
•   * matches any (possibly empty) sequence of characters.
•   ** matches the current directory and arbitrary subdirectories. To match files in arbitrary subdirectories, use **\\*. This sequence must form a single path component, so both **a and b** are invalid and will result in an error.
•   [...] matches any character inside the brackets. Character sequences can also specify ranges of characters, as ordered by Unicode, so e.g. [0-9] specifies any character between 0 and 9 inclusive. An unclosed bracket is invalid.
•   [!...] is the negation of [...], i.e. it matches any characters not in the brackets.
•   The metacharacters ?, *, [, ] can be matched by using brackets (e.g. [?]). When a ] occurs immediately following [ or [! then it is interpreted as being part of, rather then ending, the character set, so ] and NOT ] can be matched by []] and [!]] respectively. The - character can be specified inside a character sequence pattern by placing it at the start or the end, e.g. [abc-].
•   {choice1,choice2...}  match any of the comma-separated choices between braces. Can be nested, and include ?, * and character classes.
•   Character classes [ ] accept regex syntax such as [\\d] to match a single digit, see https://docs.rs/regex/latest/regex/#character-classes for character classes and escape sequences supported.

Autorecurse glob pattern transformation (active by default, use -a- to disable):
•   Constant pattern (no filter, no **) pointing to a folder: \\**\\* is appended at the end to search all files of all subfolders.
•   Patterns without ** and ending with a filter: \\** is inserted before final filter to find all matching files of all subfolders.";

        println!("{}", Self::format_text(text, width));
    }

    fn format_text(text: &str, width: usize) -> String {
        let mut s = String::new();
        for line in text.lines() {
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

        let mut options = Options {
            autorecurse: true,
            ..Default::default()
        };
        let mut opts = getopt::Parser::new(&args, "h?ftva:");

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
