// fa_reparsepoints.rs - File analysis for reparse points
// Detects junctions and symbolic links (for file and directory) on Windows NTFS volumes
//
// 2025-04-02   PV      First version
// 2025-04-04   PV      Code streamlined, all attributes constants included
// 2025-04-21   PV      Clippy optimizations
// 2025-10-19   PV      get_tag_description
// 2025-10-25   PV      Integration in Rdir

#![allow(unused)]

// Standard library imports
use std::ffi::OsStr;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::fs::MetadataExt;
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::AsRawHandle;
use std::path::Path;
use windows::{
    Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE},
    Win32::Storage::FileSystem::{FILE_ATTRIBUTE_REPARSE_POINT, GetFileAttributesW, MAXIMUM_REPARSE_DATA_BUFFER_SIZE},
    Win32::System::IO::DeviceIoControl,
    core::PCWSTR,
};

use crate::Options;

// Define used constants
const FSCTL_GET_REPARSE_POINT: u32 = 0x900a8;
const IO_REPARSE_TAG_MOUNT_POINT: u32 = 0xA0000003;
const IO_REPARSE_TAG_SYMLINK: u32 = 0xA000000C;
const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x00200000;
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;

#[derive(Debug)]
pub struct ReparseInfo {
    pub kind: ReparseType,
    pub detail: String,
}

pub fn get_reparsepoints_information(path: &Path, _options: &Options) -> Result<ReparseInfo, String> {
    match reparse_type(path) {
        //Ok(kind) => println!("Reparse point type: {:?}  at1={:08X}  at2={:08X}", kind, attributes1(path),attributes2(path)),
        Ok((kind, detail)) => Ok(ReparseInfo { kind, detail }),
        Err(e) => Err(e),
    }
}

/// Values returned by reparse_type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ReparseType {
    No_reparse = 0,
    Symlink = 2,
    Junction = 3,
    Other = 4,
}

#[repr(C)]
struct ReparseDataBuffer {
    reparse_tag: u32,
    reparse_data_length: u16,
    reserved: u16,
    data_buffer: [u8; 1], // Placeholder for variable-length data
}

pub fn reparse_type(path: &Path) -> Result<(ReparseType, String), String> {
    // WARNING!
    // Can't use path.metadata().unwrap().file_attributes()... since REPARSE_POINT flag is always 0!!!!  Why?
    // Convert path to wide string
    let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();

    // Check if path has FILE_ATTRIBUTE_REPARSE_POINT
    let attributes = unsafe { GetFileAttributesW(PCWSTR(path_wide.as_ptr())) };
    if attributes == u32::MAX {
        return Err(io::Error::last_os_error().to_string());
    }
    if attributes & FILE_ATTRIBUTE_REPARSE_POINT.0 == 0 {
        return Ok((ReparseType::No_reparse, String::new())); // Not a reparse point
    }

    // Open file handle to query reparse data
    let file = match OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    let handle = HANDLE(file.as_raw_handle());

    if handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error().to_string());
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
        return Err(io::Error::last_os_error().to_string());
    }

    // Interpret reparse tag
    let reparse_data = unsafe { &*(buffer.as_ptr() as *const ReparseDataBuffer) };
    let tag = reparse_data.reparse_tag;

    let link_string = if path.is_symlink() {
        let target_path = fs::read_link(path).unwrap();
        let t = target_path.to_string_lossy().replace(r"\\?\", "");
        format!("-> {}", t)
    } else {
        String::new()
    };

    let (tcode, tdesc) = get_tag_description(tag);
    let msg = format!("{}: {}", tcode, tdesc);

    match tag {
        IO_REPARSE_TAG_SYMLINK => Ok((ReparseType::Symlink, link_string)),
        IO_REPARSE_TAG_MOUNT_POINT => Ok((ReparseType::Junction, link_string)),
        _ => Ok((ReparseType::Other, msg)),
    }
}

// Values from https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-fscc/c8e77b37-3909-4fe6-a4ea-2b9d423b1ee4
fn get_tag_description(tag: u32) -> (&'static str, &'static str) {
    match tag {
        0x00000000 => ("IO_REPARSE_TAG_RESERVED_ZERO", "Reserved reparse tag value."),
        0x00000001 => ("IO_REPARSE_TAG_RESERVED_ONE", "Reserved reparse tag value."),
        0x00000002 => ("IO_REPARSE_TAG_RESERVED_TWO", "Reserved reparse tag value."),
        0xA0000003 => (
            "IO_REPARSE_TAG_MOUNT_POINT",
            "Used for mount point support, specified in section 2.1.2.5.",
        ),
        0xC0000004 => ("IO_REPARSE_TAG_HSM", "Obsolete. Used by legacy Hierarchical Storage Management Product."),
        0x80000005 => ("IO_REPARSE_TAG_DRIVE_EXTENDER", "Home server drive extender.<3>"),
        0x80000006 => ("IO_REPARSE_TAG_HSM2", "Obsolete. Used by legacy Hierarchical Storage Management Product."),
        0x80000007 => (
            "IO_REPARSE_TAG_SIS",
            "Used by single-instance storage (SIS) filter driver.",
        ),
        0x80000008 => (
            "IO_REPARSE_TAG_WIM",
            "Used by the WIM Mount filter.",
        ),
        0x80000009 => (
            "IO_REPARSE_TAG_CSV",
            "Obsolete. Used by Clustered Shared Volumes (CSV) version 1 in Windows Server 2008 R2 operating system.",
        ),
        0x8000000A => (
            "IO_REPARSE_TAG_DFS",
            "Used by the Distributed File System (DFS) filter.",
        ),
        0x8000000B => ("IO_REPARSE_TAG_FILTER_MANAGER", "Used by filter manager test harness.<4>"),
        0xA000000C => ("IO_REPARSE_TAG_SYMLINK", "Used for symbolic link support."),
        0xA0000010 => (
            "IO_REPARSE_TAG_IIS_CACHE",
            "Used by Microsoft Internet Information Services (IIS) caching.",
        ),
        0x80000012 => (
            "IO_REPARSE_TAG_DFSR",
            "Used by the Distributed File System (DFS) filter.",
        ),
        0x80000013 => (
            "IO_REPARSE_TAG_DEDUP",
            "Used by the Data Deduplication (Dedup) filter.",
        ),
        0xC0000014 => ("IO_REPARSE_TAG_APPXSTRM", "Not used."),
        0x80000014 => (
            "IO_REPARSE_TAG_NFS",
            "Used by the Network File System (NFS) component.",
        ),
        0x80000015 => (
            "IO_REPARSE_TAG_FILE_PLACEHOLDER",
            "Obsolete. Used by Windows Shell for legacy placeholder files in Windows 8.1.",
        ),
        0x80000016 => (
            "IO_REPARSE_TAG_DFM",
            "Used by the Dynamic File filter.",
        ),
        0x80000017 => (
            "IO_REPARSE_TAG_WOF",
            "Used by the Windows Overlay filter, for either WIMBoot or single-file compression.",
        ),
        0x80000018 => (
            "IO_REPARSE_TAG_WCI",
            "Used by the Windows Container Isolation filter.",
        ),
        0x90001018 => (
            "IO_REPARSE_TAG_WCI_1",
            "Used by the Windows Container Isolation filter.",
        ),
        0xA0000019 => (
            "IO_REPARSE_TAG_GLOBAL_REPARSE",
            "Used by NPFS to indicate a named pipe symbolic link from a server silo into the host silo.",
        ),
        0x9000001A => (
            "IO_REPARSE_TAG_CLOUD",
            "Used by the Cloud Files filter, for files managed by a sync engine such as Microsoft OneDrive.",
        ),
        0x9000101A => (
            "IO_REPARSE_TAG_CLOUD_1",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000201A => (
            "IO_REPARSE_TAG_CLOUD_2",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000301A => (
            "IO_REPARSE_TAG_CLOUD_3",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000401A => (
            "IO_REPARSE_TAG_CLOUD_4",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000501A => (
            "IO_REPARSE_TAG_CLOUD_5",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000601A => (
            "IO_REPARSE_TAG_CLOUD_6",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000701A => (
            "IO_REPARSE_TAG_CLOUD_7",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000801A => (
            "IO_REPARSE_TAG_CLOUD_8",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000901A => (
            "IO_REPARSE_TAG_CLOUD_9",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000A01A => (
            "IO_REPARSE_TAG_CLOUD_A",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000B01A => (
            "IO_REPARSE_TAG_CLOUD_B",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000C01A => (
            "IO_REPARSE_TAG_CLOUD_C",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000D01A => (
            "IO_REPARSE_TAG_CLOUD_D",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000E01A => (
            "IO_REPARSE_TAG_CLOUD_E",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x9000F01A => (
            "IO_REPARSE_TAG_CLOUD_F",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive.",
        ),
        0x8000001B => (
            "IO_REPARSE_TAG_APPEXECLINK",
            "Used by Universal Windows Platform (UWP) packages to encode information that allows the application to be launched by CreateProcess.",
        ),
        0x9000001C => (
            "IO_REPARSE_TAG_PROJFS",
            "Used by the Windows Projected File System filter, for files managed by a user mode provider such as VFS for Git.",
        ),
        0xA000001D => (
            "IO_REPARSE_TAG_LX_SYMLINK",
            "Used by the Windows Subsystem for Linux (WSL) to represent a UNIX symbolic link.",
        ),
        0x8000001E => (
            "IO_REPARSE_TAG_STORAGE_SYNC",
            "Used by the Azure File Sync (AFS) filter.",
        ),
        0x90000027 => (
            "IO_REPARSE_TAG_STORAGE_SYNC_FOLDER",
            "Used by the Azure File Sync (AFS) filter for folder.",
        ),
        0xA000001F => (
            "IO_REPARSE_TAG_WCI_TOMBSTONE",
            "Used by the Windows Container Isolation filter.",
        ),
        0x80000020 => (
            "IO_REPARSE_TAG_UNHANDLED",
            "Used by the Windows Container Isolation filter.",
        ),
        0x80000021 => ("IO_REPARSE_TAG_ONEDRIVE", "Not used."),
        0xA0000022 => (
            "IO_REPARSE_TAG_PROJFS_TOMBSTONE",
            "Used by the Windows Projected File System filter, for files managed by a user mode provider such as VFS for Git.",
        ),
        0x80000023 => (
            "IO_REPARSE_TAG_AF_UNIX",
            "Used to represent a UNIX domain socket. No defined structure.",
        ),
        0x80000024 => (
            "IO_REPARSE_TAG_LX_FIFO",
            "Used by the Windows Subsystem for Linux (WSL) to represent a UNIX FIFO (named pipe). No defined structure.",
        ),
        0x80000025 => (
            "IO_REPARSE_TAG_LX_CHR",
            "Used by the Windows Subsystem for Linux (WSL) to represent a UNIX character special file. No defined structure.",
        ),
        0x80000026 => (
            "IO_REPARSE_TAG_LX_BLK",
            "Used by the Windows Subsystem for Linux (WSL) to represent a UNIX block special file. No defined structure.",
        ),
        0xA0000027 => (
            "IO_REPARSE_TAG_WCI_LINK",
            "Used by the Windows Container Isolation filter.",
        ),
        0xA0001027 => (
            "IO_REPARSE_TAG_WCI_LINK_1",
            "Used by the Windows Container Isolation filter.",
        ),
        _ => ("IO_REPARSE_TAG_UNKNOWN", "Unknown reparse tag."),
    }
}
