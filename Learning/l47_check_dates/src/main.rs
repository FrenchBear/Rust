 // l47_check_dates: Check dates in source files headers
//
// 2025-04-21	PV      First version
// 2025-05-02   PV      Use textautodecode crate instead of decode_encoding module
// 2025-05-14   PV      1.2 Use logging crate instead of logging module

//#![allow(unused)]

// standard library imports
use std::path::Path;
use std::process;
use std::sync::LazyLock;
use std::time::Instant;
use std::{collections::HashMap, fmt::Debug};

// external crates imports
use myglob::{MyGlobMatch, MyGlobSearch};
use regex::Regex;
use textautodecode::{TextAutoDecode, TextFileEncoding};
use logging::{LogWriter, log, logln};

// -----------------------------------
// Global constants

const APP_NAME: &str = "check_dates";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// -----------------------------------
// Main

#[derive(Debug, Default)]
struct DataBag {
    files_count: usize,
    errors_count: usize,
    comment_lines_count: usize,
    ext_counter: HashMap<String, usize>,
}

fn main() {
    let globstrsources: Vec<String> = vec![r"C:\Development\**\*.{awk,c,cpp,cs,fs,go,h,java,jl,js,lua,py,rs,sql,ts,vb}".to_string()];

    // Prepare log writer
    let mut writer = logging::new(APP_NAME, APP_VERSION, true);

    let mut b = DataBag { ..Default::default() };

    let start = Instant::now();

    // Convert String sources into MyGlobSearch structs
    let mut sources: Vec<(&String, MyGlobSearch)> = Vec::new();
    for source in globstrsources.iter() {
        let resgs = MyGlobSearch::build(source);
        match resgs {
            Ok(gs) => sources.push((source, gs)),
            Err(e) => {
                logln(&mut writer, format!("*** Error building MyGlob: {:?}", e).as_str());
            }
        }
    }
    if sources.is_empty() {
        logln(&mut writer, "*** No source to process, aborting.");
        process::exit(1);
    }

    log(&mut writer, "\nSources(s): ");
    for source in sources.iter() {
        logln(&mut writer, format!("- {}", source.0).as_str());
    }

    for gs in sources.iter() {
        for ma in gs.1.explore_iter() {
            match ma {
                MyGlobMatch::File(pb) => process_file(&mut writer, &mut b, &pb),

                MyGlobMatch::Dir(_) => {}

                MyGlobMatch::Error(err) => {
                    logln(&mut writer, format!("{APP_NAME}: MyGlobMatch error {}", err).as_str());
                }
            }
        }
    }

    if b.files_count == 0 {
        logln(&mut writer, "*** No file found, nothing to report.");
    } else {
        let duration = start.elapsed();
        logln(&mut writer, "");
        log(&mut writer, format!("{} files(s)", b.files_count).as_str());
        log(&mut writer, format!(", {} error(s)", b.errors_count).as_str());
        logln(&mut writer, format!(" found in {:.3}s", duration.as_secs_f64()).as_str());
        logln(
            &mut writer,
            format!(
                "Average comment header size: {:.2} line(s)",
                b.comment_lines_count as f64 / b.files_count as f64
            )
            .as_str(),
        );

        println!();
        let mut kvp = b.ext_counter.into_iter().collect::<Vec<_>>();
        kvp.sort_by_key(|(_k, v)| -(*v as i32));
        for (ext, cnt) in kvp {
            println!("{:5} {:5}", ext, cnt);
        }
    }
}

fn process_file(writer: &mut LogWriter, b: &mut DataBag, p: &Path) {
    let res = TextAutoDecode::read_text_file(p);
    match res {
        Ok(tad) => {
            if tad.encoding == TextFileEncoding::NotText {
                // Non-text files are silently ignored
                // Note that this can include some source files with many semi-graphic characters such as:
                // - C:\Development\GitHub\Python\Learning\041_Unicode\BoxesAndSymbols.py
                // - C:\Development\GitHub\Julia\Learning\17_operators\test_Sm.jl
                // - C:\Development\GitHub\Go\src\golang.org\x\text\collate\tools\colcmp\chars.go

                // b.errors_count += 1;
                // logln(writer, format!("*** Non-text file: {}", p.display()).as_str());
            } else {
                let extension = p.extension().map(|p| p.to_str().unwrap()).unwrap_or("").to_ascii_lowercase();
                let comment = match extension.as_str() {
                    "cs" | "c" | "h" | "cpp" | "rs" | "fs" | "go" | "java" | "js" | "ts" => "//",
                    "py" | "jl" | "awk" => "#",
                    "lua" | "sql" => "--",
                    "vb" => "'",
                    _ => {
                        b.errors_count += 1;
                        logln(writer, format!("*** Unknown/unsupported extension: {}", p.display()).as_str());
                        return;
                    }
                };

                // Count extension
                let e = b.ext_counter.entry(extension);
                *e.or_insert(0) += 1;

                process_text(writer, p, tad.text.unwrap().as_str(), comment, b);
            }
        }
        Err(e) => {
            b.errors_count += 1;
            logln(writer, format!("*** Error reading file {}: {}", p.display(), e).as_str());
        }
    }
}

fn process_text(writer: &mut LogWriter, p: &Path, source: &str, comment: &str, b: &mut DataBag) {
    static DATE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+(\d+)-(\d+)-(\d+)\s").unwrap());

    b.files_count += 1;
    let mut clc = 0;
    for line in source.lines() {
        if !line.starts_with(comment) && !line.trim().is_empty() {
            b.comment_lines_count += clc;
            return;
        }

        if line.len() < 10 {
            continue;
        }
        if let Some(caps) = DATE.captures(&line[comment.len()..]) {
            let y = caps[1].parse::<i32>().unwrap();
            let m = caps[2].parse::<u32>().unwrap();
            let d = caps[3].parse::<u32>().unwrap();

            if !(1..=31).contains(&d) || !(1..=12).contains(&m) || !(1980..=2025).contains(&y) {
                logln(writer, format!("*** Invalid date: {}\n    {}", p.display(), line).as_str());
                b.errors_count += 1;
                return;
            }

            // There are too many date sequence issues, and it's painful to fix, so for now, ignore.
            /*
            let d = NaiveDate::from_ymd_opt(y, m, d);
            if d.is_none() {
                logln(writer, format!("*** Invalid date: {}\n    {}", p.display(), line).as_str());
                b.errors_count += 1;
                return;
            }

            if last_date.is_some() && d<last_date {
                logln(writer, format!("*** Invalid date sequence: {}\n    {}\n    {}", p.display(), last_date.unwrap(), d.unwrap()).as_str());
                b.errors_count += 1;
                return;
            }
            last_date = d;
            */
        }

        clc += 1;
    }
}
