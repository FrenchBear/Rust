// l37_read_lines
// Learning Rust, Iterate over lines of a text file
//
// 2025-04-08   PV      First version

//#![allow(unused)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let temp_file_path = r"C:\Temp\temp.txt";

    // Create temp file
    {
        use std::io::Write;
        let mut file = File::create(temp_file_path).expect("Failed to create temp file");
        let _ = writeln!(file, "Line 1");
        let _ = writeln!(file, "Line 2");
        let _ = writeln!(file, "Line 3");
    }

    println!("Method 1");
    if let Ok(lines) = lines_from_file(temp_file_path) {
        for line_result in lines {
            match line_result {
                Ok(line) => {
                    println!("{}", line);
                }
                Err(err) => {
                    eprintln!("Error reading line: {}", err);
                }
            }
        }
    } else {
        eprintln!("Could not open file");
    }

    println!("\nMethod 2");
    let f = File::open(temp_file_path).expect("Couldn't open file");
    for line in io::BufReader::new(f).lines().map_while(Result::ok) {
        println!("{line}");
    }

    // Delete the temporary file
    std::fs::remove_file(temp_file_path).unwrap();
}
