// l47_checkdates: Check dates in source files headers
//
// 2025-04-21	PV      First version

#![allow(unused)]

// standard library imports
use std::path::{Path, PathBuf};
use std::process;
use std::sync::LazyLock;
use std::time::Instant;
use std::{collections::HashMap, fmt::Debug};

use chrono::NaiveDate;
// external crates imports
use myglob::{MyGlobMatch, MyGlobSearch};
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

// -----------------------------------
// Submodules

mod decode_encoding;
mod logging;

use decode_encoding::*;
use logging::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = "check_dates";
const APP_VERSION: &str = "1.0.1";

// -----------------------------------
// Main

#[derive(Debug, Default)]
struct DataBag {
    files_count: usize,
    errors_count: usize,
    comment_lines_count: usize,
}

fn main() {
    let globstrsources: Vec<String> = vec![r"C:\Development\**\*.{cs,rs,py,fs,c,cpp,go,java,js,jl,lua,ts,vb}".to_string()];

    // Prepare log writer
    let mut writer = logging::new(false);
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
            format!("Average comment header size: {:.1} line(s)", b.comment_lines_count as f64 / b.files_count as f64).as_str(),
        );
    }
}

fn process_file(writer: &mut LogWriter, b: &mut DataBag, p: &Path) {
    let res = read_text_file(p);
    match res {
        Ok((Some(s), _)) => {
            let extension = p.extension().map(|p| p.to_str().unwrap()).unwrap_or("").to_ascii_lowercase();
            let comment = match extension.as_str() {
                "cs" | "c" | "cpp" | "rs" | "fs" | "go" | "java" | "js" | "ts" => "//",
                "py" | "jl" => "#",
                "lua" => "--",
                "vb" => "'",
                _ => {
                    b.errors_count += 1;
                    logln(writer, format!("*** Unknown/unsupported extension: {}", p.display()).as_str());
                    return;
                }
            };

            process_text(writer, p, s.as_str(), comment, b);
        }
        Ok((None, _)) => {
            // Non-text files are ignored
            // println!("{APP_NAME}: ignored non-text file {}", pb.display());
        }
        Err(e) => {
            b.errors_count += 1;
            logln(writer, format!("*** Error reading file {}: {}", p.display(), e).as_str());
        }
    }
}

fn process_text(writer: &mut LogWriter, p: &Path, source: &str, comment: &str, b: &mut DataBag) {
    static DATE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+(\d+)-(\d+)-(\d+)\s").unwrap());

    let mut last_date: Option<NaiveDate> = None;

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
