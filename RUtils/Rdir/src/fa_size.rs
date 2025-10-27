// fa_size.rs - File analysis for size
//
// 2025-10-25   PV      First version

use std::{fs, os::windows::fs::MetadataExt};

use std::ffi::{OsString, c_void};
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::core::*;

use crate::Options;

#[derive(Debug)]
pub struct SizeInfo {
    pub size: u64,         // Apparent size, ignoring ADS, compression, sparse files and cluster counding
    pub size_on_disk: u64, // Total size including ADS, compression, sparse files and cluster rounding
    pub dir_filescount: u32,
    pub dir_dirscount: u32,
    pub dir_linkscount: u32,
}

pub fn get_size_information(path: &Path, options: &Options) -> core::result::Result<SizeInfo, String> {
    // Special case, link to invalid target
    if path.is_symlink() && !options.show_link_target_info {
        return Ok(SizeInfo {
            size: 0,
            size_on_disk: 0,
            dir_filescount: 0,
            dir_dirscount: 0,
            dir_linkscount: 0,
        });
    }

    if !path.is_dir() && !path.is_file() && !path.is_symlink() {
        return Err(format!("{}: Not found", path.display()));
    }

    let meta_res = if path.is_symlink() && !options.show_link_target_info {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };

    // Dir size is different, we return the count of files, directories and links
    if path.is_dir() {
        let mut dir_filescount = 0;
        let mut dir_dirscount = 0;
        let mut dir_linkscount = 0;

        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_symlink() {
                // Includes files links, dir links and invalid links
                dir_linkscount += 1;
            }
            else if path.is_dir() {
                dir_dirscount += 1;
            } else if path.is_file() {
                dir_filescount += 1;
            }
        }

        return Ok(SizeInfo {
            size: 0,
            size_on_disk: 0,
            dir_filescount,
            dir_dirscount,
            dir_linkscount,
        });
    }

    let meta = match meta_res {
        Ok(m) => m,
        Err(e) => return Err(e.to_string()),
    };

    // Get size as shown by internet explorer
    let size_on_disk = get_size_on_disk_with_ads(path)?;

    // This should't work for directories
    Ok(SizeInfo {
        size: meta.len(),
        size_on_disk,
        dir_filescount: 0,
        dir_dirscount: 0,
        dir_linkscount: 0,
    })
}

/// This function accounts for file compression and sparse files.
/// Returs info for main data stream or for a single specific data dtream
pub fn get_stream_size_on_disk(path: &Path) -> std::io::Result<u64> {
    // 1. Convert the Rust &Path into a null-terminated UTF-16 "wide string"
    //    that the Windows API expects.
    let path_wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0)) // Add the null terminator
        .collect();

    let mut high_order: u32 = 0;
    let high_order_ptr: *mut u32 = &mut high_order;

    // 2. Call the Win32 API function
    let low_order: u32 = unsafe { GetCompressedFileSizeW(PCWSTR(path_wide.as_ptr()), Some(high_order_ptr)) };

    // 3. Check for errors
    if low_order == INVALID_FILE_SIZE {
        let error: WIN32_ERROR = unsafe { GetLastError() };

        // A return of INVALID_FILE_SIZE is an error *unless* GetLastError
        // returns NO_ERROR (0), which can happen for a file that is exactly
        // 4,294,967,295 bytes and not compressed.
        if error != NO_ERROR {
            return Err(std::io::Error::from_raw_os_error(error.0 as i32));
        }
    }

    // 4. Combine the high and low 32-bit parts into a single 64-bit unsigned integer
    let size_on_disk = ((high_order as u64) << 32) | (low_order as u64);

    Ok(size_on_disk)
}

/// Retrieves the number of bytes per allocation cluster for the
/// drive that contains the given path.
pub fn get_bytes_per_cluster(path: &Path) -> io::Result<u64> {
    // 1. We must first find the root of the path (e.g., "C:\").
    // To do this reliably, we make the path absolute first.
    let canonical_path = path.canonicalize()?;

    // 2. Get the root component (ancestors().last() gives the root).
    let root_path = canonical_path
        .ancestors()
        .last()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Could not find root path"))?;

    // 3. Convert the root path to a null-terminated UTF-16 string.
    let root_path_wide: Vec<u16> = root_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();

    // 4. Prepare output variables for the API call
    let mut sectors_per_cluster: u32 = 0;
    let mut bytes_per_sector: u32 = 0;
    let mut _number_of_free_clusters: u32 = 0;
    let mut _total_number_of_clusters: u32 = 0;

    // 5. Call the Win32 API
    let result = unsafe {
        GetDiskFreeSpaceW(
            PCWSTR(root_path_wide.as_ptr()),
            Some(&mut sectors_per_cluster as *mut _),
            Some(&mut bytes_per_sector as *mut _),
            Some(&mut _number_of_free_clusters as *mut _),
            Some(&mut _total_number_of_clusters as *mut _),
        )
    };

    // 6. Check for errors
    if result.is_err() {
        // API call failed
        let error_code = unsafe { GetLastError() };
        return Err(io::Error::from_raw_os_error(error_code.0 as i32));
    }

    // 7. Calculate the final value
    let bytes_per_cluster = (sectors_per_cluster as u64) * (bytes_per_sector as u64);

    if bytes_per_cluster == 0 {
        Err(io::Error::other("FileSystem reported 0 bytes per cluster"))
    } else {
        Ok(bytes_per_cluster)
    }
}

/// Calculates the "Size on disk" as shown in Windows Explorer.
///
/// This accounts for compression, sparse files, and cluster rounding.
/// It does NOT account for Alternate Data Streams (ADS).
pub fn get_explorer_size_on_disk(path: &Path) -> io::Result<u64> {
    // 1. Get the compressed/sparse size
    let compressed_size = get_stream_size_on_disk(path)?;

    // Handle the simple case
    if compressed_size == 0 {
        return Ok(0);
    }

    // 2. Get the cluster size for the drive
    let bytes_per_cluster = get_bytes_per_cluster(path)?;

    // 3. Round the compressed size up to the nearest cluster
    // let total_clusters = (compressed_size + bytes_per_cluster - 1) / bytes_per_cluster;
    let total_clusters = compressed_size.div_ceil(bytes_per_cluster);
    let size_on_disk = total_clusters * bytes_per_cluster;

    Ok(size_on_disk)
}

pub fn get_size_on_disk_with_ads(path: &Path) -> core::result::Result<u64, String> {
    // let streams = match crate::fa_streams::get_streams_list(path, true) {
    //     Ok(si) => si,
    //     Err(e) => return Err(e),
    // };
    let streams = crate::fa_streams::get_streams_list(path, true)?;

    let bytes_per_cluster = match get_bytes_per_cluster(path) {
        Ok(bpc) => bpc,
        Err(e) => return Err(e.to_string()),
    };

    let path_str = path.as_os_str().to_str().unwrap();
    let mut total_size_on_disk = 0_u64;
    for si in streams {
        let streampath = path_str.to_string() + si.name.as_str();
        let streampath_path = Path::new(&streampath);
        let mut compressed_size = match get_stream_size_on_disk(streampath_path) {
            Ok(size) => size,
            Err(e) => return Err(e.to_string()),
        };

        // round to cluster size
        let size_on_disk = if compressed_size != 0 {
            //let total_clusters = (compressed_size + bytes_per_cluster - 1) / bytes_per_cluster;
            let total_clusters = compressed_size.div_ceil(bytes_per_cluster);
            total_clusters * bytes_per_cluster
        } else {
            0
        };

        //println!("{}: {} -> {} bytes", streampath, si.size, size_on_disk);

        total_size_on_disk += size_on_disk;
    }

    Ok(total_size_on_disk)
}
