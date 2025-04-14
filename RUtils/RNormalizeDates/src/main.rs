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

const MONTHS: [(&str, i32); 77] = [
    ("01", 1),
    ("Janvier", 1),
    ("Janv", 1),
    ("Jan", 1),
    ("January", 1),
    ("New year", 1),
    ("Février", 2),
    ("02", 2),
    ("Fevrier", 2),
    ("F vrier", 2),
    ("Fvrier", 2),
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
    ("Printemps", 13),
    ("Spring", 13),
    ("Été", 14),
    ("Eté", 14),
    ("Ete", 14),
    ("Summer", 14),
    ("Autonne", 15),
    ("Autumn", 15),
    ("Fall", 15),
    ("Hiver", 16),
    ("Winter", 16),
];

// -----------------------------------
// Main

// Dev tests
fn main() {
    let dp = DatePatterns::new();

    // let c1opt = dp.re_date_ymd_head.captures("2023-02-10 Echappee Belle Magazine");
    // if let Some(c1) = c1opt {
    //     let cf = &c1[0];
    //     let y = get_year_num(&c1[1]);
    //     let m = get_month_num(&c1[2]);
    //     let d = get_day_num(&c1[3]);

    //     println!("full: «{cf}»  y: «{y}», m: «{m}», d: «{d}»")
    // }

    for filefp in get_dev_data() {
        process_file(&PathBuf::from(filefp), &dp);
    }
}

// 2-digit pivot at 50: <50=19xx, >50=20xx
fn get_year_num(year: &str) -> i32 {
    let y = year.parse::<i32>().unwrap();
    if y > 100 {
        y
    } else if y > 50 {
        y + 1900
    } else {
        y + 2000
    }
}

fn get_month_num(month: &str) -> i32 {
    if let Ok(m) = month.parse::<i32>() {
        if (1..=12).contains(&m) {
            return m;
        }
    } else {
        let month_lc = month.to_lowercase();
        for &(month_name, month_num) in MONTHS.iter() {
            if month_lc == month_name.to_lowercase() {
                return month_num;
            }
        }
    }
    panic!("Invalid month {}", month);
}

fn get_day_num(day: &str) -> i32 {
    if let Ok(d) = day.parse::<i32>() {
        if (1..=31).contains(&d) {
            return d;
        }
    } else if day.to_lowercase() == "1er" {
        return 1;
    }
    panic!("Invalid day {}", day);
}

fn get_month_name(m: i32) -> &'static str {
    assert!((1..=16).contains(&m));
    [
        "01",
        "02",
        "03",
        "04",
        "05",
        "06",
        "07",
        "08",
        "09",
        "10",
        "11",
        "12",
        "Printemps",
        "Été",
        "Autonne",
        "Hiver",
    ][(m - 1) as usize]
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

    let date_patterns = DatePatterns::new();
    for source in options.sources.iter() {
        let resgs = MyGlobSearch::new(source).autorecurse(true).compile();
        match resgs {
            Ok(gs) => {
                for ma in gs.explore_iter() {
                    match ma {
                        MyGlobMatch::File(pb) => {
                            process_file(&pb, &date_patterns);
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
    re_date_mmmy: Regex,
    re_date_mymmy: Regex,
    re_date_mmymy: Regex,
    re_date_mmy: Regex,
    re_date_dmy: Regex,
    re_date_my: Regex,
    re_date_ym: Regex,
    re_date_ymd: Regex,
    re_date_ymm: Regex,
    re_date_ynm: Regex,
}

impl DatePatterns {
    fn new() -> Self {
        // Prepare regex
        // Note: \b est une ancre de limite de mot (mais backspace dans une [classe])
        let mut month = String::new();
        let mut months_sorted = MONTHS.clone();
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

        let re_date_ymd_head = Regex::new((String::from("(?i)") + "^ " + year + "[ -]" + "(0[1-9]|10|11|12)" + "[ -]" + day + " ").as_str()).unwrap();
        let re_date_ynm = Regex::new((String::from("(?i)") + year + r" +№(\d+) +" + month).as_str()).unwrap();

        let re_date_dmdmy =
            Regex::new((String::from("(?i)") + day + " +" + month + " *(?:au)? *" + day + " +" + month + " +" + year).as_str()).unwrap();
        let re_date_mymy = Regex::new((String::from("(?i)") + month + " +" + year + "[ à-]+" + month + " +" + year).as_str()).unwrap();

        let re_date_mmmy = Regex::new((String::from("(?i)") + month + "-" + month + "-" + month + " +" + year).as_str()).unwrap();
        let re_date_mymmy = Regex::new((String::from("(?i)") + month + " +" + year + " +" + month + "-" + month + " +" + year).as_str()).unwrap();
        let re_date_mmymy = Regex::new((String::from("(?i)") + month + "-" + month + " +" + year + " +" + month + " +" + year).as_str()).unwrap();

        let re_date_mmy = Regex::new((String::from("(?i)") + month + "[ à-]+" + month + " +" + year).as_str()).unwrap();
        let re_date_dmy = Regex::new((String::from("(?i)") + day + " +" + month + " +" + year).as_str()).unwrap();
        let re_date_my = Regex::new((String::from("(?i)") + month + " +" + year).as_str()).unwrap();
        let re_date_ym = Regex::new((String::from("(?i)") + year + " +" + month).as_str()).unwrap();
        let re_date_ymd = Regex::new((String::from("(?i)") + year + " +" + month + " +" + day).as_str()).unwrap();
        let re_date_ymm = Regex::new((String::from("(?i)") + year + "[ -]+" + month + "-" + day).as_str()).unwrap();

        // If name starts with a Ymd date, then move it to the end, and analyse remaining patterns

        DatePatterns {
            re_date_ymd_head,
            re_date_dmdmy,
            re_date_mymy,

            re_date_mmmy,
            re_date_mymmy,
            re_date_mmymy,

            re_date_mmy,
            re_date_dmy,
            re_date_my,
            re_date_ym,
            re_date_ymd,
            re_date_ymm,

            re_date_ynm,
        }
    }
}

fn process_file(pb: &Path, dp: &DatePatterns) {
    //println!("Processing {}", pb.display());

    let filename_original = pb.file_name().unwrap().to_string_lossy().into_owned();
    let stem_original = pb.file_stem().expect("No stem??").to_string_lossy().into_owned();
    let extension = pb.extension().unwrap().to_ascii_lowercase();

    let stem = apply_transformations(&stem_original, dp);
}

fn apply_transformations(stem_original: &str, dp: &DatePatterns) -> String
{
    let mut stem: String = stem_original.nfc().collect();
    stem = stem.replace('_', " ");
    stem = stem.replace("..", "$"); // Keep double dots
    stem = stem.replace(".", " "); // But replace simple dots by spaces
    stem = stem.replace("$", "..");
    stem = stem.replace("\u{FFFD}", " "); // Replacement character

    // Add starting/ending space to simplyfy some detections
    stem = format!(" {} ", stem);
    loop {
        let mut update = false;

        if stem.contains("  ") {
            stem = stem.replace("  ", " ");
            update = true;
        }
        if stem.contains("- -") {
            stem = stem.replace("- -", "-");
            update = true;
        }
        if stem.contains("--") {
            stem = stem.replace("--", "-");
            update = true;
        }
        if icontains(&stem, "PDF-NOTAG") {
            stem = ireplace(&stem, "PDF-NOTAG", "");
            update = true;
        }
        if icontains(&stem, " FRENCH ") {
            stem = ireplace(&stem, " FRENCH ", " ");
            update = true;
        }
        if icontains(&stem, " francais ") {
            stem = ireplace(&stem, " francais ", " ");
            update = true;
        }

        if !update {
            break;
        }
    }

    let mut start = 0;
    let mut len = 0;
    let mut res = String::new();
    let mut trans: &'static str = "";

    // If name starts with a Ymd date, then move it to the end, and analyse remaining patterns
    if let Some(caps) = dp.re_date_ymd_head.captures(&stem) {
        let cf = &caps[0];
        let y = get_year_num(&caps[1]);
        let m = get_month_num(&caps[2]);
        let d = get_day_num(&caps[3]);
        // Special case, generate directly new version of stem without res intermediate
        stem = format!(" {}- {}-{}-{:02} ", &stem[cf.len()..], y, get_month_name(m), d);
        trans = "ymd_head";
    } else if let Some(caps) = dp.re_date_mmmy.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let m1 = get_month_num(&caps[1]);
        let m3 = get_month_num(&caps[3]);
        let y = get_year_num(&caps[4]);
        res = format!("{}-{}..{}", y, get_month_name(m1), get_month_name(m3));
        trans = "mmmy"
    } else if let Some(caps) = dp.re_date_mymmy.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let m1 = get_month_num(&caps[1]);
        let y1 = get_year_num(&caps[2]);
        let m3 = get_month_num(&caps[4]);
        let y2 = get_year_num(&caps[5]);
        res = format!("{}-{}..{}-{}", y1, get_month_name(m1), y2, get_month_name(m3));
        trans = "mymmy"
    } else if let Some(caps) = dp.re_date_mmymy.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let m1 = get_month_num(&caps[1]);
        let y1 = get_year_num(&caps[3]);
        let m3 = get_month_num(&caps[4]);
        let y2 = get_year_num(&caps[5]);
        res = format!("{}-{}..{}-{}", y1, get_month_name(m1), y2, get_month_name(m3));
        trans = "mmymy"
    } else if let Some(caps) = dp.re_date_mymy.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let m1 = get_month_num(&caps[1]);
        let y1 = get_year_num(&caps[2]);
        let m2 = get_month_num(&caps[3]);
        let y2 = get_year_num(&caps[4]);
        res = format!("{}-{}..{}-{}", y1, get_month_name(m1), y2, get_month_name(m2));
        trans = "mymy"
    } else if let Some(caps) = dp.re_date_dmdmy.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let d1 = get_day_num(&caps[1]);
        let m1 = get_month_num(&caps[2]);
        let d2 = get_day_num(&caps[3]);
        let m2 = get_month_num(&caps[4]);
        let y = get_year_num(&caps[5]);
        res = format!("{}-{}-{:02}..{}-{:02}", y, get_month_name(m1), d1, get_month_name(m2), d2);
        trans = "dmdmy"
    } else if let Some(caps) = dp.re_date_dmy.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let d = get_day_num(&caps[1]);
        let m = get_month_num(&caps[2]);
        let y = get_year_num(&caps[3]);
        res = format!("{}-{}-{:02}", y, get_month_name(m), d);
        trans = "dmy"
    } else if let Some(caps) = dp.re_date_mmy.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let m1 = get_month_num(&caps[1]);
        let m2 = get_month_num(&caps[2]);
        let y = get_year_num(&caps[3]);
        res = format!("{}-{}..{}", y, get_month_name(m1), get_month_name(m2));
        trans = "mmy"
    } else if let Some(caps) = dp.re_date_my.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let m = get_month_num(&caps[1]);
        let y = get_year_num(&caps[2]);
        res = format!("{}-{}", y, get_month_name(m));
        trans = "my"
    } else if let Some(caps) = dp.re_date_ymd.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y = get_year_num(&caps[1]);
        let m = get_month_num(&caps[2]);
        let d = get_day_num(&caps[3]);
        if d > 12 || d <= m {
            res = format!("{}-{}-{:02}", y, get_month_name(m), d);
            trans = "ymd"
        } else {
            res = format!(
                "{}-{}-{:02} $$$ {}-{}..{} ",
                y,
                get_month_name(m),
                d,
                y,
                get_month_name(m),
                get_month_name(d)
            );
            trans = "ymd$"
        }
    } else if let Some(caps) = dp.re_date_ymm.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y = get_year_num(&caps[1]);
        let m1 = get_month_num(&caps[2]);
        let m2 = get_month_num(&caps[3]);
        res = format!("{}-{}..{}", y, get_month_name(m1), get_month_name(m2));
        trans = "ymm"
    } else if let Some(caps) = dp.re_date_ym.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y = get_year_num(&caps[1]);
        let m = get_month_num(&caps[2]);
        res = format!("{}-{}", y, get_month_name(m));
        trans = "ym"
    } else if let Some(caps) = dp.re_date_ynm.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y = get_year_num(&caps[1]);
        let n = &caps[2];
        let m = get_month_num(&caps[3]);
        res = format!("n°{} {}-{}", n, y, get_month_name(m));
        trans = "ynm"
    }

    if !res.is_empty() {
        stem = format!("{}{}{}", &stem[..start], res, &stem[start + len..]);
    }

    if !trans.is_empty() {
        println!("{:70} {:9} «{}»", stem_original.nfc().collect::<String>(), trans, stem);
    } else {
        println!("{:70} {:9}", stem_original.nfc().collect::<String>(), "???");
    }

    stem
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
