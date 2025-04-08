// l38_counter
// Learning Rust, Simple example of counter, sorting by values descending, printing occurrences>1
//
// 2025-04-08   PV      First version

//#![allow(unused)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let temp_file_path = r"C:\Temp\temp.txt";

    // Create temp file
    {
        use std::io::Write;
        let mut file = File::create(temp_file_path).expect("Failed to create temp file");
        let _ = writeln!(
            file,
            "os\ncodecs\ntyping\ntyping_extensions\nos\nshutil\ncommon_fs\nwebbrowser\nfolium\nos\nos\ncommon_fs\nsubprocess\ncommon_fs\nshutil\ncommon_fs\ncommon_fs\npiexif\nPIL\nos\ndatetime\nre\nos\nre\ncommon_fs\ncommon_fs\nos\nshutil\nitertools\nmath\nsys\nfunctools\nnumpy\nmultiprocessing\nbeat\npprint\ncontextlib\nos\ncommon_fs\nlibrosa\npandas\nlibrosa\npandas\nlibrosa\nos\ncommon_fs\nre\nos\nshutil\ncommon_fs\nshutil\nos\ncommon_fs\ncollections\nre\nos\nshutil\ntyping\ncommon_fs\nsyncrename\nre\nos\nshutil\ntyping\ncommon_fs\nos\nre"
        );
    }

    let f = File::open(temp_file_path).expect("Couldn't open file");
    let mut counter = HashMap::<String, u32>::new();
    for line in io::BufReader::new(f).lines().map_while(Result::ok) {
        *counter.entry(line).or_insert(0) += 1;
    }

    let mut vec: Vec<(&String, &u32)> = counter.iter().collect();
    vec.sort_by(|&a, &b| b.1.cmp(a.1));
    for (key, value) in vec.into_iter().take_while(|&x| *(x.1) > 1) {
        println!("{}\t{}", key, value);
    }

    // Delete the temporary file
    std::fs::remove_file(temp_file_path).unwrap();
}
