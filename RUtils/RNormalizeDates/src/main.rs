// rnormalizedates: Rust version of NormalizeDates, Normalizes dates in filenames, replace 'January 2020' by '2020-01'
//
// 2025-04-12	PV      First version

#![allow(unused)]

// standard library imports
use std::path::Path;
use std::process;
use std::time::Instant;
use std::{error::Error, path::PathBuf};

// external crates imports
use getopt::Opt;
use myglob::{MyGlobMatch, MyGlobSearch};
use terminal_size::{Width, terminal_size};
use unicode_normalization::UnicodeNormalization;

// -----------------------------------
// Submodules

mod devdata;
mod tests;

use devdata::get_dev_data;

// -----------------------------------
// Global constants

const APP_NAME: &str = "rnormalizedates";
const APP_VERSION: &str = "1.0.0";

// ==============================================================================================
// Options processing

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    sources: Vec<String>,
    final_pause: bool,
    segment: u8,
    no_action: bool,
    verbose: bool,
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
    fn new() -> Result<Options, Box<dyn Error>> {
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

// -----------------------------------
// Main

// Dev tests
fn main() {
    for filefp in get_dev_data() {
        process_file(&PathBuf::from(filefp));
    }
}

fn zz_main() {
    // Process options
    let options = Options::new().unwrap_or_else(|err| {
        let msg = format!("{}", err);
        if msg.is_empty() {
            process::exit(0);
        }
        eprintln!("{APP_NAME}: Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let start = Instant::now();

    for source in options.sources.iter() {
        let resgs = MyGlobSearch::new(source).autorecurse(true).compile();
        match resgs {
            Ok(gs) => {
                for ma in gs.explore_iter() {
                    match ma {
                        MyGlobMatch::File(pb) => {
                            process_file(&pb);
                        }

                        // We ignore matching directories in rgrep, we only look for files
                        MyGlobMatch::Dir(_) => {}

                        MyGlobMatch::Error(_) => {}
                    }
                }
            }

            Err(e) => {
                eprintln!("{APP_NAME}: Error building MyGlob: {:?}", e);
            }
        }
    }

    let duration = start.elapsed();
    println!("\nDuration: {:.3}s", duration.as_secs_f64());
}

fn process_file(pb: &Path) {
    //println!("Processing {}", pb.display());

    let basename_original = pb.file_stem().expect("No stem??").to_string_lossy().into_owned();

    let mut base_name: String = basename_original.nfc().collect();
    base_name = base_name.replace('_', " ");
    base_name = base_name.replace("..", "$");   // Keep double dots
    base_name = base_name.replace(".", " ");    // But replace simple dots by spaces
    base_name = base_name.replace("$", "..");
    base_name = base_name.replace("\u{FFFD}", " ");     // Replacement character


    // Add starting/ending space to simplyfy some detections
    base_name=format!(" {} ", base_name);
    loop {
        let mut update = false;

        if base_name.contains("  ") {
            base_name=base_name.replace("  ", " ");
            update = true;
        }
        if base_name.contains("- -") {
            base_name=base_name.replace("- -", "-");
            update = true;
        }
        if base_name.contains("--") {
            base_name=base_name.replace("--", "-");
            update = true;
        }
        if icontains(&base_name, "PDF-NOTAG") {
            base_name=ireplace(&base_name, "PDF-NOTAG", "");
            update = true;
        }
        if icontains(&base_name, " FRENCH ") {
            base_name=ireplace(&base_name, " FRENCH ", " ");
            update = true;
        }
        if icontains(&base_name, " francais ") {
            base_name=ireplace(&base_name, " francais ", " ");
            update = true;
        }

        if !update {break;}
    }

    //base_name=base_name.trim().into();

    println!("{:70} «{}»", basename_original, base_name);

}

// Case-insensitive version of contains
fn icontains(s: &str, pattern: &str) -> bool {
    s.to_lowercase().contains(&pattern.to_lowercase())
}

// Case-insensitive version of replace
fn ireplace(s: &str, search: &str, replace: &str) -> String {
    if search.is_empty() {panic!("search can't be empty");}
    let mut result = String::new();
    let lower_s = s.to_lowercase();
    let lower_search = search.to_lowercase();
    let mut i = 0;

    while i < s.len() {
        if lower_s[i..].starts_with(&lower_search) {
            result.push_str(replace);
            i += search.len();
        } else {
            let ch = &s[i..].chars().next().unwrap();
            result.push(*ch);
            i += ch.len_utf8();
        }
    }

    result
}