// my_glob
// Attempt to implement an efficient glob in Rust - Main program, for testing/debugging during dev
//
// 2025-03-25   PV  First version

//#![allow(unused)]

use myglob::{MyGlobMatch, MyGlobSearch};
use std::time::Instant;
use regex as _;

fn main() {
    println!("MyGlob lib version: {}\n", MyGlobSearch::version());

    test_myglob(r"C:\Temp\*.log", true, &vec!["d2"], 1);
}

// Entry point for testing
pub fn test_myglob(pattern: &str, autorecurse: bool, ignore_dirs: &[&str], loops: usize) {
    let mut durations: Vec<f64> = Vec::new();
    for pass in 0..loops {
        println!("\nTest #{pass}");

        let start = Instant::now();
        let mut builder = MyGlobSearch::new(pattern).autorecurse(autorecurse);
        for ignore_dir in ignore_dirs {
            builder = builder.add_ignore_dir(ignore_dir);
        }
        let resgs=builder.compile();

        match resgs {
            Ok(gs) => {
                let mut nf = 0;
                let mut nd = 0;
                for ma in gs.explore_iter() {
                    match ma {
                        MyGlobMatch::File(pb) => {
                            println!("{}", pb.display());
                            nf += 1;
                        }
                        MyGlobMatch::Dir(pb) => {
                            println!("{}\\", pb.display());
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

            Err(e) => println!("Error building MyGlob: {:?}", e),
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
