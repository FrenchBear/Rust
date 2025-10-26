// my_glob
// Attempt to implement an efficient glob in Rust - Main program, for testing/debugging during dev
//
// 2025-03-25   PV      First version
// 2025-10-17   PV      Case sensitive
// 2025-20-22   PV      Clippy review

#![allow(unused)]

use myglob::{MyGlobBuilder, MyGlobError, MyGlobMatch, MyGlobSearch};
use regex as _;
use std::env;
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("MyGlob lib version: {}\n", MyGlobSearch::version());

    // Error cases
    // test_myglob(r"azerty", true, false, &[], 2, true, 2, 1);
    // test_myglob(r"NUL", true, false, &[], 2, true, 2, 1);
    // test_myglob(r"Z:\hello.txt", true, false, &[], 2, true, 2, 1);
    // test_myglob(r"C:\Timp\File.txt", true, false, &[], 2, true, 2, 1);
    // test_myglob(r"C:\Temp\**\NUL", true, false, &[], 2, true, 2, 1);
    // test_myglob(r"C:\Temp\*\NUL", true, false, &[], 2, true, 2, 1);
    
    // Test optimization
    // test_myglob(r"C:\Temp\**\**\**\**\*.txt", true, false, &[], 2, true, 2, 1);
    
    // test_myglob(r"S:\**", false, false, &[], 0, true, 2, 1);
    // test_myglob(r"C:\Users\Pierr\.julia\packages\FilePathsBase\NV2We\docs\src\*", false, false, &[], 0, true, 2, 1);
    test_myglob(r"C:\**\thumbs.db", false, false, &[], 0, true, 1, 1);
}

// Entry point for testing
#[allow(clippy::too_many_arguments)]
pub fn test_myglob(
    pattern: &str,
    autorecurse: bool,
    clear_default_ignore_dirs: bool,
    extra_ignore_dirs: &[&str],
    max_depth: usize,
    case_senstitive: bool,
    link_mode: usize,
    loops: usize,
) {
    let mut durations: Vec<f64> = Vec::new();
    for pass in 0..loops {
        println!("\nTest #{pass}");

        let start = Instant::now();
        let mut builder = MyGlobSearch::new(pattern)
            .autorecurse(autorecurse)
            .max_depth(max_depth)
            .case_sensitive(case_senstitive)
            .set_link_mode(link_mode);
        if clear_default_ignore_dirs {
            builder = builder.clear_ignore_dirs();
        }
        for ignore_dir in extra_ignore_dirs {
            builder = builder.add_ignore_dir(ignore_dir);
        }
        println!("builder: {:?}", builder);

        let resgs = builder.compile();
        match resgs {
            Ok(gs) => {
                println!("gs: {:?}\n", gs);

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
                            println!("Err: «{}»", e);
                        }
                    }
                }
                let duration = start.elapsed();
                println!();
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
    if l.is_multiple_of(2) {
        (v2[l >> 1] + v2[(l >> 1) - 1]) / 2.0
    } else {
        v2[l >> 1]
    }
}
