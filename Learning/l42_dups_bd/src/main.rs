// l42_dups_bd: App to find out duplicates BD after reloading in qBitTorrent
//
// 2025-04-12	PV      First version
// 2025-04-21   PV      Clippy optimizations

#![allow(unused)]

// standard library imports
use std::path::PathBuf;
use std::process;
use std::time::Instant;
use std::{collections::HashMap, fs};

// external crates imports
use myglob::{MyGlobMatch, MyGlobSearch};

// -----------------------------------
// Global constants

const APP_NAME: &str = "dups_bd";
const APP_VERSION: &str = "1.0.0";

// -----------------------------------
// Main

fn main() {
    let start = Instant::now();

    // Convert String sources into MyGlobSearch structs
    let source = r"W:\BD\{Classique,Extra,Ancien}\**\*.pdf";
    let resgs = MyGlobSearch::build(source);
    let gs = match resgs {
        Ok(gs) => gs,
        Err(e) => panic!("*** Error building MyGlob: {:?}", e),
    };

    println!("Source: {source}");

    // First collect information on files on W:\BD
    let mut files: HashMap<String, PathBuf> = HashMap::new();
    for ma in gs.explore_iter() {
        match ma {
            MyGlobMatch::File(pb) => {
                let basename = (pb.file_stem().unwrap().to_str().unwrap().to_lowercase());
                let res = files.insert(basename.clone(), pb.clone());
                if res.is_some() {
                    println!(
                        "Dup {basename}:\n  {}\n  {}\n",
                        res.unwrap().display(),
                        pb.display()
                    );
                }
            }
            MyGlobMatch::Dir(_) => {}
            MyGlobMatch::Error(err) => {
                println!("*** MyGlobMatch error {}", err);
            }
        }
    }

    if files.is_empty() {
        println!("*** No BD file found.");
        process::exit(0);
    }

    let duration = start.elapsed();
    println!(
        "{} BD files found in {:.3}s",
        files.len(),
        duration.as_secs_f64()
    );

    // Now enumerate files in
    let resgs2 = MyGlobSearch::build(r"C:\Users\Pierr\Downloads\A_Trier\!A_Trier_BD\**\*.pdf");
    let gs2 = match resgs2 {
        Ok(gs) => gs,
        Err(e) => panic!("*** Error building MyGlob: {:?}", e),
    };

    for ma in gs2.explore_iter() {
        match ma {
            MyGlobMatch::File(pb) => {
                let basename = (pb.file_stem().unwrap().to_str().unwrap().to_lowercase());
                if files.contains_key(&basename) {
                    let mpb = &files[&basename];
                    let size1 = get_file_size(&pb);
                    let size2 = get_file_size(mpb);

                    if size1.abs_diff(size2) < 500u64 {
                        println!(
                            "Matching name/size:\n  {}\n  {}\n",
                            pb.display(),
                            mpb.display()
                        );
                        trash::delete(pb);
                    } else {
                        println!(
                            "Matching name but != size:\n  {}\n  {}\n",
                            pb.display(),
                            mpb.display()
                        );
                    }
                }
            }
            MyGlobMatch::Dir(_) => {}
            MyGlobMatch::Error(err) => {
                println!("*** MyGlobMatch error {}", err);
            }
        }
    }
}

fn get_file_size(path: &PathBuf) -> u64 {
    fs::metadata(path).unwrap().len()
}
