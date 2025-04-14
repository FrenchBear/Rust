// rnormalizedates: Rust version of NormalizeDates, Normalizes dates in filenames, replace 'January 2020' by '2020-01'
//
// 2025-04-12	PV      First version

#![allow(unused)]

// standard library imports
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

// external crates imports
use getopt::Opt;
use myglob::{MyGlobMatch, MyGlobSearch};
use regex::Regex;
use terminal_size::{Width, terminal_size};
use unicode_normalization::UnicodeNormalization;

// -----------------------------------
// Submodules

mod devdata;
mod options;
mod tests;

use devdata::get_dev_data;
use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = "rnormalizedates";
const APP_VERSION: &str = "1.0.0";

// -----------------------------------
// Main

// Dev tests
fn main() {
    let date_patterns = DatePatterns::new();

    // for filefp in get_dev_data() {
    //     process_file(&PathBuf::from(filefp));
    // }
}

#[allow(unused)]
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

struct DatePatterns {
    re_date_ymd_head: Regex,
    re_date_dmdmy: Regex,
    re_date_mymy: Regex,
    re_date_mmy: Regex,
    re_date_dmy: Regex,
    re_date_my: Regex,
    re_date_ym: Regex,
    re_date_ymd: Regex,
}

impl DatePatterns {
    fn new() -> Self {
        // Prepare regex
        let months: [(&str, i32); 64] = [
            ("01", 1),
            ("Janvier", 1),
            ("Janv", 1),
            ("Jan", 1),
            ("January", 1),
            ("Février", 2),
            ("02", 2),
            ("Fevrier", 2),
            ("F vrier", 2),
            ("Fev", 2),
            ("Fév", 2),
            ("Feb", 2),
            ("February", 2),
            ("Mars", 3),
            ("03", 3),
            ("Mar", 3),
            ("March", 3),
            ("04", 4),
            ("Avril", 4),
            ("Avr", 4),
            ("Apr", 4),
            ("April", 4),
            ("05", 5),
            ("Mai", 5),
            ("May", 5),
            ("06", 6),
            ("Juin", 6),
            ("Jui", 6),
            ("Jun", 6),
            ("June", 6),
            ("07", 7),
            ("Juillet", 7),
            ("Juil", 7),
            ("Juill", 7),
            ("Jul", 7),
            ("July", 7),
            ("08", 8),
            ("Août", 8),
            ("Aout", 8),
            ("Ao t", 8),
            ("Aoû", 8),
            ("Aou", 8),
            ("Aug", 8),
            ("August", 8),
            ("09", 9),
            ("Septembre", 9),
            ("Sept", 9),
            ("Sep", 9),
            ("September", 9),
            ("10", 10),
            ("Octobre", 10),
            ("Oct", 10),
            ("October", 10),
            ("11", 11),
            ("Novembre", 11),
            ("Nov", 11),
            ("November", 11),
            ("12", 12),
            ("Décembre", 12),
            ("Decembre", 12),
            ("D cembre", 12),
            ("Dec", 12),
            ("Déc", 12),
            ("December", 12),
        ];

        fn get_month_num(month: &str, months: &[(&str, i32)]) -> i32 {
            if let Ok(m) = month.parse::<i32>() {
                if (1..=12).contains(&m) {
                    return m;
                }
            } else {
                let month_lc = month.to_lowercase();
                for &(month_name, month_num) in months.iter() {
                    if month_lc == month_name.to_lowercase() {
                        return month_num;
                    }
                }
            }
            0 // Not found
        }

        // Note: \b est une ancre de limite de mot (mais backspace dans une [classe])
        let mut month = String::new();
        let mut months_sorted = months.clone();
        months_sorted.sort_by_key(|k| -(k.0.len() as i32));
        for &(month_name, month_num) in months_sorted.iter() {
            month.push_str(if month.is_empty() { r"\b(" } else { "|" });
            month.push_str(month_name);
        }
        month.push_str(r")\b");
        let month = month.as_str();
        // Years from 1920
        let year = r"\b((?:19[2-9][0-9]|20[0-2][0-9])|(?:2[0-9]))(?:B?)\b"; // New version, 2020 and more only, absorb a B following a year
        let day = r"\b(1er|30|31|(?:0?[1-9])|[12][0-9])\b";

        let re_date_ymd_head =
            Regex::new((String::from("(?i)") + "^" + year + r"[ _\-]" + r"(0[1-9]|10|11|12)" + r"[_\-]" + day + r"[ _]*").as_str()).unwrap();

        let re_date_dmdmy =
            Regex::new((String::from("(?i)") + day + r" +" + month + r" *(au)? *" + day + r" +" + month + r" +" + year).as_str()).unwrap();
        let re_date_mymy = Regex::new((String::from("(?i)") + month + r" +" + year + r"( |-|à)+" + month + r" +" + year).as_str()).unwrap();
        let re_date_mmy = Regex::new((String::from("(?i)") + month + r"( |-|à)+" + month + r" +" + year).as_str()).unwrap();
        let re_date_dmy = Regex::new((String::from("(?i)") + day + r" +" + month + r" +" + year).as_str()).unwrap();
        let re_date_my = Regex::new((String::from("(?i)") + month + r" +" + year).as_str()).unwrap();
        let re_date_ym = Regex::new((String::from("(?i)") + year + r" +" + month).as_str()).unwrap();
        let re_date_ymd = Regex::new((String::from("(?i)") + year + r" +" + month + r" +" + day).as_str()).unwrap();

        // If name starts with a Ymd date, then move it to the end, and analyse remaining patterns

        DatePatterns {
            re_date_ymd_head,
            re_date_dmdmy,
            re_date_mymy,
            re_date_mmy,
            re_date_dmy,
            re_date_my,
            re_date_ym,
            re_date_ymd,
        }
    }
}

fn process_file(pb: &Path) {
    //println!("Processing {}", pb.display());

    let basename_original = pb.file_stem().expect("No stem??").to_string_lossy().into_owned();

    let mut base_name: String = basename_original.nfc().collect();
    base_name = base_name.replace('_', " ");
    base_name = base_name.replace("..", "$"); // Keep double dots
    base_name = base_name.replace(".", " "); // But replace simple dots by spaces
    base_name = base_name.replace("$", "..");
    base_name = base_name.replace("\u{FFFD}", " "); // Replacement character

    // Add starting/ending space to simplyfy some detections
    base_name = format!(" {} ", base_name);
    loop {
        let mut update = false;

        if base_name.contains("  ") {
            base_name = base_name.replace("  ", " ");
            update = true;
        }
        if base_name.contains("- -") {
            base_name = base_name.replace("- -", "-");
            update = true;
        }
        if base_name.contains("--") {
            base_name = base_name.replace("--", "-");
            update = true;
        }
        if icontains(&base_name, "PDF-NOTAG") {
            base_name = ireplace(&base_name, "PDF-NOTAG", "");
            update = true;
        }
        if icontains(&base_name, " FRENCH ") {
            base_name = ireplace(&base_name, " FRENCH ", " ");
            update = true;
        }
        if icontains(&base_name, " francais ") {
            base_name = ireplace(&base_name, " francais ", " ");
            update = true;
        }

        if !update {
            break;
        }
    }

    println!("{:70} «{}»", basename_original, base_name);
}

// Case-insensitive version of contains
fn icontains(s: &str, pattern: &str) -> bool {
    s.to_lowercase().contains(&pattern.to_lowercase())
}

// Case-insensitive version of replace
fn ireplace(s: &str, search: &str, replace: &str) -> String {
    if search.is_empty() {
        panic!("search can't be empty");
    }
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
