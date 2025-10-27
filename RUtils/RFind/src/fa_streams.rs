// fa_streams.rs - File analysis for Alternate Data Streams
// Simplified version, only returning a list of StreamInfo
//
// 2025-10-26   PV      First version, ChatGPT+Gemini and *a lot* of me to make IA code correct (and that is no trivial matter!)
// 2025-10-28   PV      Return io::Result<_> to be easier to integrate with other file management errors

use std::ffi::c_void;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::core::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct StreamInfo {
    pub name: String,
    pub size: u64,
}

pub fn get_streams_list(path: &Path, include_main_stream: bool) -> io::Result<Vec<StreamInfo>> {
    const STREAM_INFO_SIZE: usize = std::mem::size_of::<WIN32_FIND_STREAM_DATA>();
    let mut buffer: Vec<u8> = vec![0; STREAM_INFO_SIZE];
    let p_find_stream_data: *mut WIN32_FIND_STREAM_DATA = buffer.as_mut_ptr().cast();
    
    // Static version cause random bugs, dynamic version less frequently
    // let mut find_stream_data = WIN32_FIND_STREAM_DATA {
    //    cStreamName: [0; 296],
    //    StreamSize: 296,
    // };

    let wide_path: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();

    let mut streams = Vec::new();

    unsafe {
        // The correct API for enumerating streams
        let h_find = match FindFirstStreamW(
            PCWSTR(wide_path.as_ptr()),
            FindStreamInfoStandard,
            p_find_stream_data as *mut c_void,
            //&mut find_stream_data as *mut _ as *mut c_void,
            None,
        ) {
            Ok(h) => h,
            Err(e) => {
                return Err(io::Error::from_raw_os_error(e.code().0).into());
            }
        };
        let find_stream_data = &*p_find_stream_data;

        // Use a guard to ensure FindClose is called
        let _handle_guard = HandleGuard(h_find);

        loop {
            let stream_name = String::from_utf16_lossy(&find_stream_data.cStreamName);
            // Cut at first \0
            let mut cut = stream_name.len();
            let bytes = stream_name.as_bytes();
            for ix in 0..cut {
                if bytes[ix] == 0 {
                    cut = ix;
                    break;
                }
            }
            let stream_name = &stream_name[..cut];

            // Print the stream name and size
            let stream_size = find_stream_data.StreamSize;

            // Exclude the default data stream
            if include_main_stream || stream_name != "::$DATA" {
                streams.push(StreamInfo {
                    name: stream_name.to_string(),
                    size: stream_size as u64,
                });
            }

            // Continue to the next stream
            //if FindNextStreamW(h_find, &mut find_stream_data as *mut _ as *mut c_void).is_err() {
            if FindNextStreamW(h_find, p_find_stream_data as *mut c_void).is_err() {
                break;
            }
        }
    }

    Ok(streams)
}

// RAII guard to ensure the handle is closed.
struct HandleGuard(HANDLE);
impl Drop for HandleGuard {
    fn drop(&mut self) {
        if self.0 != INVALID_HANDLE_VALUE {
            unsafe {
                let _ = FindClose(self.0);
            };
        }
    }
}
