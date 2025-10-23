// l32b_hardlink - Learning Rust
// Detects hard links on Windows NTFS volumes
// Code provided by Gemini, obsolete, I fixed it
//
// 2025-10-23   PV      First version

#![allow(unused)]

use std::fs::File;
use std::io;
use std::mem;
use std::os::windows::io::AsRawHandle;
use std::path::Path;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::FileSystem::{
    GetFileInformationByHandle,
    BY_HANDLE_FILE_INFORMATION,
};

/// Gets the hard link count for a file on Windows.
///
/// Returns `Ok(link_count)` on success.
/// A `link_count > 1` indicates the file is a hard link.
fn get_windows_link_count(path: &Path) -> io::Result<u32> {
    // 1. Open the file to get a handle.
    let file = File::open(path)?;
    let handle = HANDLE(file.as_raw_handle());

    // 2. Prepare an empty struct to hold the file information.
    //    `mem::zeroed` is unsafe but standard for initializing WinAPI structs.
    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { mem::zeroed() };

    // 3. Call the WinAPI function.
    let result = unsafe {
        GetFileInformationByHandle(handle, &mut info)
    };

    // 4. Check if the call succeeded.
    if result.is_err() {
        // If it fails, get the last Windows error.
        return Err(io::Error::last_os_error());
    }

    // 5. Return the number of links.
    Ok(info.nNumberOfLinks)
}

fn main() {

    let path_original = Path::new(r"C:\Temp\original.txt");
    let path_hardlink = Path::new(r"C:\Temp\hardlink.txt");

    // ---- SETUP ----
    // Create a file and a hard link to it.
    let _ = File::create(path_original);
    let _ = std::fs::hard_link(path_original, path_hardlink);

    match get_windows_link_count(path_original) {
        Ok(count) => {
            println!("'original.txt' link count: {}", count); // Will be 2
            if count > 1 {
                println!("'original.txt' is a hard link.");
            } else {
                println!("'original.txt' is not a hard link.");
            }
        }
        Err(e) => eprintln!("Error checking 'original.txt': {}", e),
    }

    match get_windows_link_count(path_hardlink) {
        Ok(count) => {
            println!("'hardlink.txt' link count: {}", count); // Will be 2
            if count > 1 {
                println!("'hardlink.txt' is a hard link.");
            } else {
                println!("'hardlink.txt' is not a hard link.");
            }
        }
        Err(e) => eprintln!("Error checking 'hardlink.txt': {}", e),
    }

    // ---- CLEANUP ----
    // let _ = std::fs::remove_file(path_original);
    // let _ = std::fs::remove_file(path_hardlink);
}