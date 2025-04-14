// rnormalizedates: Rust version of NormalizeDates, Normalizes dates in filenames, replace 'January 2020' by '2020-01'
//
// 2025-04-12	PV      First version

#![allow(unused)]

// standard library imports
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf, Prefix};
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
#[allow(unused)]
fn main() {
    let dp = DatePatterns::new();
    process_file(&PathBuf::from("Cerveau___Psycho-Novembre_2022.pdf"), &dp, false, true);
    // for filefp in get_dev_data() {
    //     process_file(&PathBuf::from(filefp), &dp, false, false);
    // }
}

fn mmain() {
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
                            process_file(&pb, &date_patterns, !options.no_action, options.verbose);
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

    if options.final_pause {
        print!("\n(pause) ");
        io::stdout().flush().unwrap();
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).unwrap(); // Wait for a single byte (key press)
    }
}

struct DatePatterns {
    re_date_ymd_head: Regex,
    re_date_ynm: Regex,
    re_no: Regex,

    re_date_ymd_std: Regex,
    re_date_ymm_std: Regex,

    re_date_mymy: Regex,
    re_date_ymym: Regex,

    re_date_mmmy: Regex,
    re_date_ymmm: Regex,

    re_date_mymmy: Regex,
    re_date_ymymm: Regex,

    re_date_mmymy: Regex,
    re_date_ymmym: Regex,

    re_date_mmy: Regex,
    re_date_ymm: Regex,

    re_date_dmdmy: Regex,

    re_date_dmy: Regex,
    re_date_ymd: Regex,

    re_date_my: Regex,
    re_date_ym: Regex,
}

impl DatePatterns {
    fn new() -> Self {
        // Prepare regex
        // Note: \b est une ancre de limite de mot (mais backspace dans une [classe])
        let mut month = String::new();
        let mut months_sorted = MONTHS.clone();
        months_sorted.sort_by_key(|k| -(k.0.len() as i32));
        for &(month_name, _) in months_sorted.iter() {
            month.push_str(if month.is_empty() { r"\b(" } else { "|" });
            month.push_str(month_name);
        }
        month.push_str(r")\b");
        let month = month.as_str();
        // Years from 1920
        let year = r"\b((?:19[2-9][0-9]|20[0-2][0-9])|(?:2[0-9]))(?:B?)\b"; // New version, 2020 and more only, absorb a B following a year
        let day = r"\b(1er|30|31|(?:0?[1-9])|[12][0-9])\b";

        // Dates already valid, to ignore
        let re_date_ymd_std = Regex::new((String::from("(?i)") + year + "-" + month + "-" + day + r"(\.\." + day + ")?").as_str()).unwrap();
        let re_date_ymm_std = Regex::new((String::from("(?i)") + year + "-" + month + r"(\.\." + month + ")?").as_str()).unwrap();

        // Special patterns
        let re_date_ymd_head = Regex::new((String::from("(?i)") + "^ " + year + "[ -]" + "(0[1-9]|10|11|12)" + "[ -]" + day + " ").as_str()).unwrap();
        let re_date_ynm = Regex::new((String::from("(?i)") + year + r" +(Volume \d+ +)?№(\d+) +" + month).as_str()).unwrap();
        let re_no = Regex::new(r"(?i)( n°? *\d+)").unwrap();

        let re_date_mymy = Regex::new((String::from("(?i)") + month + "[ -]+" + year + "[ à-]+" + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ymym = Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ -]+" + year + "[ -]+" + month).as_str()).unwrap();

        let re_date_mmmy = Regex::new((String::from("(?i)") + month + "-" + month + "-" + month + " +" + year).as_str()).unwrap();
        let re_date_ymmm = Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ -]+" + month + "[ -]+" + month).as_str()).unwrap();

        let re_date_mymmy =
            Regex::new((String::from("(?i)") + month + "[ -]+" + year + "[ -]+" + month + "[ -]+" + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ymymm =
            Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ -]+" + year + "[ -]+" + month + "[ -]+" + month).as_str()).unwrap();

        let re_date_mmymy =
            Regex::new((String::from("(?i)") + month + "-" + month + "[ -]+" + year + "[ -]+" + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ymmym =
            Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ -]+" + month + "[ -]+" + year + "[ -]+" + month).as_str()).unwrap();

        let re_date_mmy = Regex::new((String::from("(?i)") + month + "[ à-]+" + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ymm = Regex::new((String::from("(?i)") + year + "[ -]+" + month + "[ à-]+" + month).as_str()).unwrap();

        let re_date_dmdmy =
            Regex::new((String::from("(?i)") + day + " +" + month + " *(?:au)? *" + day + " +" + month + " +" + year).as_str()).unwrap();

        let re_date_dmy = Regex::new((String::from("(?i)") + day + " +" + month + " +" + year).as_str()).unwrap();
        let re_date_ymd = Regex::new((String::from("(?i)") + year + " +" + month + " +" + day).as_str()).unwrap();

        let re_date_my = Regex::new((String::from("(?i)") + month + "[ -]+" + year).as_str()).unwrap();
        let re_date_ym = Regex::new((String::from("(?i)") + year + "[ -]+" + month).as_str()).unwrap();
        // let re_date_my = Regex::new((String::from("(?i)") + r"(\d[ -])?" + month + "[ -]+" + year + r"([ -]\d)?").as_str()).unwrap();
        // let re_date_ym = Regex::new((String::from("(?i)") + r"(\d[ -])?" + year + "[ -]+" + month + r"([ -]\d)?").as_str()).unwrap();

        // If name starts with a Ymd date, then move it to the end, and analyse remaining patterns

        DatePatterns {
            re_date_ymd_head,
            re_date_ynm,
            re_no,

            re_date_ymm_std,
            re_date_ymd_std,

            re_date_mymy,
            re_date_ymym,

            re_date_mmmy,
            re_date_ymmm,

            re_date_mymmy,
            re_date_ymymm,

            re_date_mmymy,
            re_date_ymmym,

            re_date_mmy,
            re_date_ymm,

            re_date_dmdmy,

            re_date_dmy,
            re_date_ymd,

            re_date_my,
            re_date_ym,
        }
    }
}

fn process_file(pb: &Path, dp: &DatePatterns, do_it: bool, verbose: bool) {
    //println!("Processing {}", pb.display());
    let filename_original = pb.file_name().unwrap().to_string_lossy().into_owned();
    let stem_original = pb.file_stem().expect("No stem??").to_string_lossy().into_owned();
    let extension = pb.extension().unwrap().to_string_lossy().to_lowercase();

    let mut stem = apply_initial_transformations(&stem_original);
    stem = apply_date_transformations(&stem, dp, verbose);
    stem = apply_final_transformations(&stem) + "." + extension.as_str();

    if filename_original != stem {
        println!("{:70} {}", filename_original.nfc().collect::<String>(), stem);

        if do_it {
            let mut newpb = pb.parent().unwrap().to_path_buf().join(PathBuf::from(stem));
            if let Err(e) = fs::rename(pb, &newpb) {
                eprintln!("*** Error nenaming \"{}\" to \"{}\":\n{}", pb.display(), newpb.display(), e);
            }
        }
    } else {
        println!("{}", filename_original);
    }
}

fn apply_initial_transformations(stem_original: &str) -> String {
    let mut stem: String = stem_original.nfc().collect();
    stem = stem.replace('_', " ");
    stem = stem.replace('’', "'");
    stem = stem.replace("..", "£"); // Keep double dots
    stem = stem.replace(".", " "); // But replace simple dots by spaces
    stem = stem.replace("£", "..");
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
        if icontains(&stem, " fr ") {
            stem = ireplace(&stem, " fr ", " ");
            update = true;
        }
        if icontains(&stem, " francais ") {
            stem = ireplace(&stem, " francais ", " ");
            update = true;
        }
        if stem.contains(" N°") {
            stem = stem.replace(" N°", " n°");
            update = true;
        }

        if !update {
            break;
        }
    }

    stem
}

fn apply_date_transformations(stem_original: &str, dp: &DatePatterns, verbose: bool) -> String {
    let mut stem = stem_original.to_string();

    let mut start = 0;
    let mut len = 0;
    let mut res = String::new();
    let mut trans: &'static str = "";

    // Protect n° so it won't interfere with date processing
    if let Some(caps) = dp.re_no.captures(&stem) {
        let nstart = caps.get(0).unwrap().start();
        let nlen = caps.get(0).unwrap().len();
        stem = format!("{}‹{}›{}", &stem[..nstart], &caps[0], &stem[nstart + nlen..]);
    }

    // If name starts with a Ymd date, then move it to the end, and analyse remaining patterns
    if let Some(caps) = dp.re_date_ymd_head.captures(&stem) {
        let cf = &caps[0];
        let y = get_year_num(&caps[1]);
        let m = get_month_num(&caps[2]);
        let d = get_day_num(&caps[3]);
        // Special case, generate directly new version of stem without res intermediate
        stem = format!(" {}- {}-{}-{:02} ", &stem[cf.len()..], y, get_month_name(m), d);
        trans = "ymd_head";
    } else if let Some(caps) = dp.re_date_ymd_std.captures(&stem) {
        // Already standard date
    } else if let Some(caps) = dp.re_date_ymm_std.captures(&stem) {
        // Already standard date
    } else if let Some(caps) = dp.re_date_ynm.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y = get_year_num(&caps[1]);
        let vol = if let Some(ma) = caps.get(2) { ma.as_str() } else { "" };
        let n = &caps[3];
        let m = get_month_num(&caps[4]);
        res = format!("{}n°{} - {}-{}", vol, n, y, get_month_name(m));
        trans = "ynm"
    } else if let Some(caps) = dp.re_date_mmmy.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let m1 = get_month_num(&caps[1]);
        let m3 = get_month_num(&caps[3]);
        let y = get_year_num(&caps[4]);
        res = format!("{}-{}..{}", y, get_month_name(m1), get_month_name(m3));
        trans = "mmmy"
    } else if let Some(caps) = dp.re_date_ymmm.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y = get_year_num(&caps[1]);
        let m1 = get_month_num(&caps[2]);
        let m3 = get_month_num(&caps[4]);
        res = format!("{}-{}..{}", y, get_month_name(m1), get_month_name(m3));
        trans = "ymmm"
    } else if let Some(caps) = dp.re_date_mymmy.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let m1 = get_month_num(&caps[1]);
        let y1 = get_year_num(&caps[2]);
        let m3 = get_month_num(&caps[4]);
        let y2 = get_year_num(&caps[5]);
        res = format!("{}-{}..{}-{}", y1, get_month_name(m1), y2, get_month_name(m3));
        trans = "mymmy"
    } else if let Some(caps) = dp.re_date_ymymm.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y1 = get_year_num(&caps[1]);
        let m1 = get_month_num(&caps[2]);
        let y2 = get_year_num(&caps[3]);
        let m3 = get_month_num(&caps[5]);
        res = format!("{}-{}..{}-{}", y1, get_month_name(m1), y2, get_month_name(m3));
        trans = "ymymm"
    } else if let Some(caps) = dp.re_date_ymmym.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y1 = get_year_num(&caps[1]);
        let m1 = get_month_num(&caps[2]);
        let y2 = get_year_num(&caps[4]);
        let m3 = get_month_num(&caps[5]);
        res = format!("{}-{}..{}-{}", y1, get_month_name(m1), y2, get_month_name(m3));
        trans = "ymmym"
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
    } else if let Some(caps) = dp.re_date_ymym.captures(&stem) {
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y1 = get_year_num(&caps[1]);
        let m1 = get_month_num(&caps[2]);
        let y2 = get_year_num(&caps[3]);
        let m2 = get_month_num(&caps[4]);
        res = format!("{}-{}..{}-{}", y1, get_month_name(m1), y2, get_month_name(m2));
        trans = "ymym"
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
        // if caps.get(1).is_none() && caps.get(4).is_none() {
        //     // first/last capture should be a negative look ahead assertion, but it's not supported by regex crate...
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let m = get_month_num(&caps[1]);
        let y = get_year_num(&caps[2]);
        res = format!("{}-{}", y, get_month_name(m));
        trans = "my"
        // }
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
        // if caps.get(1).is_none() && caps.get(4).is_none() {
        //     // first/last capture should be a negative look ahead assertion, but it's not supported by regex crate...
        start = caps.get(0).unwrap().start();
        len = caps.get(0).unwrap().len();
        let y = get_year_num(&caps[1]);
        let m = get_month_num(&caps[2]);
        res = format!("{}-{}", y, get_month_name(m));
        trans = "ym"
        // }
    }

    if !res.is_empty() {
        let p = if res.starts_with("n°") {""} else {"- "};
        stem = format!("{} {p}{} - {}", &stem[..start], res, &stem[start + len..]);
    }

    if verbose {
        if !trans.is_empty() {
            println!("{:70} {:9} «{}»", stem_original.nfc().collect::<String>(), trans, stem);
        } else {
            println!("{:70} {:9}", stem_original.nfc().collect::<String>(), "???");
        }
    }

    stem
}

fn apply_final_transformations(stem_original: &str) -> String {
    let mut stem = stem_original.to_string();

    if !stem.contains("du pirate") {
        stem = ireplace(&stem, " du ", " - ");
    }

    loop {
        let mut update = false;

        if stem.contains("  ") {
            stem = stem.replace("  ", " ");
            update = true;
        }
        if stem.contains("- -") {
            stem = stem.replace("- -", " - ");
            update = true;
        }
        if stem.contains("--") {
            stem = stem.replace("--", " - ");
            update = true;
        }
        if stem.contains("(-") {
            stem = stem.replace("(-", "(");
            update = true;
        }
        if stem.contains("-)") {
            stem = stem.replace("-)", ")");
            update = true;
        }
        if stem.contains("( ") {
            stem = stem.replace("( ", "(");
            update = true;
        }
        if stem.contains(" )") {
            stem = stem.replace(" )", ")");
            update = true;
        }
        if stem.contains("‹") {
            stem = stem.replace('‹', "");
            update = true;
        }
        if stem.contains("›") {
            stem = stem.replace('›', "");
            update = true;
        }
        if stem.starts_with('-') {
            stem = (&stem[1..]).into();
            update = true;
        }
        if stem.ends_with("- ") {
            stem = (&stem[..stem.len() - 2]).into();
            update = true;
        }
        if stem.ends_with('-') {
            stem = (&stem[..stem.len() - 1]).into();
            update = true;
        }

        if !update {
            break;
        }
    }

    stem = ireplace(&stem, "Hors-Série", "HS");
    stem = ireplace(&stem, "Hors-S rie", "HS");
    stem = ireplace(&stem, "01net", "01net");
    stem = ireplace(&stem, "4x4 Magazine France", "4x4 Magazine");
    stem = ireplace(&stem, "60 Millions de Consommateurs", "60M de consommateurs");
    stem = ireplace(&stem, "Ça M'Intéresse", "Ça m'intéresse");
    stem = ireplace(&stem, "a M Int resse", "Ça m'intéresse");
    stem = ireplace(&stem, "a M Int resse Questions R ponses", "Ça m'intéresse Questions Réponses");
    stem = ireplace(&stem, "Ça m'intéresse Questions R ponses", "Ça m'intéresse Questions Réponses");
    stem = ireplace(&stem, "Questions & Réponses", "Questions Réponses");
    stem = ireplace(&stem, "Auto Moto France", "Auto Moto");
    stem = ireplace(&stem, "Auto Plus - Guide de L'Acheteur", "Auto Plus Guide de l'acheteur");
    stem = ireplace(&stem, "Auto Plus - HS - Crossovers-Suv", "Auto Plus Crossovers");
    stem = ireplace(&stem, "Auto Plus Hors-S rie Crossovers Suv", "Auto Plus Crossovers");
    stem = ireplace(&stem, "Cerveau & Psycho", "Cerveau & Psycho");
    stem = ireplace(&stem, "Cerveau Psycho", "Cerveau & Psycho");
    stem = ireplace(&stem, "Comp tence Mac", "Compétence Mac");
    stem = ireplace(&stem, "Belle Magazine", "Belle");
    stem = ireplace(&stem, "Echappee", "Échappée");
    stem = ireplace(&stem, "Echappée", "Échappée");
    stem = ireplace(&stem, "Elektor France", "Elektor");
    stem = ireplace(&stem, "Geo France", "Géo");
    stem = ireplace(&stem, " Geo ", " Géo ");
    stem = ireplace(&stem, "Historia", "Historia");
    stem = ireplace(&stem, "Histoire Civilisations", "Histoire & Civilisations");
    stem = ireplace(&stem, "L'Auto Journal", "L'Auto-Journal");
    stem = ireplace(&stem, "L Auto-Journal", "L'Auto-Journal");
    stem = ireplace(&stem, "L Automobile Magazine", "L'Automobile Magazine");
    stem = ireplace(&stem, "L'AUTO JOURNAL Le Guide", "Le guide de l'Auto-Journal");
    stem = ireplace(&stem, "L'Auto-Journal Le Guide", "Le guide de l'Auto-Journal");
    stem = ireplace(&stem, "L'Auto-Journal - Le guide", "Le guide de l'Auto-Journal");
    stem = ireplace(&stem, "L'essentiel de l'Auto", "L'essentiel de l'Auto");
    stem = ireplace(&stem, " enchaîné ", " enchainé ");
    stem = ireplace(&stem, "Le Canard", "Le canard");
    stem = ireplace(&stem, "Le Figaro Histoire", "Le Figaro Histoire");
    stem = ireplace(&stem, "Le Monde - Histoire & Civilisations", "Histoire & Civilisations");
    stem = ireplace(&stem, "Le Monde Histoire Civilisations", "Histoire & Civilisations");
    stem = ireplace(&stem, "Le Monde Histoire & Civilisations", "Histoire & Civilisations");
    stem = ireplace(&stem, "science et vie", "Science & Vie");
    stem = ireplace(&stem, "Les Collections de L'Histoire", "Les collections de L'Histoire");
    stem = ireplace(&stem, "Magazine CERVEAU et PSYCHO", "Cerveau & Psycho");
    stem = ireplace(&stem, "Merci pour l'info", "Merci pour l'info");
    stem = ireplace(&stem, " N ", " n°");
    stem = ireplace(&stem, "QC pratique", "Que Choisir Pratique");
    stem = ireplace(&stem, "Que choisir - HS Budgets", "Que choisir Budgets");
    stem = ireplace(&stem, "Que Choisir Hors-Série Budgets", "Que choisir Budgets");
    stem = ireplace(&stem, "Que Choisir Sante", "Que Choisir Santé");
    stem = ireplace(&stem, "Que Choisir Sant ", "Que Choisir Santé ");
    stem = ireplace(&stem, "Science & Vie - Guerres & Histoire", "Science & Vie Guerres & Histoire");
    stem = ireplace(&stem, "Science Vie Guerres Histoire", "Science & Vie Guerres & Histoire");
    stem = ireplace(&stem, "Secrets d Histoire", "Secrets d'Histoire");
    stem = ireplace(&stem, "Super picsou geant", "Super Picsou Géant");
    stem = ireplace(&stem, "T3 France", "T3");
    stem = ireplace(&stem, "Terre Sauvage", "Terre Sauvage");
    stem = ireplace(&stem, "What Hi-Fi France", "What Hi-Fi");
    stem = ireplace(&stem, "Windows Internet Pratique", "Windows & Internet Pratique");
    stem = ireplace(&stem, "01net", "01net");
    stem = ireplace(&stem, "01net", "01net");
    stem = ireplace(&stem, "01net", "01net");

    while stem.contains("  ") {
        stem = stem.replace("  ", " ")
    }
    stem = stem.trim().to_string();

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
