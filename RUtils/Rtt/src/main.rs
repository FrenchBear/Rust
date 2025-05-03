// rtt: Text type utility in Rust
//
// 2025-05-03	PV      First version

#![allow(unused)]

use std::{collections::HashMap, ops::AddAssign};
// standard library imports
use std::io;
use std::path::Path;
use std::process;
use std::time::Instant;

// external crates imports
use colored::*;
use myglob::{MyGlobMatch, MyGlobSearch};
use textautodecode::{TextAutoDecode, TextFileEncoding};

// -----------------------------------
// Submodules

mod options;
pub mod tests;

use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = "rtt";
const APP_VERSION: &str = "1.0.0";

const TEXT_EXT: [&str; 53] = [
    // Sources
    "awk",
    "c",
    "cpp",
    "cs",
    "fs",
    "go",
    "h",
    "java",
    "jl",
    "js",
    "lua",
    "py",
    "rs",
    "sql",
    "ts",
    "vb",
    "xaml",
    // VB6
    "bas",
    "frm",
    "cls",
    "ctl",
    "vbp",
    "vbg",
    // Projects
    "sln",
    "csproj",
    "vbproj",
    "fsproj",
    "pyproj",
    "vcxproj",
    // Misc
    "appxmanifest",
    "clang-format",
    "classpath",
    "ruleset",
    "editorconfig",
    "gitignore",
    "globalconfig",
    "resx",
    "targets",
    "pubxml",
    "filters",
    // Config
    "ini",
    "xml",
    "yml",
    "yaml",
    "json",
    "toml",
    // Scripts
    "bat",
    "cmd",
    "ps1",
    "sh",
    "vbs",
    // Text
    "txt",
    "md",
];

// ==============================================================================================
// Main

#[derive(Debug, Default)]
struct DataBag {
    files_types: FileTypeCounts,
    eol_styles: EOLStyleCounts,
    counters: HashMap<String, HashMap<String, FolderExtCounts>>,
}

#[derive(Debug, Default)]
struct FolderExtCounts {
    files_types: FileTypeCounts,
    eol_styles: EOLStyleCounts,
}

#[derive(Debug, Default)]
struct FileTypeCounts {
    total: usize,

    empty: usize,
    ascii: usize,
    utf8: usize,
    utf16: usize,
    eightbit: usize,
    nontext: usize,
}

#[derive(Debug, Default)]
struct EOLStyleCounts {
    total: usize,

    windows: usize,
    unix: usize,
    mac: usize,
    mixed: usize,
}

fn main() {
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

    let mut b = DataBag { ..Default::default() };

    for source in options.sources.iter() {
        let resgs = MyGlobSearch::new(source).autorecurse(options.autorecurse).compile();
        match resgs {
            Ok(gs) => {
                for ma in gs.explore_iter() {
                    match ma {
                        MyGlobMatch::File(pb) => {
                            process_file(&mut b, &pb, &options);
                        }

                        // We ignore matching directories in rgrep, we only look for files
                        MyGlobMatch::Dir(_) => {}

                        MyGlobMatch::Error(err) => {
                            if options.verbose {
                                eprintln!("{APP_NAME}: error {}", err);
                            }
                        }
                    }
                }
            }

            Err(e) => {
                eprintln!("{APP_NAME}: Error building MyGlob: {:?}", e);
            }
        }
    }

    // Warnings per folder+extension
    if !options.sources.is_empty() {
        let mut header_printed = false;
        let mut fk: Vec<&String> = b.counters.keys().collect();
        fk.sort();
        for f in fk {
            let mut ek: Vec<&String> = b.counters[f].keys().collect();
            ek.sort();
            for e in ek {
                let mut file_printed = false;

                let ft = &b.counters[f][e].files_types;
                if ft.utf8 > 0 && ft.utf16 > 0 || ft.utf8 > 0 && ft.eightbit > 0 || ft.utf16 > 0 && ft.ascii > 0 || ft.utf16 > 0 && ft.eightbit > 0 {
                    if !header_printed {
                        println!("\nMixed folder contents:");
                        header_printed = true;
                    }
                    print!("{}, ext .{}: ", f, e);
                    file_printed = true;

                    print!("{}", "Mixed text file contents".red().bold());
                }

                let eol = &b.counters[f][e].eol_styles;
                if eol.total > 1 {
                    if eol.windows > 0 && eol.unix > 0 || eol.windows > 0 && eol.mac > 0 || eol.unix > 0 && eol.mac > 0 {
                        if !header_printed {
                            println!("\nMixed folder contents:");
                            header_printed = true;
                        }

                        if file_printed {
                            print!(", ");
                        } else {
                            file_printed = true;
                            print!("{}, ext .{}: ", f, e);
                        }
                        print!("{}", "Mixed EOF styles".red().bold());
                    }
                }

                if file_printed {
                    println!();
                }
            }
        }
    }

    // If no source has been provided, use stdin
    if options.sources.is_empty() {
        if options.verbose {
            println!("Reading from stdin");
        }
        let s = io::read_to_string(io::stdin()).unwrap();
        let eol = get_eol(s.as_str());
        println!("(stdin): {:?}", eol);
    }

    let duration = start.elapsed();

    if options.verbose {
        println!("\nGlobal stats:");
        print_files_types_counts(&b.files_types);
        print_eol_styles_counts(&b.eol_styles);

        println!("\n{} files(s) searched in {:.3}s", b.files_types.total, duration.as_secs_f64());
        // ToDo: print other stats and warnings par folder+extension
    }
}

fn print_files_types_counts(f: &FileTypeCounts) {
    let tot = f.empty + f.ascii + f.utf8 + f.utf16 + f.eightbit + f.nontext;
    println!("Total files: {}", tot);
    if f.empty > 0 {
        println!("- Empty: {}", f.empty)
    }
    if f.ascii > 0 {
        println!("- ASCII: {}", f.ascii)
    }
    if f.utf8 > 0 {
        println!("- UTF-8: {}", f.utf8)
    }
    if f.utf16 > 0 {
        println!("- UTF-16: {}", f.utf16)
    }
    if f.eightbit > 0 {
        println!("- 8-Bit: {}", f.eightbit)
    }
    if f.nontext > 0 {
        println!("- Non text: {}", f.nontext)
    }
}

fn print_eol_styles_counts(e: &EOLStyleCounts) {
    let tot = e.windows + e.unix + e.mac + e.mixed;
    println!("Total EOL styles: {}", tot);
    if e.windows > 0 {
        println!("- Windows: {}", e.windows)
    }
    if e.unix > 0 {
        println!("- Unix: {}", e.unix)
    }
    if e.mac > 0 {
        println!("- Mac: {}", e.mac)
    }
    if e.mixed > 0 {
        println!("- Mixed: {}", e.mixed)
    }
}

fn warning(path: &Path, msg: &str) {
    println!("{}: {}", path.display(), msg);
}

/// First step processing a file, read text content from path and call process_text.
fn process_file(b: &mut DataBag, path: &Path, options: &Options) {
    let res = TextAutoDecode::read_text_file(path);
    b.files_types.total += 1;
    match res {
        Ok(tad) => {
            let ext = match path.extension() {
                Some(e) => e.to_string_lossy().to_lowercase(),
                None => String::new(),
            };

            // Collect stats per directory+ext
            let dir = match path.parent() {
                Some(p) => p.to_string_lossy().to_lowercase(),
                None => String::new(),
            };

            let fc = b.counters.entry(dir).or_default().entry(ext.clone()).or_default();

            let (enc, war) = match tad.encoding {
                TextFileEncoding::NotText => {
                    b.files_types.nontext += 1;
                    fc.files_types.nontext += 1;
                    // Silently ignore non-text files, but check whether it should have contained text
                    if TEXT_EXT.contains(&ext.as_str()) {
                        warning(
                            path,
                            format!("Non-text file detected, but extension {ext} is usually a text file").as_str(),
                        );
                    }
                    return;
                }
                TextFileEncoding::Empty => {
                    b.files_types.empty += 1;
                    // Don't collect infos per folder+ext for empty files
                    warning(path, "Empty file");
                    // No need to continue if it's empty
                    return;
                }
                TextFileEncoding::ASCII => {
                    b.files_types.ascii += 1;
                    fc.files_types.ascii += 1;
                    ("ASCII", "")
                }
                TextFileEncoding::EightBit => {
                    b.files_types.eightbit += 1;
                    fc.files_types.eightbit += 1;
                    ("8-Bit text", "")
                }
                TextFileEncoding::UTF8 | TextFileEncoding::UTF8BOM => {
                    b.files_types.utf8 += 1;
                    fc.files_types.utf8 += 1;
                    ("UTF-8", if tad.encoding == TextFileEncoding::UTF8BOM { "with BOM" } else { "" })
                }
                TextFileEncoding::UTF16LE | TextFileEncoding::UTF16BE | TextFileEncoding::UTF16LEBOM | TextFileEncoding::UTF16BEBOM => {
                    b.files_types.utf16 += 1;
                    fc.files_types.utf16 += 1;
                    (
                        if tad.encoding == TextFileEncoding::UTF16LE || tad.encoding == TextFileEncoding::UTF16LEBOM {
                            "UTF-16 LE"
                        } else {
                            "UTF-16 BE"
                        },
                        if tad.encoding == TextFileEncoding::UTF16LE || tad.encoding == TextFileEncoding::UTF16BE {
                            "without BOM"
                        } else {
                            ""
                        },
                    )
                }
                _ => unreachable!(),
            };

            let eol = get_eol(tad.text.unwrap().as_str());

            fc.eol_styles.windows += eol.windows;
            fc.eol_styles.unix += eol.unix;
            fc.eol_styles.mac += eol.mac;
            fc.eol_styles.mixed += eol.mixed;
            fc.eol_styles.total += eol.total;

            b.eol_styles.windows += eol.windows;
            b.eol_styles.unix += eol.unix;
            b.eol_styles.mac += eol.mac;
            b.eol_styles.mixed += eol.mixed;
            b.eol_styles.total += eol.total;
            
            print!("{}: {}", path.display(), enc);
            if !war.is_empty() {
                print!(" {}", war.red().bold())
            }
            print!(", ");

            if eol.mixed > 0 {
                print!("{}", "Mixed EOL styles".red().bold());
            } else if eol.windows + eol.unix + eol.mac == 0 {
                print!("No EOL detected");
            } else if eol.windows > 0 {
                print!("Windows");
            } else if eol.unix > 0 {
                print!("Unix");
            } else if eol.mac > 0 {
                print!("Mac");
            }
            println!();
        }
        Err(e) => {
            eprintln!("*** Error reading file {}: {}", path.display(), e);
        }
    }
}

/// Core rgrep process, search for re in txt, read from filename, according to options.
fn get_eol(txt: &str) -> EOLStyleCounts {
    let mut eol = EOLStyleCounts { ..Default::default() };
    let mut iter = txt.as_bytes().iter().peekable();
    while let Some(c) = iter.next() {
        match c {
            b'\n' => eol.unix |= 1,
            b'\r' => {
                if let Some(&next_c) = iter.peek() {
                    if *next_c == b'\n' {
                        iter.next();
                        eol.windows |= 1;
                        continue;
                    }
                }
                eol.mac |= 1;
            }
            _ => {}
        }
    }

    // Don't count files without EOL detected in total
    eol.total += eol.windows + eol.unix + eol.mac;

    // Helper
    eol.mixed = if eol.windows + eol.unix + eol.mac > 1 { 1 } else { 0 };

    eol
}
