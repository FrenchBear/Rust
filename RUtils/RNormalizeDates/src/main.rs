// rnormalizedates: Rust version of NormalizeDates, Normalizes dates in filenames, replace 'January 2020' by '2020-01'
//
// 2025-04-12	PV      First version
// 2025-04-16   PV      Better normalization of n°
// 2025-04-16   PV      1.1 Final counts, DataBag, Implementation of option -S, improved filtering of HS
// 2025-04-18   PV      1.1.1 Better normalization of Ça m'intéresse

//#![allow(unused)]

// standard library imports
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

// external crates imports
use getopt::Opt;
use myglob::{MyGlobMatch, MyGlobSearch};
use terminal_size::{Width, terminal_size};
use unicode_normalization::UnicodeNormalization;

// -----------------------------------
// Submodules

mod logging;
mod options;
mod re;
mod tests;

use logging::*;
use options::*;
use re::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = "rnormalizedates";
const APP_VERSION: &str = "1.1.1";

// -----------------------------------
// Main

// Dev tests
// #[allow(unused)]
// fn test_main() {
//     let dp = DatePatterns::new();
//     let mut writer = logging::new(true);
//     let options = Options {..Default::default()};
//     let mut b = DataBag {..Default::default()};

//     process_file(&mut writer, &PathBuf::from("Destination France - 03.04.05.2025.pdf"), &dp, &options, &mut b);
// }

#[derive(Debug, Default)]
struct DataBag {
    files_count: usize,
    files_renamed_count: usize,
    errors_count: usize,
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

    // Prepare log writer
    let mut writer = logging::new(true);
    let mut b = DataBag { ..Default::default() };
    let start = Instant::now();

    let date_patterns = DatePatterns::new();
    for source in options.sources.iter() {
        let resgs = MyGlobSearch::new(source).autorecurse(true).compile();
        match resgs {
            Ok(gs) => {
                logln(&mut writer, format!("Processing {}\n", source).as_str());
                for ma in gs.explore_iter() {
                    match ma {
                        MyGlobMatch::File(pb) => {
                            process_file(&mut writer, &pb, &date_patterns, &options, &mut b);
                        }

                        // We ignore matching directories in rgrep, we only look for files
                        MyGlobMatch::Dir(_) => {}

                        MyGlobMatch::Error(_) => {}
                    }
                }
                logln(&mut writer, "");
            }

            Err(e) => {
                logln(&mut writer, format!("*** Error building MyGlob: {:?}", e).as_str());
            }
        }
    }

    let duration = start.elapsed();
    log(&mut writer, format!("{} file(s) found", b.files_count).as_str());
    if b.files_renamed_count > 0 {
        log(&mut writer, format!(", {} renamed", b.files_renamed_count).as_str());
        if options.no_action {
            log(&mut writer, " (no action)");
        }
    }
    if b.errors_count > 0 {
        log(&mut writer, format!(", {} error(s)", b.errors_count).as_str());
    }
    logln(&mut writer, "");

    logln(&mut writer, format!("Duration: {:.3}s", duration.as_secs_f64()).as_str());

    if options.final_pause {
        print!("\n(pause) ");
        io::stdout().flush().unwrap();
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).unwrap(); // Wait for a single byte (key press)
    }
}

fn process_file(lw: &mut LogWriter, pb: &Path, dp: &DatePatterns, opt: &Options, b: &mut DataBag) {
    b.files_count += 1;

    let filename_original = pb.file_name().unwrap().to_string_lossy().into_owned();
    let stem_original = pb.file_stem().expect("No stem??").to_string_lossy().into_owned();
    let extension = pb.extension().unwrap().to_string_lossy().to_lowercase();

    let filename_new = if opt.segment > 0 {
        let mut ts = stem_original.split(" - ").collect::<Vec<&str>>();
        if opt.segment > ts.len() {
            filename_original.clone()
        } else {
            let mut seg = apply_initial_transformations(ts[opt.segment - 1]);
            seg = apply_date_transformations(&seg, dp, opt.verbose, true);
            seg = apply_final_transformations(&seg);
            println!("Final: «{}»", seg);
            ts[opt.segment - 1] = seg.as_str();

            ts.join(" - ") + "." + extension.as_str()
        }
    } else {
        let mut stem = apply_initial_transformations(&stem_original);
        stem = apply_date_transformations(&stem, dp, opt.verbose, false);
        stem = apply_final_transformations(&stem) + "." + extension.as_str();

        stem
    };

    if filename_original != filename_new {
        logln(
            lw,
            format!("{:70} {}", filename_original.nfc().collect::<String>(), filename_new).as_str(),
        );
        b.files_renamed_count += 1;

        if !opt.no_action {
            let newpb = pb.parent().unwrap().to_path_buf().join(PathBuf::from(filename_new));
            if let Err(e) = fs::rename(pb, &newpb) {
                logln(
                    lw,
                    format!("*** Error nenaming \"{}\" to \"{}\":\n{}", pb.display(), newpb.display(), e).as_str(),
                );
                b.files_renamed_count -= 1;
                b.errors_count += 1;
            }
        }
    } else {
        logln(lw, filename_original.as_str());
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

    // Add starting/ending space to simplify some detections
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
        // if stem.contains(" N°") {
        //     stem = stem.replace(" N°", " n°");
        //     update = true;
        // }

        if !update {
            break;
        }
    }

    stem
}

fn apply_date_transformations(stem_original: &str, dp: &DatePatterns, verbose: bool, segment_mode: bool) -> String {
    let mut stem = stem_original.to_string();

    let mut start = 0;
    let mut len = 0;
    let mut res = String::new();
    let mut trans: &'static str = "";

    // Protect n° so it won't interfere with date processing
    // Note that US/UK № is not protected
    if let Some(caps) = dp.re_no.captures(&stem) {
        let nstart = caps.get(0).unwrap().start();
        let nlen = caps.get(0).unwrap().len();
        stem = format!("{}‹ n°{}›{}", &stem[..nstart], &caps[1], &stem[nstart + nlen..]);
    }

    // If name starts with a ymd date, then move it to the end, and analyze remaining patterns
    if let (false, Some(caps)) = (segment_mode, dp.re_date_ymd_head.captures(&stem)) {
        let cf = &caps[0];
        let y = get_year_num(&caps[1]);
        let m = get_month_num(&caps[2]);
        let d = get_day_num(&caps[3]);
        // Special case, generate directly new version of stem without res intermediate
        stem = format!(" {}- {}-{}-{:02} ", &stem[cf.len()..], y, get_month_name(m), d);
        trans = "ymd_head";
    } else if dp.re_date_ymd_std.captures(&stem).is_some() || dp.re_date_ymm_std.captures(&stem).is_some() {
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
        if segment_mode {
            // In segment mode, we don't add - around dates, just do the conversion
            stem = format!("{} {} {}", &stem[..start], res, &stem[start + len..])
        } else {
            let p = if res.starts_with("n°") { "" } else { "- " };
            stem = format!("{} {p}{} - {}", &stem[..start], res, &stem[start + len..]);
        }
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

    if !icontains(&stem, "du pirate") {
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
        if stem.starts_with(" -") {
            stem = (&stem[2..]).into();
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

    stem = ireplace(&stem, " - n°", " n°");
    stem = ireplace(&stem, "Hors-Série", "HS");
    stem = ireplace(&stem, "Hors-S rie", "HS");
    stem = ireplace(&stem, " - HS", " HS");
    stem = ireplace(&stem, " HS n°", " HS ");
    stem = ireplace(&stem, "01net", "01net");
    stem = ireplace(&stem, "4x4 Magazine France", "4x4 Magazine");
    stem = ireplace(&stem, "60 Millions de Consommateurs", "60M de consommateurs");
    stem = ireplace(&stem, "Ca MInteresse", "Ça m'intéresse");
    stem = ireplace(&stem, "Ca m'intéresse", "Ça m'intéresse");
    stem = ireplace(&stem, "Ça M'Intéresse", "Ça m'intéresse");
    stem = ireplace(&stem, "a M Int resse", "Ça m'intéresse");
    stem = ireplace(&stem, "a M Int resse Questions R ponses", "Ça m'intéresse Questions Réponses");
    stem = ireplace(&stem, "Ça m'intéresse Questions R ponses", "Ça m'intéresse Questions Réponses");
    stem = ireplace(&stem, "Questions & Réponses", "Questions Réponses");
    stem = ireplace(&stem, "Auto Moto France", "Auto Moto");
    stem = ireplace(&stem, "Auto Plus - Guide de L'Acheteur", "Auto Plus Guide de l'acheteur");
    stem = ireplace(&stem, "Auto Plus HS - Crossovers-Suv", "Auto Plus Crossovers");
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
    stem = ireplace(&stem, "Les cahiers de Science & Vie", "Les cahiers de Science & Vie");
    stem = ireplace(&stem, "Les cahiers de Science Vie", "Les cahiers de Science & Vie");
    stem = ireplace(&stem, "science et vie", "Science & Vie");
    stem = ireplace(&stem, "Les Collections de L'Histoire", "Les collections de L'Histoire");
    stem = ireplace(&stem, "Magazine CERVEAU et PSYCHO", "Cerveau & Psycho");
    stem = ireplace(&stem, "Merci pour l'info", "Merci pour l'info");
    stem = ireplace(&stem, " N ", " n°");
    stem = ireplace(&stem, "QC pratique", "Que Choisir Pratique");
    stem = ireplace(&stem, "Que choisir", "Que Choisir");
    stem = ireplace(&stem, "Que choisir HS Budgets", "Que Choisir Budgets");
    stem = ireplace(&stem, "Que Choisir Hors-Série Budgets", "Que Choisir Budgets");
    stem = ireplace(&stem, "Que Choisir Sante", "Que Choisir Santé");
    stem = ireplace(&stem, "Que Choisir Sant ", "Que Choisir Santé ");
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
