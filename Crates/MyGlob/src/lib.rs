// my_glob library
//
// Attempt to implement an efficient glob in Rust
//
// 2025-03-25   PV      First version, experiments around various options to select the fastest
// 2025-03-26   PV      Second version, use my own algorithm, and use regex for Filter segments match check
// 2025-03-26   PV      Third version, a non-recursive version of explore to prepare for iterator version
// 2025-03-27   PV      Fourth version, iterator
// 2025-03-27   PV      1.0  First official version of the crate
// 2025-03-28   PV      1.1  Proper conversion from glob to regex with glob_to_segments
// 2025-03-29   PV      1.2  Test cases, documentation of regex, bug of \ inside a [ ] fixed
// 2025-03-29   PV      1.3  Now returns files and directories
// 2025-03-30   PV      1.3.1 Search for constant directory fixed; Append \* to glob ending with **
// 2025-04-03   PV      1.3.2 is_constant member added to MyGlobSearch
// 2025-04-09   PV      1.3.3 Fixed bug charindex/byteindex during initial cut of constant part
// 2025-04-09   PV      1.4.0 New MyGlob API with MyBlobBuilder, version, new, compile and add_ignore_dir.
// 2025-04-13   PV      1.5.0 Autorecurse
// 2025-04-18   PV      1.5.1 MyGlobError implements std::error::Error
// 2025-04-23   PV      1.5.2 Added impl From<regex::Error> for MyGlobError and fn source in impl Error for MyGlobError
// 2025-05-03   PV      1.5.3 Removed #![allow(...)]
// 2025-05-04   PV      1.5.4 Added glob_syntax()
// 2025-07-12   PV      1.6.0 Don't complain about glob patterns ending with / or \
// 2025-08-08   PV      1.7.0 Use get_root to rewrite root separation, and handle windows C:\xxx patterns corretcly
// 2025-09-06   PV      1.8.0 MaxDepth added
// 2025-09-08   PV      1.9.0 Use queue instead of stack to have a breadth-first search instead of depth-first, and return results in a more natural order
// 2025-09-13   PV      1.9.1 Check for unclosed brackets in glob expressions such as "C:\[a-z"
// 2025-01-01   PV      1.10  Macro !SOURCES to represend common (for me) source files extensions. d is not in the list (also rust temp build files extension)

#![allow(unused_variables, dead_code, unused_imports)]

// Standard library imports
use regex::Regex;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io::Error as IOError;
use std::path::{Path, PathBuf};

// -----------------------------------
// Submodules

mod tests;

// -----------------------------------
// Globals

const LIB_VERSION: &str = env!("CARGO_PKG_VERSION");

// -----------------------------------
// Structures

// Internal structure, store one segment of a glob pattern, either a constant string, a recurse tag (**), or a glob filter, converted into a Regex
#[derive(Debug)]
pub enum Segment {
    Constant(String),
    Recurse,
    Filter(Regex),
}

/// Main struct of MyGlob, string information such as root part, glob, dirs to ignore, ...
#[derive(Debug, Default)]
pub struct MyGlobSearch {
    root: String,
    pub segments: Vec<Segment>,     // pub for debugging
    ignore_dirs: Vec<String>,
    maxdepth: usize,
}

#[derive(Debug, Default)]
pub struct MyGlobBuilder {
    glob_pattern: String,
    ignore_dirs: Vec<String>, // just plain lowercase dir name, no path, no *
    maxdepth: usize,          // Counted from ** segment, 0 means no limit
    autorecurse: bool,        // Apply optional autorecurse transformation
}

/// Error returned by MyGlob, either a Regex error or an io::Error
#[derive(Debug)]
pub enum MyGlobError {
    IoError(std::io::Error),
    RegexError(regex::Error),
    GlobError(String),
}

// Automatically provide ToString conversion
impl Display for MyGlobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MyGlobError::IoError(error) => write!(f, "IOError: {}", error),
            MyGlobError::RegexError(error) => write!(f, "RegexError: {}", error),
            MyGlobError::GlobError(s) => write!(f, "MyGlobError: {}", s),
        }
    }
}

impl Error for MyGlobError {
    // Optional to implement, not really testes
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyGlobError::IoError(e) => Some(e),
            MyGlobError::RegexError(e) => Some(e),
            MyGlobError::GlobError(_) => None,
        }
    }
}

impl MyGlobSearch {
    pub fn version() -> &'static str {
        LIB_VERSION
    }

    pub fn glob_syntax() -> &'static str {
        "⌊Glob pattern rules⌋:
- ¬⟦?⟧ matches any single character.
- ¬⟦*⟧ matches any (possibly empty) sequence of characters.
- ¬⟦**⟧ matches the current directory and arbitrary subdirectories. To match files in arbitrary subdirectories, use ⟦**/*⟧. This sequence must form a single path component, so both ⟦**a⟧ and ⟦b**⟧ are invalid and will result in an error.
- ¬⟦[...]⟧ matches any character inside the brackets. Character sequences can also specify ranges of characters (Unicode order), so ⟦[0-9]⟧ specifies any character between 0 and 9 inclusive. Special cases: ⟦[[]⟧ represents an opening bracket, ⟦[]]⟧ represents a closing bracket. 
- ¬⟦[!...]⟧ is the negation of ⟦[...]⟧, it matches any characters not in the brackets.
- ¬The metacharacters ⟦?⟧, ⟦*⟧, ⟦[⟧, ⟦]⟧ can be matched by escaping them between brackets such as ⟦[\\?]⟧, ⟦[\\]]⟧ or ⟦[\\[]⟧. The ⟦-⟧ character can be specified inside a character sequence pattern by placing it at the start or the end, e.g. ⟦[abc-]⟧.
- ¬⟦{choice1,choice2...}⟧  match any of the comma-separated choices between braces. Can be nested, and include ⟦?⟧, ⟦*⟧ and character classes. Special macro ⟦!SOURCES⟧ is replaced by common sources extensions (.c,.cs,.cpp...) and typically used in expressions such as ⟦*.{!SOURCES}⟧ to find source files.
- ¬Character classes ⟦[ ]⟧ accept regex syntax such as ⟦[\\d]⟧ to match a single digit, see https://docs.rs/regex/latest/regex/#character-classes for character classes and escape sequences supported.

⌊Autorecurse glob pattern transformation⌋:
- ¬⟪Constant pattern⟫ (no filter, no ⟦**⟧) pointing to a directory: ⟦/**/*⟧ is appended at the end to search all files of all subdirectories.
- ¬⟪Patterns without ⟦**⟧ and ending with a filter⟫: ⟦/**⟧ is inserted before the final filter to find all matching files of all subdirectories.
"
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new(glob_pattern: &str) -> MyGlobBuilder {
        MyGlobBuilder {
            glob_pattern: glob_pattern.to_string(),
            ignore_dirs: vec![
                String::from("$recycle.bin"),
                String::from("system volume information"),
                String::from(".git"),
            ],
            ..Default::default()
        }
    }

    pub fn build(glob_pattern: &str) -> Result<Self, MyGlobError> {
        Self::new(glob_pattern).compile()
    }

    /// Iterator returning all files matching glob pattern
    pub fn explore_iter(&self) -> impl Iterator<Item = MyGlobMatch> {
        // Special case, segments is empty, only search for file
        // It's actually a but faster to process it before iterator loop, so there is no special case to handle at the beginning of each iterator call
        if self.segments.is_empty() {
            let p = Path::new(&self.root);
            let mut stack: Vec<SearchPendingData> = Vec::new();
            if p.is_file() {
                stack.insert(0, SearchPendingData::File(p.to_path_buf()));
            } else if p.is_dir() {
                stack.insert(0, SearchPendingData::Dir(p.to_path_buf()));
            }
            return MyGlobIteratorState {
                queue: stack,
                segments: &self.segments,
                ignore_dirs: &self.ignore_dirs,
                maxdepth: self.maxdepth,
            };
        }

        // Normal case, start iterator at root
        MyGlobIteratorState {
            queue: vec![SearchPendingData::DirToExplore(Path::new(&self.root).to_path_buf(), 0, false, 0)],
            segments: &self.segments,
            ignore_dirs: &self.ignore_dirs,
            maxdepth: self.maxdepth,
        }
    }

    /// Returns true if glob is valid, but it's just a constant, no filter segment and no recurse segment
    /// Note that this should always be called on a compiled MyGlob; calling on a non-compiled MyGlob will always return false
    pub fn is_constant(&self) -> bool {
        self.segments.is_empty()
    }
}

impl MyGlobBuilder {
    /// Add a directory name to ignore during search, case insensitive (no path, no *)
    pub fn add_ignore_dir(mut self, dir: &str) -> Self {
        self.ignore_dirs.push(dir.to_lowercase());
        self
    }

    /// Set maxdepth, counted from ** segment, 0 means no limit
    pub fn maxdepth(mut self, depth: usize) -> Self {
        if depth == 0 {
            return self;
        }
        self.maxdepth = depth;
        self
    }

    /// Set autorecurse flag. There is no mechanism to clear it, since it's clear by default.
    pub fn autorecurse(mut self, active: bool) -> Self {
        self.autorecurse = active;
        self
    }

    // Helper to separate constant root prefix from segments
    // Was initially part of compile, but it's better to put in a separate function for careful testing
    pub fn get_root(glob_pattern: &str) -> (String, String) {
        let mut glob = glob_pattern.to_string();
        // Instead of raising an error in case of empty pattern, just consider it's equivalent to *, similar to dir/ls commands behavior
        if glob.is_empty() {
            glob = String::from("*")
        }

        // Extract the root pattern, the constant beginning of glob expression not containing *?[{ chars
        let mut cut = 0;
        let mut pos = 0;
        let mut root: String;

        glob.push('\u{0}'); // Add a end marker to simplify code
        for c in glob.chars() {
            if "*?[{".contains(c) {
                break;
            }
            if c == '/' || c == '\\' {
                // Note that \ have a special meaning between [ ] but we break the loop at the first [ so it's Ok
                cut = pos + 1;
            }
            if c == '\u{0}' {
                cut = pos;
                break;
            }
            pos += c.len_utf8();
        }

        root = glob[..cut].to_string();
        if root.is_empty() {
            root.push('.');
        }

        let rem = if pos < glob.len() - 1 {
            &glob[if cut == 0 { 0 } else { cut }..glob.len() - 1]
        } else {
            ""
        };

        (root, rem.to_string())
    }

    /// Constructs a new MyGlobSearch based on pattern glob expression, or return an error if there is Glob/Regex error
    pub fn compile(self) -> Result<MyGlobSearch, MyGlobError> {
        let (root, rem) = MyGlobBuilder::get_root(&self.glob_pattern);

        // Then build segments
        let mut segments = if rem.is_empty() { Vec::new() } else { Self::glob_to_segments(&rem)? };

        // Process autorecurse transformation if required
        if self.autorecurse {
            // Case of constant pattern that is a valid directory, add  **/*
            if segments.is_empty() {
                let rootp = PathBuf::from(&root);
                if rootp.is_dir() {
                    segments.push(Segment::Recurse);
                    segments.push(Segment::Filter(Regex::new("(?i)^.*$").unwrap()));
                }
            } else {
                // Case of non-recursive pattern ending with a filter; insert ** before last segment
                if !segments.iter().any(|s| matches!(s, Segment::Recurse)) && matches!(segments.last().unwrap(), Segment::Filter(_)) {
                    segments.insert(segments.len() - 1, Segment::Recurse);
                }
            }
        }

        Ok(MyGlobSearch {
            root,
            segments,
            ignore_dirs: self.ignore_dirs,
            maxdepth: self.maxdepth,
        })
    }

    // Conversion of a glob string into a Vec<Segment>, or an error if glob syntax is invalid
    pub fn glob_to_segments(glob_pattern_arg: &str) -> Result<Vec<Segment>, MyGlobError> {
        // glob_pattern ends with \ so no duplicate code to process last segment
        let dir_sep = if cfg!(target_os = "windows") { '\\' } else { '/' };
        let mut glob_pattern = glob_pattern_arg.to_string();
        if !glob_pattern.ends_with("/") && !glob_pattern.ends_with("\\") {
            glob_pattern.push(dir_sep);
        }

        // Macro expansion. For efficiency, don't use regexp or complex code to check it's surrounded by braces
        if let Some(pos) = glob_pattern.to_ascii_uppercase().find("!SOURCES") {
            glob_pattern = glob_pattern[..pos].to_string() + "asm,awk,c,cc,cpp,cs,cxx,fs,go,h,hpp,hxx,java,jl,js,lua,py,rs,sql,ts,vb,xaml" + &glob_pattern[pos + 8..];
        }

        let mut segments = Vec::<Segment>::new();
        let mut regex_buffer = String::new();
        let mut constant_buffer = String::new();
        let mut brace_depth = 0;
        let mut in_bracket = false;
        let mut iter = glob_pattern.chars().peekable();
        while let Some(c) = iter.next() {
            if c != '\\' && c != '/' {
                constant_buffer.push(c);
            }

            match c {
                '*' => regex_buffer.push_str(".*"),
                '?' => regex_buffer.push('.'),
                '{' => {
                    brace_depth += 1;
                    regex_buffer.push('(');
                }
                ',' if brace_depth > 0 => regex_buffer.push('|'),
                '}' => {
                    brace_depth -= 1;
                    if brace_depth < 0 {
                        return Err(MyGlobError::GlobError("Extra closing }".to_string()));
                    }
                    regex_buffer.push(')');
                }
                '\\' | '/' => {
                    if brace_depth > 0 {
                        return Err(MyGlobError::GlobError(format!("Invalid {c} between {{ }}")));
                    }

                    if constant_buffer == "**" {
                        segments.push(Segment::Recurse);
                    } else if constant_buffer.contains("**") {
                        return Err(MyGlobError::GlobError(format!("Glob pattern ** must be alone between {c}")));
                    } else if constant_buffer.chars().any(|c| "*?[{".contains(c)) {
                        if brace_depth > 0 {
                            return Err(MyGlobError::GlobError("Unclosed {".to_string()));
                        }

                        let repat = format!("(?i)^{}$", regex_buffer);
                        // let resre = Regex::new(&repat);
                        // match resre {
                        //     Ok(re) => segments.push(Segment::Filter(re)),
                        //     Err(e) => {
                        //         return Err(MyGlobError::RegexError(e));
                        //     }
                        // }

                        // Shorter thanks to impl From<regex::Error> for MyGlobError
                        segments.push(Segment::Filter(Regex::new(&repat)?));
                    } else {
                        segments.push(Segment::Constant(constant_buffer.clone()));
                    }
                    regex_buffer.clear();
                    constant_buffer.clear();
                }
                '[' => {
                    regex_buffer.push('[');
                    in_bracket = true;

                    // Special case, ! at the beginning of a glob match is converted to a ^ in regex syntax
                    if let Some(next_c) = iter.peek() {
                        if *next_c == '!' {
                            iter.next();
                            regex_buffer.push('^');
                        }
                    }

                    while let Some(inner_c) = iter.next() {
                        match inner_c {
                            ']' => {
                                regex_buffer.push(inner_c);
                                in_bracket = false;
                                break;
                            }
                            '\\' => {
                                if let Some(next_c) = iter.next() {
                                    regex_buffer.push('\\');
                                    regex_buffer.push(next_c);
                                } else {
                                    regex_buffer.push('\\'); //Handle trailing backslash
                                }
                            }
                            _ => regex_buffer.push(inner_c),
                        }
                    }
                }
                '.' | '+' | '(' | ')' | '|' | '^' | '$' => {
                    regex_buffer.push('\\');
                    regex_buffer.push(c);
                }
                _ => regex_buffer.push(c),
            }
        }
        if in_bracket {
            return Err(MyGlobError::GlobError("Unclosed [".to_string()));
        }

        if !regex_buffer.is_empty() {
            return Err(MyGlobError::GlobError("Invalid glob pattern".to_string()));
        }

        // If last segment is a **, append a Filter * to find everything
        if let Segment::Recurse = &segments[segments.len() - 1] {
            segments.push(Segment::Filter(Regex::new("(?i)^.*$").unwrap()));
        }

        Ok(segments)
    }
}

impl From<regex::Error> for MyGlobError {
    fn from(value: regex::Error) -> Self {
        MyGlobError::RegexError(value)
    }
}

// Enum returned by iterator
#[derive(Debug)]
pub enum MyGlobMatch {
    File(PathBuf),
    Dir(PathBuf),
    Error(IOError),
}

// Internal state of iterator
struct MyGlobIteratorState<'a> {
    queue: Vec<SearchPendingData>,
    segments: &'a Vec<Segment>,
    ignore_dirs: &'a Vec<String>,
    maxdepth: usize,
}

// Internal structure of derecursived search, pending data to explore or return, stored in stack
#[derive(Debug)]
enum SearchPendingData {
    File(PathBuf),                             // Data to return
    Dir(PathBuf),                              // Data to return
    DirToExplore(PathBuf, usize, bool, usize), // Dir not explored yet
    Error(IOError),                            // Returns an error
}

impl Iterator for MyGlobIteratorState<'_> {
    type Item = MyGlobMatch;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(fof) = self.queue.pop() {
            match fof {
                SearchPendingData::Error(e) => return Some(MyGlobMatch::Error(e)),

                SearchPendingData::File(pb) => return Some(MyGlobMatch::File(pb)),

                SearchPendingData::Dir(pb) => return Some(MyGlobMatch::Dir(pb)),

                SearchPendingData::DirToExplore(root, depth, recurse, recurse_depth) => {
                    match &self.segments[depth] {
                        Segment::Constant(name) => {
                            let pb = root.join(name);
                            if depth == self.segments.len() - 1 {
                                // Final segment
                                if pb.is_file() {
                                    // Case-insensitive comparison is provided by filesystem
                                    self.queue.insert(0, SearchPendingData::File(pb));
                                } else if pb.is_dir() {
                                    self.queue.insert(0, SearchPendingData::Dir(pb.clone()));
                                }
                            } else {
                                // non-final segment, can only match a directory
                                if pb.is_dir() {
                                    // Found a matching directory, we continue exploration in next loop
                                    self.queue.insert(0, SearchPendingData::DirToExplore(pb, depth + 1, false, 0));
                                }
                            }

                            // Then if recurse mode, we also search in all subdirectories
                            if recurse && (self.maxdepth == 0 || recurse_depth < self.maxdepth) {
                                match fs::read_dir(&root) {
                                    Ok(contents) => {
                                        for resentry in contents {
                                            match resentry {
                                                Ok(entry) => {
                                                    // Don't follow folders recursively if maxdepth has been reached
                                                    if entry.file_type().unwrap().is_dir() {
                                                        let p = entry.path();
                                                        let fnlc = p.file_name().unwrap().to_string_lossy().to_lowercase();
                                                        if !self.ignore_dirs.iter().any(|ie| *ie == fnlc.to_lowercase()) {
                                                            self.queue.insert(0, SearchPendingData::DirToExplore(p, depth, true, recurse_depth + 1));
                                                        }
                                                    }
                                                }

                                                Err(e) => {
                                                    let f = std::io::Error::new(e.kind(), format!("Error enumerating dir {}: {}", root.display(), e));
                                                    self.queue.insert(0, SearchPendingData::Error(f));
                                                    continue;
                                                }
                                            }
                                        }
                                    }

                                    Err(e) => {
                                        let f = std::io::Error::new(e.kind(), format!("Error reading dir {}: {}", root.display(), e));
                                        self.queue.insert(0, SearchPendingData::Error(f));
                                    }
                                }
                            }
                        }

                        Segment::Recurse => {
                            self.queue.insert(0, SearchPendingData::DirToExplore(root, depth + 1, true, 0));
                        }

                        Segment::Filter(re) => {
                            // Search all files, return the ones that match
                            let mut dirs: Vec<PathBuf> = Vec::new();

                            match fs::read_dir(&root) {
                                Ok(contents) => {
                                    for entry in contents {
                                        match entry {
                                            Ok(entry) => {
                                                let ft = entry.file_type().unwrap();
                                                let pb = entry.path();
                                                let fname = entry.file_name().to_string_lossy().to_string();

                                                if ft.is_file() {
                                                    if depth == self.segments.len() - 1 && re.is_match(&fname) {
                                                        self.queue.insert(0, SearchPendingData::File(pb));
                                                    }
                                                } else if ft.is_dir() {
                                                    let flnc = fname.to_lowercase();
                                                    if !self.ignore_dirs.iter().any(|ie| *ie == flnc) {
                                                        if re.is_match(&fname) {
                                                            if self.maxdepth == 0 || recurse_depth < self.maxdepth {
                                                                // If it's the last segment, we just return the directory
                                                                // Otherwise, we continue exploration in next loop
                                                                if depth == self.segments.len() - 1 {
                                                                    self.queue.insert(0, SearchPendingData::Dir(pb.clone()));
                                                                } else {
                                                                    self.queue.insert(0, SearchPendingData::DirToExplore(pb.clone(), depth + 1, false, 0));
                                                                }
                                                            }
                                                        }
                                                        dirs.push(pb);
                                                    }
                                                }
                                            }

                                            Err(e) => {
                                                let f = std::io::Error::new(e.kind(), format!("Error enumerating dir {}: {}", root.display(), e));
                                                self.queue.insert(0, SearchPendingData::Error(f));
                                                continue;
                                            }
                                        }
                                    }
                                }

                                Err(e) => {
                                    let f = std::io::Error::new(e.kind(), format!("Error reading dir {}: {}", root.display(), e));
                                    self.queue.insert(0, SearchPendingData::Error(f));
                                }
                            }

                            // Then if recurse mode, we also search in all subdirectories (already collected in dirs in previous loop to avoid enumerating directory twice)
                            if recurse && (self.maxdepth == 0 || recurse_depth < self.maxdepth) {
                                for dir in dirs {
                                    self.queue.insert(0, SearchPendingData::DirToExplore(dir, depth, true, recurse_depth + 1));
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
}
