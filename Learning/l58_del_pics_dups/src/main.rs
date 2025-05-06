// l58_del_pics_dups
// Remove pics dups
//
// 2025-05-06   PV      Initial code from Gemini

// I would like to use this hashing function to find and delete duplicate files in folder and all its subfolders.
// Here is the process:
// First find duplicates of size and file extension, that is, build a list of lists of files with the same size, same
// extension. SInce there is no need to read file content to get size, this should go fast.
// Second step, we're only interested in lists containing at least two files with the same size, same extension. We want
// to detect "real" duplicates by hashing content for each files of the list, and only keep 1 copy of a file with a
// specific hash. Any second, third... file with the same hash should be deleted.

//#![allow(unused)]

use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

// Simple file hashing, not cryptographically secure and a risk of collisions, but to determine if potential pics dups
// (same size, same extension) are itentical, it's more than enough. Note that since I read the whole file into a
// Vec<u8>, I could simply directly compare the vectors themselves, but it's easier to use a simple u64 value as a
// HashMap key to detect real duplicates
fn simple_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0;
    for &byte in data {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

fn hash_file(file_path: &PathBuf) -> io::Result<u64> {
    let mut file = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(simple_hash(&buffer))
}

// First step, create a HashMap of list of files indexed by (size, extension)
fn find_duplicate_groups(sources: &[&Path]) -> io::Result<HashMap<(u64, Option<String>), Vec<PathBuf>>> {
    let mut groups: HashMap<(u64, Option<String>), Vec<PathBuf>> = HashMap::new();

    fn traverse(dir: &Path, groups: &mut HashMap<(u64, Option<String>), Vec<PathBuf>>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    let file_size = metadata.len();
                    let extension = path.extension().and_then(|s| s.to_str().map(|e| e.to_lowercase()));
                    groups.entry((file_size, extension)).or_default().push(path);
                }
            } else if path.is_dir() {
                traverse(&path, groups)?;
            }
        }
        Ok(())
    }

    for root_dir in sources {
        traverse(root_dir, &mut groups)?;
    }
    Ok(groups)
}

// Second step, we actually don't care about size and extension of the group, the only thing we care about are
// groups of more than 1 file
fn find_and_delete_duplicates(size_extension_groups: &HashMap<(u64, Option<String>), Vec<PathBuf>>) -> io::Result<()> {
    for files in size_extension_groups.values() {
        if files.len() > 1 {
            //println!("Checking for real duplicates within: {:?}", files);
            let mut content_hashes: HashMap<u64, PathBuf> = HashMap::new();
            let mut files_to_delete: Vec<PathBuf> = Vec::new();

            for file_path in files {
                match hash_file(file_path) {
                    Ok(hash) => {
                        if content_hashes.contains_key(&hash) {
                            println!(
                                "Duplicate found: {} (same content as {})",
                                file_path.display(),
                                content_hashes.get(&hash).unwrap().display()
                            );
                            files_to_delete.push(file_path.clone());
                        } else {
                            content_hashes.insert(hash, file_path.clone());
                        }
                    }
                    Err(e) => eprintln!("Error hashing file {:?}: {}", file_path, e),
                }
            }

            for file_to_delete in &files_to_delete {
                println!("Deleting: {:?}", file_to_delete);

                //match fs::remove_file(file_to_delete) {
                match trash::delete(file_to_delete) {
                    Ok(_) => {
                        // println!("Successfully deleted: {:?}", file_to_delete);
                    }
                    Err(e) => eprintln!("Error deleting {:?}: {}", file_to_delete, e),
                }
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    /*
    let target_dir = Path::new("./duplicate_test_dir");

    // Create a dummy directory structure with some duplicate files
    fs::create_dir_all(target_dir.join("subdir1"))?;
    fs::create_dir_all(target_dir.join("subdir2"))?;

    fs::write(target_dir.join("file1.txt"), "This is the content.")?;
    fs::write(target_dir.join("file2.txt"), "This is the content.")?; // Duplicate of file1.txt
    fs::write(target_dir.join("file3.txt"), "Different content.")?;
    fs::write(target_dir.join("subdir1").join("file4.txt"), "This is the content.")?; // Another duplicate
    fs::write(target_dir.join("subdir2").join("file5.txt"), "Another content.")?;
    fs::write(target_dir.join("subdir2").join("file6.log"), "Some log data.")?;
    fs::write(target_dir.join("subdir2").join("file7.log"), "Some log data.")?; // Duplicate

    println!("Starting duplicate file detection and deletion in: {:?}", target_dir);
    find_and_delete_duplicates(target_dir)?;
    println!("Finished duplicate file detection and deletion.");

    // Clean up the dummy directory (optional)
    fs::remove_dir_all(target_dir)?;

    */
    let sources = vec![
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Armpits"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Art"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\BDSM Leather"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Bear tats"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Bears and bears"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Bears"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Best Of"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Breeding"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Cages"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Crade"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Dogs"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Drawings"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\FF"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Gif"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\IA"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Jus"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Monsters"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Mp4"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Other"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Red Heads"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Sex"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Skins Cops Milit"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Tools"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\Uro"),
        Path::new(r"D:\Kaforas\OneDrive\PicturesODKB\ZM"),
    ];

    let size_extension_groups = find_duplicate_groups(&sources)?;
    find_and_delete_duplicates(&size_extension_groups)?;

    Ok(())
}
