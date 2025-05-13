// l62_check_solutions: Check Visual Studio .sln files
//
// 2025-05-14	PV      First version

#![allow(unused)]

// standard library imports
use std::fmt::Debug;
use std::path::{Path,PathBuf};
use std::{io, process};
use std::time::Instant;

// external crates imports
use colored::*;
use logging::*;
use myglob::{MyGlobError, MyGlobMatch, MyGlobSearch};
use textautodecode::{TextAutoDecode, TextFileEncoding};

// -----------------------------------
// Global constants

const APP_NAME: &str = "check_solutions";
const APP_VERSION: &str = "1.0.0";

// -----------------------------------
// Main

#[derive(Debug, Default)]
struct DataBag {
    solutions_count: usize,
    projects_count: usize,
    errors_count: usize,
}

fn tmain() {
    let mut b = DataBag { ..Default::default() };
    process_file(
        &mut None,
        &mut b,
        Path::new(r"C:\Development\GitVSTS\CSMisc\Net9\CS25_Gnu.Getopt.Getopt\CS25_Gnu.Getopt.sln"),
    );
}

fn main() {
    // Use autorecurse
    //let globstrsources: Vec<String> = vec![r"C:\Development\Git*\*.sln".to_string()];
    let globstrsources: Vec<String> = vec![r"C:\Development\GitVSTS\CSMisc\Net9\**\*.sln".to_string()];

    // Prepare log writer
    let mut writer = logging::new(APP_NAME, APP_VERSION, true);

    let mut b = DataBag { ..Default::default() };

    let start = Instant::now();

    // Convert String sources into MyGlobSearch structs
    let mut sources: Vec<(&String, MyGlobSearch)> = Vec::new();
    for source in globstrsources.iter() {
        let resgs = MyGlobSearch::new(source).autorecurse(true).compile();
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

    if b.solutions_count == 0 {
        logln(&mut writer, "*** No file found, nothing to report.");
    } else {
        let duration = start.elapsed();
        logln(&mut writer, "");
        log(
            &mut writer,
            format!("{} soultion(s), {} project(s)", b.solutions_count, b.projects_count).as_str(),
        );
        log(&mut writer, format!(", {} error(s)", b.errors_count).as_str());
        logln(&mut writer, format!(" found in {:.3}s", duration.as_secs_f64()).as_str());
    }
}

fn process_file(writer: &mut LogWriter, b: &mut DataBag, p: &Path) {
    let res = TextAutoDecode::read_text_file(p);
    match res {
        Ok(tad) => {
            if tad.encoding == TextFileEncoding::NotText {
                b.errors_count += 1;
                logln(writer, format!("*** Non-text file: {}", p.display()).as_str());
                return;
            } else {
                process_solution(writer, p, tad.text.unwrap().as_str(), b);
            }
        }
        Err(e) => {
            b.errors_count += 1;
            logln(writer, format!("*** Error reading file {}: {}", p.display(), e).as_str());
        }
    }
}

fn process_solution(writer: &mut LogWriter, path: &Path, source: &str, b: &mut DataBag) {
    b.solutions_count += 1;

    let mut nbproj = 0;
    println!("{}", path.display());
    for line in source.lines() {
        if line.starts_with("Project") {
            let t1 = line.split(" = ").collect::<Vec<&str>>();
            assert_eq!(t1.len(), 2);
            let t2 = t1[1].split(", ").collect::<Vec<&str>>();
            assert!(t2.len() == 3);
            let proj_name = t2[0].trim_matches('"');
            let proj_path = t2[1].trim_matches('"');
            if !proj_path.ends_with(".csproj") {
                continue;
            }
            b.projects_count += 1;
            nbproj += 1;

            print!("  - {proj_name}");

            let ps = path.parent().unwrap().to_path_buf();
            let pp = ps.join(Path::new(proj_path));
            if pp.exists() {
                println!();
            } else {
                print!(": ");
                println!("{}", proj_path.red());
                b.errors_count += 1;

                let pd = pp.parent().unwrap();
                let pat = pd.to_string_lossy().to_string() + "\\*.??proj";
                let matches = get_files(&pat).unwrap();
                if matches.len()==1 {
                    println!("{} {}", "Only one matching project: ".green(), matches.first().unwrap().to_string_lossy().to_string().bright_green());
                } else {
                    println!("{}", "No single project found".bright_red());
                }
            }
        }
    }
    assert!(nbproj > 0);
}

/// Returns a vector matching files PathBuf (full paths), autorecurse is not used but supports ** in pattern
fn get_files(pattern: &str) -> Result<Vec<PathBuf>, MyGlobError> {
    let resgs = MyGlobSearch::build(pattern)?;
    let mut res:Vec<PathBuf>=Vec::new();
    for ma in resgs.explore_iter() {
        match ma {
            MyGlobMatch::File(path_buf) => res.push(path_buf),
            _ => {},
        }
    }
    Ok(res)
}