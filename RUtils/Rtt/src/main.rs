// rtt: Text type utility in Rust
//
// 2025-05-03	PV      First version

#![allow(unused)]

// standard library imports
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;
use std::time::Instant;

// external crates imports
use colored::*;
use myglob::{MyGlobMatch, MyGlobSearch};
use tempfile::Builder;
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

// These extensions should indicate a text content
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

// fn main() -> Result<(), io::Error> {
//     let model: [u8; 24] = [
//         0x41, 0x00, // A
//         0x42, 0x00, // B
//         0x43, 0x00, // C
//         0x44, 0x00, // D
//         0x45, 0x00, // E
//         b'\n', 0x00, // Unix EOL
//         0x61, 0x00, // a
//         0x62, 0x00, // b
//         0x63, 0x00, // c
//         0x64, 0x00, // d
//         0x65, 0x00, // e
//         b'\n', 0x00 // Unix EOL
//     ];

//     let mut temp_file = Builder::new().tempfile()?;
//     temp_file.write_all(&model);
//     let o = Options {..Default::default()};
//     let mut b = DataBag {..Default::default()};
//     let res = process_file(&mut b, temp_file.path(), Path::new("(test utf16le)"), &o);

//     assert_eq!(res.as_str(), "(test utf16le): UTF-16 LE «without BOM», Unix");

//     assert_eq!(b.files_types.total, 1);
//     assert_eq!(b.files_types.empty, 0);
//     assert_eq!(b.files_types.ascii, 0);
//     assert_eq!(b.files_types.utf8, 0);
//     assert_eq!(b.files_types.utf16, 1);
//     assert_eq!(b.files_types.eightbit, 0);
//     assert_eq!(b.files_types.nontext, 0);

//     assert_eq!(b.eol_styles.total, 1);
//     assert_eq!(b.eol_styles.windows, 0);
//     assert_eq!(b.eol_styles.unix, 1);
//     assert_eq!(b.eol_styles.mac, 0);
//     assert_eq!(b.eol_styles.mixed, 0);

//     Ok(())
// }

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
                            print_result(process_file(&mut b, &pb, &pb, &options).as_str(), &options);
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
        process_stdin(&mut b, &options);
    }

    let duration = start.elapsed();

    if options.verbose {
        println!("\nGlobal stats:");
        print_files_types_counts(&b.files_types);
        print_eol_styles_counts(&b.eol_styles);

        println!("\n{} files(s) searched in {:.3}s", b.files_types.total, duration.as_secs_f64());
    }
}

fn process_stdin(b: &mut DataBag, options: &Options) -> Result<(), io::Error> {
    if options.verbose {
        println!("Reading from stdin");
    }

    // Create a temporary file.  The file will be automatically deleted when temp_file goes out of scope.
    let mut temp_file = Builder::new().tempfile()?;

    // Read all bytes from stdin until EOF.
    let mut stdin = io::stdin();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer)?;

    // Write the bytes to the temporary file.
    std::io::Write::write_all(&mut temp_file, &buffer)?;
    temp_file.flush()?; // Ensure all bytes are written to the file.

    process_file(b, temp_file.path(), Path::new("(stdin)"), options);

    Ok(())
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

// Print in red parts between « »
fn print_result(msg: &str, options: &Options) {
    if !options.show_only_warnings || msg.find('«').is_some() {
        print_result_core(msg);
    }
}

fn print_result_core(msg: &str) {
    let mut p0 = 0;
    loop {
        let p1 = find_from_position(&msg, '«', p0);
        if p1.is_none() {
            println!("{}", &msg[p0..]);
            return;
        }
        let p1 = p1.unwrap();

        if p1 > p0 {
            print!("{}", &msg[p0..p1]);
        }

        let p2 = find_from_position(&msg, '»', p1 + '»'.len_utf8()).expect(format!("Internal error, unbalanced « » in {msg}").as_str());
        print!("{}", &msg[p1 + '«'.len_utf8()..p2].red().bold());
        p0 = p2 + '»'.len_utf8()
    }
}

// fn test_print_result() {
//     print_result_core("");
//     print_result_core("«»");
//     print_result_core("Once upon a time");
//     print_result_core("Once «upon» a time");
//     print_result_core("«Once upon a time»");
//     print_result_core("«Once» upon «a» time");
//     print_result_core("«O»«n»«c»«e» «u»p«o»n «a» t«i»m«e»");
// }

/// Similar to (&str).find(char), but starts search at byte index start_position.
/// Returns the byte index of the first character of this string slice that matches the pattern.
/// Returns None if the pattern doesn't match.
fn find_from_position(s: &str, pattern: char, start_position: usize) -> Option<usize> {
    if start_position >= s.len() {
        return None; // Start position is out of bounds
    }

    let search_slice = &s[start_position..];
    // Note that the following map is NOT the usual iterator map, but Option::map
    // Maps an Option<T> to Option<U> by applying a function to a contained value (if Some) or returns None (if None).
    search_slice.find(pattern).map(|relative_position| start_position + relative_position)
}

/// First step processing a file, read text content from path and call process_text.
fn process_file(b: &mut DataBag, path_for_read: &Path, path_for_name: &Path, options: &Options) -> String {
    let mut res = String::new();
    let tad_res = TextAutoDecode::read_text_file(path_for_read);

    b.files_types.total += 1;
    match tad_res {
        Ok(tad) => {
            let ext = match path_for_name.extension() {
                Some(e) => e.to_string_lossy().to_lowercase(),
                None => String::new(),
            };

            // Collect stats per directory+ext
            let dir = match path_for_name.parent() {
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
                        return format!(
                            "{}: «Non-text file detected, but extension {ext} is usually a text file»",
                            path_for_name.display()
                        );
                    }
                    return res;
                }
                TextFileEncoding::Empty => {
                    b.files_types.empty += 1;
                    // Don't collect infos per folder+ext for empty files
                    // No need to continue if it's empty
                    return format!("{}: «Empty file»", path_for_name.display());
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

            res.push_str(format!("{}: {}", path_for_name.display(), enc).as_str());
            if !war.is_empty() {
                res.push_str(format!(" «{}»", war).as_str());
            }
            res.push_str(", ");

            if eol.mixed > 0 {
                res.push_str("«Mixed EOL styles»");
            } else if eol.windows + eol.unix + eol.mac == 0 {
                res.push_str("No EOL detected");
            } else if eol.windows > 0 {
                res.push_str("Windows");
            } else if eol.unix > 0 {
                res.push_str("Unix");
            } else if eol.mac > 0 {
                res.push_str("Mac");
            }
        }
        Err(e) => {
            eprintln!("*** Error reading file {}: {}", path_for_name.display(), e);
        }
    }

    res
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
    if eol.windows + eol.unix + eol.mac > 0 {
        eol.total += 1;
    }

    // Helper
    if eol.windows + eol.unix + eol.mac > 1 {
        eol.mixed = 1;
    }

    eol
}
