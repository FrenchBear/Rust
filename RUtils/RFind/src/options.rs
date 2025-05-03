// rfind - module options
// Process command line options
//
// 2025-04-22   PV      Moved to a separate file
// 2025-05-03	PV      Option -name

// standard library imports
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;

// external crates imports
use terminal_size::{Width, terminal_size};

// app imports
use crate::*;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    pub sources: Vec<String>,
    pub actions_names: HashSet<&'static str>,
    pub search_files: bool,
    pub search_dirs: bool,
    pub names: Vec<String>,
    pub isempty: bool,
    pub recycle: bool,
    pub autorecurse: bool,
    pub noaction: bool,
    pub verbose: bool,
}

impl Options {
    fn header() {
        eprintln!(
            "{APP_NAME} {APP_VERSION}\n\
            Searching files in Rust"
        );
    }

    fn usage() {
        Options::header();
        eprintln!(
            "\nUsage: {APP_NAME} [?|-?|-h|??] [-v] [-n] [-f|-type f|-d|-type d] [-e|-empty] [-r+|-r-] [-a+|-a-] [action...] [-name name] source...

Options:
?|-?|-h          Show this message
??               Show advanced usage notes
-v               Verbose output
-n               No action: display actions, but don't execute them
-f|-type f       Search for files
-d|-type d       Search for directories
-e|-empty        Only find empty files or directories
-r+|-r-          Delete to recycle bin (default) or delete forever; Recycle bin is not allowed on network sources
-a+|-a-          Enable (default) or disable glob autorecurse mode (see extended usage)
-name name       Appends **/name to each source directory (compatibility with XFinf/Search)
source           File or directory where to search (autorecurse glob pattern, see advanced notes)

Actions:
-print           Default, print matching files names and dir names (dir names end with \\)
-dir             Variant of -print, with last modification date and size
-delete          Delete matching files
-rmdir           Delete matching directories, whether empty or not"
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

Option -norecycle can be used instead of -r-, to indicate to delete forever.

Glob pattern rules:
•   ? matches any single character.
•   * matches any (possibly empty) sequence of characters.
•   ** matches the current directory and arbitrary subdirectories. To match files in arbitrary subdirectories, use **\\*. This sequence must form a single path component, so both **a and b** are invalid and will result in an error.
•   [...] matches any character inside the brackets. Character sequences can also specify ranges of characters, as ordered by Unicode, so e.g. [0-9] specifies any character between 0 and 9 inclusive. An unclosed bracket is invalid.
•   [!...] is the negation of [...], i.e. it matches any characters not in the brackets.
•   The metacharacters ?, *, [, ] can be matched by using brackets (e.g. [?]). When a ] occurs immediately following [ or [! then it is interpreted as being part of, rather then ending, the character set, so ] and NOT ] can be matched by []] and [!]] respectively. The - character can be specified inside a character sequence pattern by placing it at the start or the end, e.g. [abc-].
•   {choice1,choice2...}  match any of the comma-separated choices between braces. Can be nested, and include ?, * and character classes.
•   Character classes [ ] accept regex syntax such as [\\d] to match a single digit, see https://docs.rs/regex/latest/regex/#character-classes for character classes and escape sequences supported.

Autorecurse glob pattern transformation (active by default, use -a- to deactivate):
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
    pub fn new() -> Result<Options, Box<dyn Error>> {
        let args: Vec<String> = std::env::args().collect();

        let mut options = Options {
            autorecurse: true,
            recycle: true,
            ..Default::default()
        };

        // Since we have non-standard long options, don't use getopt for options processing but a manual loop
        let mut args_iter = args.iter();
        args_iter.next(); // Skip application executable
        while let Some(arg) = args_iter.next() {
            if arg.starts_with('-') || arg.starts_with('/') {
                // Options are case insensitive
                let arglc = arg[1..].to_lowercase();

                match &arglc[..] {
                    "?" | "h" | "help" | "-help" => {
                        Self::usage();
                        return Err("".into());
                    }

                    "??" => {
                        Self::extended_usage();
                        return Err("".into());
                    }

                    "v" => options.verbose = true,
                    "n" => options.noaction = true,

                    "f" => options.search_files = true,
                    "d" => options.search_dirs = true,
                    "type" => {
                        if let Some(search_type) = args_iter.next() {
                            match search_type.to_lowercase().as_str() {
                                "f" => options.search_files = true,
                                "d" => options.search_dirs = true,
                                _ => return Err(format!("Invalid argument {search_type} for pption -type, valid arguments are f or d").into()),
                            }
                        } else {
                            return Err("Option -type requires an argument f or d".into());
                        }
                    }

                    "name" => {
                        if let Some(name) = args_iter.next() {
                            options.names.push(name.clone());
                        } else {
                            return Err("Option -name requires an argument".into());
                        }
                    }

                    "e" | "empty" => options.isempty = true,

                    "r+" => options.recycle = true,
                    "r-" | "norecycle" => options.recycle = false,

                    "a+" => options.autorecurse = true,
                    "a-" => options.autorecurse = false,

                    "print" => {
                        options.actions_names.insert("print");
                    }
                    "dir" => {
                        options.actions_names.insert("dir");
                    }
                    "rm" | "del" | "delete" => {
                        options.actions_names.insert("delete");
                    }
                    "rd" | "rmdir" => {
                        options.actions_names.insert("rmdir");
                    }

                    _ => {
                        return Err(format!("Invalid/unsupported option {}", arg).into());
                    }
                }
            } else {
                // Non-option, some values are special
                match &arg.to_lowercase()[..] {
                    "?" | "h" | "help" => {
                        Self::usage();
                        return Err("".into());
                    }

                    "??" => {
                        Self::extended_usage();
                        return Err("".into());
                    }

                    // Everything else is considered as a source (a glob pattern), it will be validated later
                    _ => options.sources.push(arg.clone()),
                }
            }
        }

        // If neither filtering files or dirs has been requested, then we search for both
        if !options.search_dirs && !options.search_files {
            options.search_dirs = true;
            options.search_files = true;
        }

        // If no action is specified, then print action is default
        if options.actions_names.is_empty() {
            options.actions_names.insert("print");
        }

        Ok(options)
    }
}
