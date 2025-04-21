// rgrep: Basic grep project in Rust
//
// 2025-03-13	PV      First version
// 2025-03-16	PV      1.0.1   Extended help, support reading from stdin
// 2025-03-25	PV      1.1.0   Global constants; Ignore $RECYCLE.BIN
// 2025-03-27   PV      1.2.0   Option -2 to use MyGlob crate (experimental)
// 2025-03-28   PV      1.2.1   Option -1 to use glob crate, glob syntax documented in extended help
// 2025-03-29   PV      1.2.2   Option -2 is now default; Rename rgrep
// 2025-04-01   PV      1.3.0   read_text_file_2, faster to detect text encoding
// 2025-04-08   PV      1.4.0   When stdout is redirected, don't use colors (atty crate)
// 2025-04-18   PV      1.4.1   Only check help and ? on first position in command line; more extended help
// 2025-04-18   PV      1.5.0   End of glob crate and options -1/-2

// standard library imports
use std::error::Error;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

// external crates imports
use getopt::Opt;
use myglob::{MyGlobMatch, MyGlobSearch};
use regex::Regex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use terminal_size::{Width, terminal_size};

// -----------------------------------
// Submodules

mod decode_encoding;
mod grepiterator;
pub mod tests;

use decode_encoding::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = "rgrep";
const APP_VERSION: &str = "1.5.0";

// ==============================================================================================
// Options processing

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pattern: String,
    sources: Vec<String>,
    ignore_case: bool,
    whole_word: bool,
    fixed_string: bool,
    autorecurse: bool,
    show_path: bool,
    out_level: u8, // 0: normal output, 1: (-l) matching filenames only, 2: (-c) filenames and matching lines count, 3: (-c -l) only matching filenames and matching lines count
    verbose: u8,
}

impl Options {
    fn header() {
        eprintln!(
            "{APP_NAME} {APP_VERSION}\n\
            Simplified grep in rust"
        );
    }

    fn usage() {
        Options::header();
        eprintln!(
            "\nUsage: {APP_NAME} [?|-?|-h|??] [-i] [-w] [-F] [-r] [-v] [-c] [-l] pattern source...
?|-?|-h  Show this message
??       Show advanced usage notes
-i       Ignore case during search
-w       Whole word search
-F       Fixed string search (no regexp interpretation), also for patterns starting with - ? or help
-r       Use autorecurse, see advanced help
-c       Suppress normal output, show count of matching lines for each file
-l       Suppress normal output, show matching file names only
-v       Verbose output
pattern  Regular expression to search
source   File or folder where to search, glob syntax supported"
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

Options -c (show count of matching lines) and -l (show matching file names only) can be used together to show matching lines count only for matching files.

To search for help or ?, use option -F: rgrep -F help C:\\Sources\\**\\*.rs
To search for a string containing double quotes, surround string by double quotes, and double individual double quotes inside. To search for \"help\": rgrep \"\"\"help\"\"\" C:\\Sources\\**\\*.rs

Glob pattern rules:
•   ? matches any single character.
•   * matches any (possibly empty) sequence of characters.
•   ** matches the current directory and arbitrary subdirectories. To match files in arbitrary subdirectories, use **\\*. This sequence must form a single path component, so both **a and b** are invalid and will result in an error.
•   [...] matches any character inside the brackets. Character sequences can also specify ranges of characters, as ordered by Unicode, so e.g. [0-9] specifies any character between 0 and 9 inclusive. An unclosed bracket is invalid.
•   [!...] is the negation of [...], i.e. it matches any characters not in the brackets.
•   The metacharacters ?, *, [, ] can be matched by using brackets (e.g. [?]). When a ] occurs immediately following [ or [! then it is interpreted as being part of, rather then ending, the character set, so ] and NOT ] can be matched by []] and [!]] respectively. The - character can be specified inside a character sequence pattern by placing it at the start or the end, e.g. [abc-].
•   {choice1,choice2...}  match any of the comma-separated choices between braces. Can be nested, and include ?, * and character classes.
•   Character classes [ ] accept regex syntax such as [\\d] to match a single digit, see https://docs.rs/regex/latest/regex/#character-classes for character classes and escape sequences supported.

Autorecurse glob pattern transformation (option -r):
•   Constant pattern (no filter, no **) pointing to a folder: \\**\\* is appended at the end to search all files of all subfolders.
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
    fn new() -> Result<Options, Box<dyn Error>> {
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

// -----------------------------------
// Main

fn main() {
    // Process options
    let mut options = Options::new().unwrap_or_else(|err| {
        let msg = format!("{}", err);
        if msg.is_empty() {
            process::exit(0);
        }
        eprintln!("{APP_NAME}: Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let re = build_re(&options);
    if re.is_err() {
        eprintln!("{APP_NAME}: Problem with search pattern: {:?}", re.err().unwrap());
        process::exit(1);
    }
    let re = re.unwrap();

    let start = Instant::now();

    // Building list of files
    // ToDo: It could be better to process file just when it's returned by iterator rather than stored in a Vec and processed later...
    let mut files: Vec<PathBuf> = Vec::new();
    for source in options.sources.iter() {
        let mut count = 0;

        let resgs = MyGlobSearch::new(source).autorecurse(options.autorecurse).compile();
        match resgs {
            Ok(gs) => {
                for ma in gs.explore_iter() {
                    match ma {
                        MyGlobMatch::File(pb) => {
                            count += 1;
                            files.push(pb);
                        }

                        // We ignore matching directories in rgrep, we only look for files
                        MyGlobMatch::Dir(_) => {}

                        MyGlobMatch::Error(err) => {
                            if options.verbose > 0 {
                                eprintln!("{APP_NAME}: error {}", err);
                            }
                        }
                    }
                }
            }

            Err(e) => {
                eprintln!("{APP_NAME}: Error building MyGlob: {:?}", e);
                count = -1; // No need to display "no file found" in this case
            }
        }

        if count == 0 {
            println!("{APP_NAME}: no file found matching {}", source);
        }
    }

    // Finally processing files, if more than 1 file, prefix output with file
    if options.sources.is_empty() {
        if options.verbose > 0 {
            println!("Reading from stdin");
        }
        let s = io::read_to_string(io::stdin()).unwrap();
        process_text(&re, s.as_str(), "(stdin)", &options);
    } else {
        if files.len() > 1 {
            options.show_path = true;
        }
        for pb in &files {
            if options.verbose > 1 {
                println!("Process: {}", pb.display());
            }
            process_path(&re, pb, &options);
        }
    }
    let duration = start.elapsed();

    if options.verbose > 0 {
        if files.is_empty() {
            print!("\nstdin");
        } else {
            print!("\n{} file", files.len());
            if files.len() > 1 {
                print!("s");
            }
        }
        println!(" searched in {:.3}s", duration.as_secs_f64());
    }
}

/// Helper, build Regex according to options (case, fixed string, whole word).<br/>
/// Return an error in case of invalid Regex.
pub fn build_re(options: &Options) -> Result<Regex, regex::Error> {
    let spat = if options.fixed_string {
        regex::escape(options.pattern.as_str())
    } else if options.whole_word {
        format!("\\b{}\\b", options.pattern)
    } else {
        options.pattern.clone()
    };
    let spat = String::from(if options.ignore_case { "(?imR)" } else { "(?mR)" }) + spat.as_str();
    Regex::new(spat.as_str())
}

/// First step processing a file, read text content from path and call process_text.
fn process_path(re: &Regex, path: &Path, options: &Options) {
    let res = read_text_file(path);
    match res {
        Ok((Some(s), _)) => {
            let filename = path.display().to_string();
            process_text(re, s.as_str(), filename.as_str(), options);
        }
        Ok((None, _)) => {
            // Non-text files are ignored
            if options.verbose == 1 {
                println!("{APP_NAME}: ignored non-text file {}", path.display());
            }
        }
        Err(e) => {
            eprintln!("*** Error reading file {}: {}", path.display(), e);
        }
    }
}

/// Core rgrep process, search for re in txt, read from filename, according to options.
fn process_text(re: &Regex, txt: &str, filename: &str, options: &Options) {
    let mut matchlinecount = 0;

    if atty::is(atty::Stream::Stdout) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut match_color = ColorSpec::new();
        match_color.set_fg(Some(Color::Red)).set_bold(true);
        let mut file_color = ColorSpec::new();
        file_color.set_fg(Some(Color::Black)).set_intense(true);

        for gi in grepiterator::GrepLineMatches::new(txt, re) {
            matchlinecount += 1;

            if options.out_level == 1 {
                println!("{}", filename);
                return;
            }

            if options.out_level == 0 {
                if options.show_path {
                    let _ = stdout.set_color(&file_color);
                    let _ = write!(&mut stdout, "{}: ", filename);
                    let _ = stdout.reset();
                }

                let mut p: usize = 0;
                for ma in gi.ranges {
                    let e = ma.end;
                    print!("{}", &gi.line[p..ma.start]);
                    let _ = stdout.set_color(&match_color);
                    let _ = write!(&mut stdout, "{}", &gi.line[ma]);
                    let _ = stdout.reset();
                    p = e;
                }
                println!("{}", &gi.line[p..]);
            }
        }
    } else {
        for gi in grepiterator::GrepLineMatches::new(txt, re) {
            matchlinecount += 1;

            if options.out_level == 1 {
                println!("{}", filename);
                return;
            }

            if options.out_level == 0 {
                if options.show_path {
                    print!("{}: ", filename);
                }
                println!("{}", gi.line);
            }
        }
    }

    // Note: both options -c and -l (out_level==3) is not supported by Linux version
    if options.out_level == 2 || (options.out_level == 3 && matchlinecount > 0) {
        println!("{}:{}", filename, matchlinecount);
    }
}
