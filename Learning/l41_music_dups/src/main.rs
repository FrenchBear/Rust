// l41_music_dups: Detects dups or close dups for music files
//
// 2025-04-11	PV      First version

//#![allow(unused)]

// standard library imports
use std::path::PathBuf;
use std::process;
use std::sync::LazyLock;
use std::time::Instant;
use std::{collections::HashMap, fmt::Debug};

// external crates imports
use myglob::{MyGlobMatch, MyGlobSearch};
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

// -----------------------------------
// Submodules

mod logging;
use logging::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = "music_dups";
const APP_VERSION: &str = "1.0.0";

// -----------------------------------
// Main

#[derive(Debug, Default)]
struct DataBag {
    files_count: usize,
    errors_count: usize,
    files: Vec<MusicFile>,
}

fn main() {
    let mut globstrsources: Vec<String> = Vec::new();
    globstrsources.push(r"C:\MusicOD\MP3P\**\*.mp3".to_string());

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
        logln(&mut writer, format!("*** No source to process, aborting.").as_str());
        process::exit(1);
    }

    log(&mut writer, "\nSources(s): ");
    for source in sources.iter() {
        logln(&mut writer, format!("- {}", source.0).as_str());
    }

    // First collect information on files in DataBag
    for gs in sources.iter() {
        for ma in gs.1.explore_iter() {
            match ma {
                MyGlobMatch::File(pb) => process_file(&mut writer, &mut b, pb),

                MyGlobMatch::Dir(_) => {}

                MyGlobMatch::Error(err) => {
                    logln(&mut writer, format!("{APP_NAME}: MyGlobMatch error {}", err).as_str());
                }
            }
        }
    }

    if b.files.is_empty() {
        logln(&mut writer, "*** No music file found, nothing to report.");
    } else {
        logln(&mut writer, format!("{} music file(s) found, consolidating data", b.files.len()).as_str());

        fn getter(mf: &MusicFile) -> &str {
            &mf.segments[0]
        }
        let data_name = "segments[0]";

        // Counters
        //let mut counter = HashMap::<&str, u32>::new();
        let mut counter_ics = HashMap::<String, (u32, HashMap<&str, u32>)>::new(); // Ignore case and spaces
        for mf in b.files.iter() {
            let data = getter(mf);
            //*counter.entry(data).or_insert(0) += 1;

            let data_ics = filter_alphanum(data);
            let entry_ics = counter_ics.entry(data_ics).or_insert((0, HashMap::<&str, u32>::new()));
            (*entry_ics).0 += 1;
            let subentry_ics = entry_ics.1.entry(data).or_insert(0);
            *subentry_ics += 1;
        }

        //     // Sort and print direct counter
        //     // logln(&mut writer, "\n{data_name}: Simple groups, at least 2 values");
        //     // let mut vec: Vec<(&&str, &u32)> = counter.iter().collect();
        //     // vec.sort_by(|&a, &b| b.1.cmp(a.1));
        //     // for (key, value) in vec.into_iter().take_while(|&x| *(x.1) > 1) {
        //     //     let skey = if key.is_empty() { "(empty)" } else { *key };
        //     //     logln(&mut writer, format!("{}: {}", skey, value).as_str());
        //     // }

        // Sort and print case-insensitive space-insensitive direct counter
        logln(&mut writer, format!("\n{data_name}: Groups ignoring case and spaces, at least 2 files").as_str());
        let mut vec_ics: Vec<(&String, &(u32, HashMap<&str, u32>))> = counter_ics.iter().collect();
        vec_ics.sort_by(|&a, &b| (b.1.0).cmp(&a.1.0));
        let mut vec_repr = Vec::<(&str, &str)>::new(); // Collect representants for Levenshtein distance
        for (key, value) in vec_ics.into_iter()
        /* .take_while(|&x| *(&x.1.0) > 1) */
        {
            // Now sort subvector
            let mut subvec: Vec<(&&str, &u32)> = value.1.iter().collect();
            if subvec.len() == 1 {
                // Single form class
                let (ukey, uvalue) = *subvec.first().unwrap();
                let sukey = if ukey.is_empty() { "(empty)" } else { *ukey };
                vec_repr.push((key, sukey));
                if value.0 > 1 {
                    logln(&mut writer, format!("{}: {}", sukey, uvalue).as_str());
                }
            } else {
                // Representant of class is the most encountered element
                subvec.sort_by(|a, b| b.1.cmp(a.1));
                let repr = subvec.first().unwrap();
                let (rkey, _) = *repr;
                let srkey = if rkey.is_empty() { "(empty)" } else { *rkey };
                vec_repr.push((key, srkey));

                if value.0 > 1 {
                    // Print representant and total count
                    log(&mut writer, format!("{}: {}\t", srkey, value.0).as_str());
                    // Print all variants and individual count
                    for (vkey, vvalue) in subvec.iter() {
                        let svkey = if vkey.is_empty() { "(empty)" } else { *vkey };
                        log(&mut writer, format!("{}: {}\t", svkey, vvalue).as_str());
                    }
                    logln(&mut writer, "");
                }
            }
        }

        // Find close representants
        logln(
            &mut writer,
            format!("\n{data_name}: Possible confusions, Levenshtein distance=1").as_str(),
        );
        for i in 0..vec_repr.len() {
            let (cnorm1, crepr1) = vec_repr[i];
            for j in i + 1..vec_repr.len() {
                let (cnorm2, crepr2) = vec_repr[j];
                let d = levenshtein_distance(cnorm1, cnorm2, 1);
                if d == 1 {
                    // Found a close pair, print it
                    let s1 = if crepr1.is_empty() { "(empty)" } else { crepr1 };
                    let s2 = if crepr2.is_empty() { "(empty)" } else { crepr2 };
                    logln(&mut writer, format!("{} <-> {}", s1, s2).as_str());
                }
            }
        }
    }

    let duration = start.elapsed();
    logln(&mut writer, "");
    log(&mut writer, format!("{} files(s)", b.files_count).as_str());
    log(&mut writer, format!(", {} error(s)", b.errors_count).as_str());
    logln(&mut writer, format!(" found in {:.3}s", duration.as_secs_f64()).as_str());
}

// Only keep letters and digits, converted to ASCII lowercase, doubling digits
// This is used to derive canonical representation of segments to detect variations
// Doubling digits avoids a Levenshtein distance of 1 when strigs differ only by 1 digit such as xxx vol.1 xxx and xxx vol.2 xxx
fn filter_alphanum(s: &str) -> String {
    static DIGIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)").unwrap());
    let t = s
        .chars()
        .nfd()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>();
    // Use regex to double digits by plain lazyness...
    DIGIT.replace_all(t.as_str(), "$1$1").to_string()
}

fn process_file(writer: &mut LogWriter, b: &mut DataBag, pb: PathBuf) {
    b.files_count += 1;
    let music_file = break_music_file(pb);
    match music_file {
        Ok(mf) => b.files.push(mf),
        Err(e) => logln(writer, format!("*** {}", e).as_str()),
    }
}

#[derive(Debug)]
#[allow(unused)]
struct MusicFile {
    pb: PathBuf,
    stem: String,
    segments: Vec<String>,
}

fn break_music_file(pb: PathBuf) -> Result<MusicFile, String> {
    let file = pb.file_name().unwrap().to_str().unwrap();
    let stem = pb.file_stem().unwrap().to_str().unwrap();

    if !is_balanced(stem) {
        return Err(format!("Err: Unbalanced brackets: {}", file));
    }

    Ok(MusicFile {
        pb: pb.clone(),
        stem: stem.to_string(),
        segments: stem.split(" - ").map(|s| s.to_string()).collect::<Vec<String>>(),
    })
}

/// Checks that () [] {} «» ‹› pairs are correctly embedded and closed in a string
pub fn is_balanced(s: &str) -> bool {
    // Unit tests in rcheckfiles
    let mut stack = Vec::<char>::new();
    let mut current_state = ' ';

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '«' | '‹' => {
                stack.push(current_state);
                current_state = c;
            }
            ')' | ']' | '}' | '»' | '›' => {
                if stack.len() == 0 {
                    return false;
                }

                let opener = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '»' => '«',
                    '›' => '‹',
                    _ => unreachable!(),
                };
                if current_state == opener {
                    current_state = stack.pop().unwrap();
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    current_state == ' '
}

/// Computes Levenshtein distance between two strings with early exit if the distance exceeds dmax.
pub fn levenshtein_distance(s1: &str, s2: &str, dmax: usize) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    // If the difference in lengths exceeds dmax, return early
    if (len1 as isize - len2 as isize).abs() as usize > dmax {
        return dmax + 1;
    }

    let mut prev_row: Vec<usize> = (0..=len2).collect();
    let mut curr_row = vec![0; len2 + 1];

    for (i, c1) in s1.chars().enumerate() {
        curr_row[0] = i + 1;

        let mut min_in_row = curr_row[0]; // Track the minimum value in the current row
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            curr_row[j + 1] = (prev_row[j + 1] + 1).min(curr_row[j] + 1).min(prev_row[j] + cost);

            min_in_row = min_in_row.min(curr_row[j + 1]);
        }

        // Early exit if the minimum value in the row exceeds dmax
        if min_in_row > dmax {
            return dmax + 1;
        }

        prev_row.copy_from_slice(&curr_row);
    }

    let distance = curr_row[len2];
    if distance > dmax { dmax + 1 } else { distance }
}
