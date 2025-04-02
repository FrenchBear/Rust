// l32_symlink_junction - Learning Rust
// Detects junctions and symbolic links (for file and directory) on Windows NTFS volumes
// Big help from ChatGPT for the code, while Gemini was hallucinating inexistent functions and useless
//
// 2025-04-02   PV      First version

//#![allow(unused)]

use std::path::Path;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::fs::OpenOptions;
use std::io;
use windows::{
    Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE},
    Win32::Storage::FileSystem::{ GetFileAttributesW, FILE_ATTRIBUTE_REPARSE_POINT, MAXIMUM_REPARSE_DATA_BUFFER_SIZE, },
    Win32::System::IO::DeviceIoControl,
    core::PCWSTR,
};
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::AsRawHandle;

// Manually define missing constants
const FSCTL_GET_REPARSE_POINT: u32 = 0x900a8;
const IO_REPARSE_TAG_MOUNT_POINT: u32 = 0xA0000003;
const IO_REPARSE_TAG_SYMLINK: u32 = 0xA000000C;
const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x00200000;
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;

fn main() {
    show(r"C:\truc_inexistant");    // Does not exist
    show(r"C:\vfcompat.dll");       // Simple file
    show(r"C:\vfcompat_link.dll");  // File link
    show(r"C:\vfcompat.dll - Shortcut.lnk");  // Shell link, just seen as a plain file
    show(r"C:\Backup");             // Simple directory
    show(r"C:\Downloads");          // Directory link
    show(r"C:\Development");        // Junction
    show(r"\\teraz\temp");          // ?
}

fn show(filename: &str) {
    let path = Path::new(filename);
    if path.exists() {
        if path.is_file() {
            println!("File: {}", path.display());
        } else if path.is_dir() {
            println!("Dir: {}", path.display());
        } else if path.is_symlink() {
            println!("Link: {}", path.display());
        } else {
            println!("Other: {:?}", path);
        }

        match is_reparse_point_and_type(filename) {
            Ok(Some(kind)) => println!("Reparse point type: {}", kind),
            Ok(None) => println!("Not a reparse point"),
            Err(e) => eprintln!("Error: {}", e),
        }
        println!();
    
    } else {
        println!("Does not exist: {}\n", path.display())
    }
}

#[repr(C)]
struct ReparseDataBuffer {
    reparse_tag: u32,
    reparse_data_length: u16,
    reserved: u16,
    data_buffer: [u8; 1], // Placeholder for variable-length data
}

fn is_reparse_point_and_type(path: &str) -> io::Result<Option<&'static str>> {
    // Convert path to wide string
    let path_wide: Vec<u16> = OsStr::new(path).encode_wide().chain(std::iter::once(0)).collect();

    // Check if path has FILE_ATTRIBUTE_REPARSE_POINT
    let attributes = unsafe { GetFileAttributesW(PCWSTR(path_wide.as_ptr())) };
    if attributes == u32::MAX {
        return Err(io::Error::last_os_error());
    }
    if attributes & FILE_ATTRIBUTE_REPARSE_POINT.0 == 0 {
        return Ok(None); // Not a reparse point
    }

    // Open file handle to query reparse data
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)?;
    let handle = HANDLE(file.as_raw_handle() as isize);

    if handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }

    // Prepare buffer and call DeviceIoControl
    let mut buffer = vec![0u8; MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize];
    let mut bytes_returned: u32 = 0;

    let success = unsafe {
        DeviceIoControl(
            handle,
            FSCTL_GET_REPARSE_POINT,
            None, // No input buffer
            0,    // Input buffer size
            Some(buffer.as_mut_ptr() as *mut _),
            buffer.len() as u32,
            Some(&mut bytes_returned),
            None, // No overlapped structure
        )
    };

    if success.is_err() {
        return Err(io::Error::last_os_error());
    }

    // Interpret reparse tag
    let reparse_data = unsafe { &*(buffer.as_ptr() as *const ReparseDataBuffer) };
    let tag = reparse_data.reparse_tag;

    let kind = match tag {
        IO_REPARSE_TAG_SYMLINK => Some("Symbolic Link"),
        IO_REPARSE_TAG_MOUNT_POINT => Some("Junction"),
        _ => Some("Other Reparse Point"),
    };

    Ok(kind)
}
