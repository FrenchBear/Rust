// fa_streams.rs - File analysis for Alternate Data Streams
//
// 2025-10-26   PV      First version, ChatGPT+Gemini and *a lot* of me to make IA code correct (and that is no trivial matter!)
// 2025-10-27   PV      Fixed stream name trimming at first \0

use std::path::Path;

use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::core::*;

use std::ffi::c_void;
use std::os::windows::ffi::OsStrExt;

use crate::Options;

#[derive(Debug)]
pub struct StreamInfo {
    pub name: String,
    pub size: u64,
}

#[derive(Debug)]
pub struct StreamsInfo {
    pub streams: Vec<StreamInfo>,
}

/// Enumerate Alternate Data Streams for a given path.
/// The default `::$DATA` stream is excluded from the results.
pub fn get_streams_information(path: &Path, _options: &Options) -> core::result::Result<StreamsInfo, String> {
    // No streams for directories or invalid links
    if path.is_dir() || path.is_symlink() {
        return  Ok(StreamsInfo { streams: Vec::new() });
    }

    if !path.is_file() {
        return Err(format!("{}: Not found", path.display()));
    }

    match get_streams_list(path, false) {
        Ok(streams) => Ok(StreamsInfo { streams }),
        Err(e) => Err(e),
    }
}

pub fn get_streams_list(path: &Path, include_main_stream: bool) -> core::result::Result<Vec<StreamInfo>, String> {
    let wide_path: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    let mut find_stream_data = WIN32_FIND_STREAM_DATA {
        cStreamName: [0; 296],
        StreamSize: 296,
    };

    let mut streams = Vec::new();

    unsafe {
        // The correct API for enumerating streams
        let h_find = match FindFirstStreamW(
            PCWSTR(wide_path.as_ptr()),
            FindStreamInfoStandard,
            &mut find_stream_data as *mut _ as *mut c_void,
            None,
        ) {
            Ok(h) => h,
            Err(e) => return Err(e.to_string()),
        };

        // Use a guard to ensure FindClose is called
        let _handle_guard = HandleGuard(h_find);

        loop {
            let stream_name = String::from_utf16_lossy(&find_stream_data.cStreamName);
            let mut cut = 0;
            for ix in 0..stream_name.len() {
                if stream_name.as_bytes()[ix] == 0 {
                    cut = ix;
                    break;
                }
            }
            let stream_name = &stream_name[..cut];

            // Print the stream name and size
            let stream_size = find_stream_data.StreamSize;

            // Note that each stream also maintains its own state for compression, encryption, and sparseness
            // in the dwFileAttributes member, but here we don't care about this level of detail

            // Exclude the default data stream
            if include_main_stream || stream_name != "::$DATA" {
                streams.push(StreamInfo {
                    name: stream_name.to_string(),
                    size: stream_size as u64,
                });
            }

            // Continue to the next stream
            if FindNextStreamW(h_find, &mut find_stream_data as *mut _ as *mut c_void).is_err() {
                break;
            }
            // os error: return Err(io::Error::from_raw_os_error(last_error.0 as i32).to_string());
        }
    }

    Ok(streams)
}

// RAII guard to ensure the handle is closed.
struct HandleGuard(HANDLE);
impl Drop for HandleGuard {
    fn drop(&mut self) {
        if self.0 != INVALID_HANDLE_VALUE {
            unsafe { FindClose(self.0) };
        }
    }
}
