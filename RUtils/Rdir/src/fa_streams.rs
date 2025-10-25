// fa_streams.rs - File analysis for Alternate Data Streams
//
// 2025-10-26   PV      First version

use std::ffi::OsString;
use std::io;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;

use windows::{
    core::PCWSTR,
    Win32::Foundation::{CloseHandle, GetLastError, ERROR_HANDLE_EOF, HANDLE, INVALID_HANDLE_VALUE},
    Win32::Storage::FileSystem::{
        FindClose, FindFirstStreamW, FindNextStreamW, FindStreamInfoStandard,
        WIN32_FIND_STREAM_DATA,
    },
};

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
pub fn get_streams_information(
    path: &Path,
    _options: &Options,
) -> Result<StreamsInfo, String> {
    let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    let mut find_stream_data: WIN32_FIND_STREAM_DATA = unsafe { std::mem::zeroed() };

    let find_handle = unsafe {
        FindFirstStreamW(
            PCWSTR(path_wide.as_ptr()),
            FindStreamInfoStandard,
            &mut find_stream_data,
            0,
        )
    };

    if find_handle == INVALID_HANDLE_VALUE {
        let last_error = unsafe { GetLastError() };
        // ERROR_HANDLE_EOF means no streams were found, which is not an error for us.
        if last_error == ERROR_HANDLE_EOF {
            return Ok(StreamsInfo { streams: vec![] });
        }
        return Err(io::Error::last_os_error().to_string());
    }

    // Use a guard to ensure FindClose is called
    let _handle_guard = HandleGuard(find_handle);

    let mut streams = Vec::new();

    loop {
        // The stream name is a null-terminated string within the cStreamName array.
        let name_len = find_stream_data.cStreamName.iter().position(|&w| w == 0).unwrap_or(0);
        let os_string = OsString::from_wide(&find_stream_data.cStreamName[..name_len]);
        let name = os_string.to_string_lossy().to_string();

        // Exclude the default data stream
        if name != "::$DATA" {
            streams.push(StreamInfo {
                name,
                size: unsafe { *find_stream_data.StreamSize.QuadPart() as u64 },
            });
        }

        if unsafe { FindNextStreamW(find_handle, &mut find_stream_data) }.as_bool() {
            continue;
        }

        let last_error = unsafe { GetLastError() };
        if last_error == ERROR_HANDLE_EOF {
            break; // No more streams
        } else {
            return Err(io::Error::from_raw_os_error(last_error.0 as i32).to_string());
        }
    }

    Ok(StreamsInfo { streams })
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