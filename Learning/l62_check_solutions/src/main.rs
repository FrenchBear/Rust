// l62_check_solutions: Check Visual Studio .sln files
//
// 2025-05-14	PV      First version

#![allow(unused)]

// standard library imports
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{fs, io, process};

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
    //let globstrsource = r"C:\Development\Git*\*.sln";
    let globstrsource = r"C:\Development\GitVSTS\CSMisc\**\*.sln";

    let mut writer = logging::new(APP_NAME, APP_VERSION, true);
    let mut b = DataBag { ..Default::default() };
    let start = Instant::now();

    // Convert String sources into MyGlobSearch structs
    let resgs = MyGlobSearch::new(globstrsource).autorecurse(true).compile();
    let gs = match resgs {
        Ok(gs) => gs,
        Err(e) => {
            logln(&mut writer, format!("*** Error building MyGlob: {:?}", e).as_str());
            return;
        }
    };

    logln(&mut writer, format!("Source: {}", globstrsource).as_str());

    for ma in gs.explore_iter() {
        match ma {
            MyGlobMatch::File(pb) => process_file(&mut writer, &mut b, &pb),
            MyGlobMatch::Dir(_) => {}
            MyGlobMatch::Error(err) => {
                logln(&mut writer, format!("{APP_NAME}: MyGlobMatch error {}", err).as_str());
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

fn process_solution(writer: &mut LogWriter, sol_path: &Path, source: &str, b: &mut DataBag) {
    logln(writer, &sol_path.to_string_lossy());
    b.solutions_count += 1;

    let mut nbproj = 0;
    let mut new_source = String::new();

    for line in source.lines() {
        if line.starts_with("Project") {
            let mut new_line: Option<String> = None;
            let t1 = line.split(" = ").collect::<Vec<&str>>();
            assert_eq!(t1.len(), 2);
            let t2 = t1[1].split(", ").collect::<Vec<&str>>();
            assert!(t2.len() == 3);
            let proj_name = t2[0].trim_matches('"');
            let proj_rel_path = t2[1].trim_matches('"');
            if !proj_rel_path.ends_with(".csproj") {
                new_source.push_str(line);
                new_source.push_str("\r\n");
                continue;
            }
            b.projects_count += 1;
            nbproj += 1;

            log(writer, format!("  - {proj_name}").as_str());

            let sol_dir = sol_path.parent().unwrap().to_path_buf();
            let proj_abs_path = sol_dir.join(Path::new(proj_rel_path));
            if proj_abs_path.exists() {
                logln(writer, "");
            } else {
                log(writer, ": ");
                logln(writer, format!("{}", proj_rel_path.red()).as_str());
                b.errors_count += 1;

                let pd = proj_abs_path.parent().unwrap(); // Project directory
                let pat = pd.to_string_lossy().to_string() + "\\*.??proj";
                let matches = get_files(&pat).unwrap();
                if matches.len() == 1 {
                    let new_proj_rel_path = matches.first().unwrap().strip_prefix(sol_dir).unwrap();
                    let new_prps = new_proj_rel_path.to_string_lossy().to_string();
                    logln(
                        writer,
                        format!("{} {}", "Only one matching project: ".green(), new_prps.bright_green()).as_str(),
                    );

                    new_line = Some(format!("{} = {}, \"{}\", {}", t1[0], t2[0], new_prps, t2[2]));
                } else {
                    logln(writer, format!("{}", "No single project found".bright_red()).as_str());
                }
            }
            if let Some(nl) = new_line {
                new_source.push_str(&nl);
            } else {
                new_source.push_str(line)
            };
        } else {
            new_source.push_str(line);
        }
        new_source.push_str("\r\n");
    }
    if source != new_source {
        //let sps = sol_path.to_string_lossy().to_string().replace(".sln", "_new.sln");
        let sps = sol_path.to_string_lossy().to_string();
        logln(writer, format!("Updated solution: {}", sps).as_str());
        fs::write(&sps, new_source);
    }
    assert!(nbproj > 0);
}

/// Returns a vector matching files PathBuf (full paths), autorecurse is not used but supports ** in pattern
fn get_files(pattern: &str) -> Result<Vec<PathBuf>, MyGlobError> {
    let resgs = MyGlobSearch::build(pattern)?;

    Ok(resgs
        .explore_iter()
        .filter_map(|r| match r {
            MyGlobMatch::File(path_buf) => Some(path_buf),
            _ => None,
        })
        .collect::<Vec<_>>())
}
