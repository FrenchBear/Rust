// rdups: Detect duplicate files (same content)
// First build a list of potential dups (same size) then check in depth hashing file content
//
// 2025-11-04	PV      First version

//#![allow(unused)]

// Standard library imports
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{DefaultHasher, Hasher};
use std::io;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

// External crates imports
use myglob::{MyGlobMatch, MyGlobSearch};

// -----------------------------------
// Submodules

mod options;
pub mod tests;

use options::*;

// -----------------------------------
// Global constants

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// ==============================================================================================
// Main

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

    // If no source has been provided, use stdin
    if options.sources.is_empty() {
        eprintln!("{APP_NAME}: No source provided, abort.");
        process::exit(1);
    }

    let start = Instant::now();
    let res = global_process(&options);
    if let Err(e) = res {
        eprintln!("{APP_NAME}: Error during processing: {}", e);
        process::exit(1);
    }

    let mut res = res.unwrap();
    let files_count = res.file_count;

    // Sort alphabetically duplicates
    for files in res.size_hash_dict.values_mut() {
        files.sort();
    }
    
    // Sort dictionary entries by 1st duplicate
    let mut vres = res.size_hash_dict.into_iter().collect::<Vec<_>>();
    vres.sort_by(|a, b| a.1[0].cmp(&b.1[0]));

    // Print groups
    print!("# Blocks of duplicate files ");
    if options.content_hash {
        println!("(with content hash)");
    } else {
        println!("(without content hash)");
    }

    for (size, files) in vres.into_iter() {
        if files.len() > 1 {
            println!("\n# Size {}", size);
            for file in files {
                println!("#del \"{}\"", file.display());
            }
        }
    }

    let duration = start.elapsed();

    if options.verbose {
        println!("\n# {} file{} analyzed in {:.3}s", files_count, s(files_count), duration.as_secs_f64());
    }
}

struct GlobalResult {
    file_count: usize,
    size_hash_dict: HashMap<u64, Vec<PathBuf>>,
}

// Warp all processing into a separate function so that it's easy to unit test
fn global_process(options: &Options) -> io::Result<GlobalResult> {
    let mut files_count: usize = 0;

    // Step 0: Validate all globs
    if options.verbose {
        println!("Step 0: Validation");
    }
    let mut problem = false;
    let mut globs: Vec<MyGlobSearch> = Vec::new();
    for source in options.sources.iter() {
        let resgs = MyGlobSearch::new(source).autorecurse(options.autorecurse).set_link_mode(0).compile();
        match resgs {
            Ok(gs) => {
                globs.push(gs);
            }

            Err(e) => {
                eprintln!("{APP_NAME}: Error building MyGlob: {:?}", e);
                problem = true;
            }
        }
    }
    if problem {
        return Err(io::Error::other("Error during validation step"));
    }

    // Step 1: build size_dict
    let mut already_processed: HashSet<PathBuf> = HashSet::new();
    let mut size_dict: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    if options.verbose {
        println!("Step 1: build size_dict");
    }
    for gs in globs.into_iter() {
        for ma in gs.explore_iter() {
            match ma {
                MyGlobMatch::File(pb) => {
                    if pb.file_name().unwrap().to_string_lossy().to_lowercase() == "thumbs.db" {
                        continue;
                    }
                    // Use a subfinction returning io::Result<()> so can use ? with file operations
                    let _ = process_file(&mut already_processed, &mut size_dict, pb);
                    files_count += 1;
                    if options.verbose && files_count.is_multiple_of(1000) {
                        println!("# {}", files_count);
                    }
                }
                MyGlobMatch::Dir(_) => {} // We ignore matching directories, we only look for files
                MyGlobMatch::Error(err) => {
                    if options.verbose {
                        eprintln!("{APP_NAME}: error {}", err);
                    }
                }
            }
        }
    }
    if files_count == 0 {
        return Err(io::Error::other("No matching file found, abort"));
    }
    if options.verbose {
        println!("End of step 1, {} files found", size_dict.len());
        let mut empty_files_count: usize = 0;
        let mut groups_dup_size_count: usize = 0;
        let mut dup_files_count: usize = 0;
        for sf in size_dict.iter().filter(|(_size, files)| files.len() > 1) {
            if *sf.0 == 0 {
                empty_files_count += 1;
            } else {
                groups_dup_size_count += 1;
                dup_files_count += sf.1.len();
            }
        }
        println!("Empty files: {empty_files_count}");
        println!("Groups of dup size: {groups_dup_size_count}");
        println!("Dup files: {dup_files_count}");
        println!();
    }

    if !options.content_hash {
        return Ok(GlobalResult {
            file_count: files_count,
            size_hash_dict: size_dict,
        });
    }

    // Step 2: Continue with content hashing for size duplicates
    if options.verbose {
        println!("Step 2: Hash size dups content");
    }
    let mut hashes_count: usize = 0;
    let mut size_hash_dict: HashMap<(u64, u64), Vec<PathBuf>> = HashMap::new();
    for (size, files) in size_dict.into_iter() {
        if files.len() > 1 {
            if size == 0 {
                // Don't hash empty files
                size_hash_dict.insert((0, 0), files);
            } else {
                for pb in files.into_iter() {
                    if options.verbose {
                        println!("{0:<10} {1}", size, pb.display());
                    }
                    if let Ok(hash) = hash_file_sip(&pb) {
                        let entry = size_hash_dict.entry((size, hash)).or_default();
                        entry.push(pb);
                        hashes_count += 1;
                        if options.verbose && hashes_count.is_multiple_of(100) {
                            println!("# {}", hashes_count);
                        }
                    }
                }
            }
        }
    }
    if options.verbose {
        println!("End of step 2, {} groups of dup (size, hash) found", size_hash_dict.len());
        println!("{} hashes calculated", hashes_count);
        println!();
    }

    // Step 3: Final filtering, keep groups (size, hash) with more than one file
    if options.verbose {
        println!("Step 3: Final filtering");
    }
    let mut res = HashMap::new();
    let mut blocks_count: usize = 0;
    let mut total_dups: usize = 0;
    for ((size, _hash), files) in size_hash_dict.into_iter() {
        if files.len() > 1 {
            blocks_count += 1;
            total_dups += files.len();
            res.insert(size, files);
        }
    }
    if options.verbose {
        println!("{blocks_count} blocks of dup files found");
        println!("{total_dups} dup files found");
        println!();
    }

    Ok(GlobalResult {
        file_count: files_count,
        size_hash_dict: res,
    })
}

fn process_file(already_processed: &mut HashSet<PathBuf>, size_dict: &mut HashMap<u64, Vec<PathBuf>>, pb: PathBuf) -> io::Result<()> {
    let pc = pb.canonicalize()?;
    if already_processed.contains(&pc) {
        return Ok(());
    }
    already_processed.insert(pc);

    let size = pb.metadata()?.len();
    let entry = size_dict.entry(size).or_default();
    entry.push(pb);

    Ok(())
}

fn s(n: usize) -> &'static str {
    if n > 1 { "s" } else { "" }
}

/// Hashes a file's content using DefaultHasher (SipHash).
/// WARNING: This hash will be DIFFERENT every time you run the program!
fn hash_file_sip(path: &Path) -> io::Result<u64> {
    // Open the file in a buffered reader for efficiency
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Initialize the hasher
    let mut hasher = DefaultHasher::new();

    // Create a buffer to read chunks of the file
    let mut buffer = [0; 8192]; // 8KB buffer

    loop {
        // Read a chunk from the file
        let bytes_read = reader.read(&mut buffer)?;

        // If bytes_read is 0, we've reached the end of the file
        if bytes_read == 0 {
            break;
        }

        // Feed the chunk into the hasher
        // We only hash the bytes that were actually read
        hasher.write(&buffer[..bytes_read]);
    }

    // Finalize the hash and return it
    Ok(hasher.finish())
}
