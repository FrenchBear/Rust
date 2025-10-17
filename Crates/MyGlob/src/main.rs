// my_glob
// Attempt to implement an efficient glob in Rust - Main program, for testing/debugging during dev
//
// 2025-03-25   PV      First version
// 2025-10-17   PV      Case sensitive

#![allow(unused)]

use myglob::{MyGlobMatch, MyGlobSearch, MyGlobBuilder, MyGlobError};
use regex as _;
use std::env;
use std::path::Path;
use std::time::Instant;


fn main() {
    println!("MyGlob lib version: {}\n", MyGlobSearch::version());

    //let new_path = Path::new(r"S:\Temp");
    //_ = env::set_current_dir(&new_path);
    //test_myglob(r"S:\**\*Intel*", true, &["d2"], 0, 1);
    //test_myglob(r"C:\Temp\search1\info", false, &[], 0, 1);
    //test_myglob(r"S:\MaxDepth", true, &[], 1, 1);
    //test_myglob(r"C:\Development\GitVSTS\DevForFun\**\*.{!SOURCES}", true, &[], 2, 1);
    test_myglob(r"C:\MusicOD\Humour\**\*Eric*", true, &[], 0, true, 1);
}

// Entry point for testing
pub fn test_myglob(pattern: &str, autorecurse: bool, ignore_dirs: &[&str], maxdepth: usize, case_senstitive: bool, loops: usize) {
    let mut durations: Vec<f64> = Vec::new();
    for pass in 0..loops {
        println!("\nTest #{pass}");

        let start = Instant::now();
        let mut builder = MyGlobSearch::new(pattern).autorecurse(autorecurse).maxdepth(maxdepth).case_sensitive(case_senstitive);
        for ignore_dir in ignore_dirs {
            builder = builder.add_ignore_dir(ignore_dir);
        }
        println!("builder: {:?}", builder);

        let resgs = builder.compile();
        match resgs {
            Ok(gs) => {
                println!("gs: {:?}", gs);

                let mut nf = 0;
                let mut nd = 0;
                for ma in gs.explore_iter() {
                    match ma {
                        MyGlobMatch::File(pb) => {
                            println!("{}", pb.display());
                            nf += 1;
                        }
                        MyGlobMatch::Dir(pb) => {
                            let dir_sep = if cfg!(target_os = "windows") { '\\' } else { '/' };
                            println!("{}{dir_sep}", pb.display());
                            nd += 1;
                        }
                        MyGlobMatch::Error(e) => {
                            println!("{}", e);
                        }
                    }
                }
                let duration = start.elapsed();
                println!("{nf} file(s) found");
                println!("{nd} dir(s) found");
                println!("Iterator search in {:.3}s\n", duration.as_secs_f64());
                durations.push(duration.as_secs_f64());
            }

            Err(e) => println!("Error building MyGlob: {}", e),
        }
    }

    if loops > 1 {
        durations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("Median time: {:.3}", median(&durations));
    }
}

fn median(v: &[f64]) -> f64 {
    let l = v.len();
    if l == 0 {
        return f64::NAN;
    }
    let mut v2 = v.to_owned();
    v2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if l % 2 == 0 { (v2[l >> 1] + v2[(l >> 1) - 1]) / 2.0 } else { v2[l >> 1] }
}
