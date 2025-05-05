// L33_local_remote
// Find out if a path is local or remote (or CD-ROM, ...)
//
// 2025-04-03   PV      First version

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows::Win32::Storage::FileSystem::GetDriveTypeW;
use windows::core::PCWSTR;

const DRIVE_UNKNOWN: u32 = 0;
const DRIVE_NO_ROOT_DIR: u32 = 1;
const DRIVE_REMOVABLE: u32 = 2;
const DRIVE_FIXED: u32 = 3;
const DRIVE_REMOTE: u32 = 4;
const DRIVE_CDROM: u32 = 5;
const DRIVE_RAMDISK: u32 = 6;

fn is_local_or_remote(path: &Path) -> Result<String, String> {
    let root_path = match path.components().next() {
        Some(std::path::Component::Prefix(prefix)) => {
            match prefix.kind() {
                std::path::Prefix::Disk(disk) => {
                    // let drive_str: Vec<u16> = OsStr::new(&format!("{}:\\", disk as char)).encode_wide().chain(std::iter::once(0)).collect();
                    // drive_str
                    &format!("{}:\\", disk as char)
                }
                // std::path::Prefix::VerbatimDisk(disk) => {
                //     let drive_str: Vec<u16> = OsStr::new(&format!("{}:\\", disk as char)).encode_wide().chain(std::iter::once(0)).collect();
                //     drive_str
                // }
                std::path::Prefix::UNC(server, share) => {
                    // let unc_str: Vec<u16> = OsStr::new(&format!("\\\\{}\\{}\\", server.to_string_lossy(), share.to_string_lossy())).encode_wide().chain(std::iter::once(0)).collect();
                    // unc_str
                    &format!(
                        "\\\\{}\\{}\\",
                        server.to_string_lossy(),
                        share.to_string_lossy()
                    )
                }
                _ => {
                    // Other cases: 
                    // Prefix::Verbatim(os_str)
                    // Prefix::VerbatimUNC(os_str, os_str1)
                    // Prefix::VerbatimDisk(_)
                    // Prefix::DeviceNS(os_str)
                    // Prefix::UNC(os_str, os_str1)
                    // Prefix::Disk(_)
            return Err("Path is not a drive or UNC path".to_string());
                }
            }
        }
        _ => {
            return Err("Path does not have a drive component".to_string());
        }
    };

    //println!("Drive_str: {:?}", drive_letter);
    let lprootpathname: Vec<u16> = OsStr::new(root_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        match GetDriveTypeW(PCWSTR(lprootpathname.as_ptr())) {
            DRIVE_UNKNOWN | DRIVE_NO_ROOT_DIR => Ok("Unknown/Invalid/Inexistent".into()),
            DRIVE_FIXED => Ok("Local".to_string()),
            DRIVE_REMOVABLE => Ok("Removable".to_string()),
            DRIVE_REMOTE => Ok("Remote".to_string()),
            DRIVE_CDROM => Ok("CD-ROM".to_string()),
            DRIVE_RAMDISK => Ok("RAM Disk".to_string()),
            n => Ok(format!("Other: {n}")),
        }
    }
}

fn main() {
    test(Path::new("K:\\"));
    test(Path::new("C:\\Windows"));
    test(Path::new("B:\\"));
    test(Path::new("E:\\"));
    test(Path::new("\\\\teraz\\temp\\file.txt"));
    test(Path::new("\\\\teraz\\timp\\file.txt"));
    test(Path::new("X:\\"));

    // K:\ is Unknown
    // C:\Windows is Local
    // B:\ is Local
    // E:\ is Removable
    // \\teraz\temp\file.txt is Remote
    // \\teraz\thing\file.txt is Unknown
    // X:\ is Remote
}

fn test(path: &Path) {
    match is_local_or_remote(path) {
        Ok(result) => println!("{} is {}", path.display(), result),
        Err(err) => println!("Error: {}", err),
    }
}
