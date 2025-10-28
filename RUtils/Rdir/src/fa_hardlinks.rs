// fa_hardlinks.rs - File analysis for hard links
//
// 2025-10-25   PV      First version

use std::fs::File;
use std::io;
use std::mem;
use std::os::windows::io::AsRawHandle;
use std::path::Path;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::FileSystem::{BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle};

use crate::Options;

#[derive(Debug)]
pub struct HardlinksInfo {
    pub hardlinks_count: u32,
}

pub fn get_hardlinks_information(path: &Path, _options: &Options) -> Result<HardlinksInfo, String> {
    // Special cases: inexistent link don't have a hard count, and this code doesn't work with directories
    if path.is_symlink() || path.is_dir() {
        return Ok(HardlinksInfo {
            hardlinks_count: 1,
        });
    }
    
    if !path.is_file() {
        return Err(format!("{}: Not found", path.display()));
    }
    
    // 1. Open the file to get a handle.
    let file = File::open(path).unwrap();
    let handle = HANDLE(file.as_raw_handle());

    // 2. Prepare an empty struct to hold the file information.
    //    `mem::zeroed` is unsafe but standard for initializing WinAPI structs.
    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { mem::zeroed() };

    // 3. Call the WinAPI function.
    let result = unsafe { GetFileInformationByHandle(handle, &mut info) };

    // 4. Check if the call succeeded.
    if result.is_err() {
        // If it fails, get the last Windows error.
        return Err(format!("os error: {}", io::Error::last_os_error()));
    }

    Ok(HardlinksInfo {
        hardlinks_count: info.nNumberOfLinks,
    })
}
