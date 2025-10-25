// l32c_streams
// Enumerate streams (ADS) on a NTFS volume
//
// 2025-10-25   PV      First version, ChatGPT+Gemini+a lot of me to make IA code correct (and that is no trivial matter!)

#![allow(unused)]

use std::io;

use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::Threading::*;
use windows::core::*;

use std::ffi::{OsString, c_void};
use std::os::windows::ffi::OsStrExt;

fn enumerate_ads(path: &str) -> Result<()> {
    // Convert the path to a wide string
    let wide_path: Vec<u16> = OsString::from(path)
        .encode_wide()
        .chain(Some(0)) // null terminator
        .collect();

    unsafe {
        let mut find_data = WIN32_FIND_STREAM_DATA {
            cStreamName: [0; 296],
            StreamSize: 296,
            // dwStreamDataType: 0,
            // dwStreamSize: 0,
        };

        // The correct API for enumerating streams
        let h_find = FindFirstStreamW(
            PCWSTR(wide_path.as_ptr()),
            FindStreamInfoStandard,
            &mut find_data as *mut _ as *mut c_void,
            None,
        )?;

        loop {
            let stream_name = String::from_utf16_lossy(&find_data.cStreamName);
            let stream_name = stream_name.trim_end_matches(char::from(0)); // remove null terminator

            // Print the stream name and size
            let file_size = find_data.StreamSize;
            println!("Stream: {}, Size: {} bytes", stream_name, file_size);

            // Continue to the next stream
            if FindNextStreamW(h_find, &mut find_data as *mut _ as *mut c_void).is_err() {
                break;
            }
        }

        // Clean up
        FindClose(h_find);
    }

    Ok(())
}

fn main() -> Result<()> {
    let path = r"s:\Streams\file.txt";
    enumerate_ads(path)
}
