// l32a_symlink_junction - Learning Rust
// Detects junctions and symbolic links (for file and directory) on Windows NTFS volumes
// Big help from ChatGPT for the code, while Gemini was hallucinating inexistent functions and useless
//
// 2025-04-02   PV      First version
// 2025-04-04   PV      Code streamlined, all attributes constants included
// 2025-04-21   PV      Clippy optimizations
// 2025-10-19   PV      get_tag_description

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

// Define used constants
const FSCTL_GET_REPARSE_POINT: u32 = 0x900a8;
const IO_REPARSE_TAG_MOUNT_POINT: u32 = 0xA0000003;
const IO_REPARSE_TAG_SYMLINK: u32 = 0xA000000C;
const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x00200000;
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;

// Windows file attributes
// https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
const FILE_ATTRIBUTE_READONLY: u32 = 0x00000001;                // A file that is read-only. Applications can read the file, but cannot write to it or delete it. This attribute is not honored on directories.
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;                  // The file or directory is hidden. It is not included in an ordinary directory listing.
const FILE_ATTRIBUTE_SYSTEM: u32 = 0x00000004;                  // A file or directory that the operating system uses a part of, or uses exclusively.
const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x00000010;               // The handle that identifies a directory.
const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x00000020;                 // A file or directory that is an archive file or directory. Applications typically use this attribute to mark files for backup or removal.
const FILE_ATTRIBUTE_DEVICE: u32 = 0x00000040;                  // This value is reserved for system use.
const FILE_ATTRIBUTE_NORMAL: u32 = 0x00000080;                  // A file that does not have other attributes set. This attribute is valid only when used alone.
const FILE_ATTRIBUTE_TEMPORARY: u32 = 0x00000100;               // A file that is being used for temporary storage. File systems avoid writing data back to mass storage if sufficient cache memory is available, because typically, an application deletes a temporary file after the handle is closed. In that scenario, the system can entirely avoid writing the data. Otherwise, the data is written after the handle is closed.
const FILE_ATTRIBUTE_SPARSE_FILE: u32 = 0x00000200;             // A file that is a sparse file.
const FILE_ATTRIBUTE_REPARSE_POINT_TYPE: u32 = 0x00000400;      // A file or directory that has an associated reparse point, or a file that is a symbolic link.
const FILE_ATTRIBUTE_COMPRESSED: u32 = 0x00000800;              // A file or directory that is compressed. For a file, all of the data in the file is compressed. For a directory, compression is the default for newly created files and subdirectories.
const FILE_ATTRIBUTE_OFFLINE: u32 = 0x00001000;                 // The data of a file is not available immediately. This attribute indicates that the file data is physically moved to offline storage. This attribute is used by Remote Storage, which is the hierarchical storage management software. Applications should not arbitrarily change this attribute.
const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: u32 = 0x00002000;     // The file or directory is not to be indexed by the content indexing service.
const FILE_ATTRIBUTE_ENCRYPTED: u32 = 0x00004000;               // A file or directory that is encrypted. For a file, all data streams in the file are encrypted. For a directory, encryption is the default for newly created files and subdirectories.
const FILE_ATTRIBUTE_INTEGRITY_STREAM: u32 = 0x00008000;        // The directory or user data stream is configured with integrity (only supported on ReFS volumes). It is not included in an ordinary directory listing. The integrity setting persists with the file if it's renamed. If a file is copied the destination file will have integrity set if either the source file or destination directory have integrity set.
const FILE_ATTRIBUTE_VIRTUAL: u32 = 0x00010000;                 // This value is reserved for system use.
const FILE_ATTRIBUTE_NO_SCRUB_DATA: u32 = 0x00020000;           // The user data stream not to be read by the background data integrity scanner (AKA scrubber). When set on a directory it only provides inheritance. This flag is only supported on Storage Spaces and ReFS volumes. It is not included in an ordinary directory listing.
const FILE_ATTRIBUTE_EA: u32 = 0x00040000;                      // A file or directory with extended attributes. (Internal use only)
const FILE_ATTRIBUTE_PINNED: u32 = 0x00080000;                  // This attribute indicates user intent that the file or directory should be kept fully present locally even when not being actively accessed. This attribute is for use with hierarchical storage management software.
const FILE_ATTRIBUTE_UNPINNED: u32 = 0x00100000;                // This attribute indicates that the file or directory should not be kept fully present locally except when being actively accessed. This attribute is for use with hierarchical storage management software.
const FILE_ATTRIBUTE_RECALL_ON_OPEN: u32 = 0x00040000;          // This attribute only appears in directory enumeration classes (FILE_DIRECTORY_INFORMATION, FILE_BOTH_DIR_INFORMATION, etc.). When this attribute is set, it means that the file or directory has no physical representation on the local system; the item is virtual. Opening the item will be more expensive than normal, e.g. it will cause at least some of it to be fetched from a remote store.
const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 0x00400000;   // When this attribute is set, it means that the file or directory is not fully present locally. For a file that means that not all of its data is on local storage (e.g. it may be sparse with some data still in remote storage). For a directory it means that some of the directory contents are being virtualized from another location.

fn main() {
    show(r"C:\inexistant"); // Does not exist
    show(r"C:\vfcompat.dll - Shortcut.lnk"); // Shell link, just seen as a plain file
    show(r"C:\vfcompat.dll - Hardlink"); // Shell link, just seen as a plain file
    show(r"C:\Backup"); // Simple directory
    show(r"C:\Downloads"); // Directory link
    show(r"C:\Development"); // Junction (IO_REPARSE_TAG_MOUNT_POINT)
    show(r"C:\Tmp"); // Junction (IO_REPARSE_TAG_MOUNT_POINT)
    show(r"\\teraz\temp"); // ?
    // show(r"\\cloro-02\Cloro\Images\Accessoires");   // Other: IO_REPARSE_TAG_MOUNT_POINT
    // show(r"\\cloro-02\Cloro\Images\Formation photo");   // Other: IO_REPARSE_TAG_MOUNT_POINT
    show(r"C:\hosts"); // IO_REPARSE_TAG_SYMLINK
    show(r"C:\Users\Pierr\.julia\packages\FilePathsBase\NV2We\docs\src\index.md");
    show(r"D:\Andrew.Black.IA\OneDrive\Revues\Auto, Moto\4x4 Magazine\4x4 Magazine HS 24 - 2014 - Spécial 150 essais.pdf") // STUB
}

fn show(filename: &str) {
    let prefix: &str;
    let path = Path::new(filename);
    if path.is_file() {
        prefix = "File";
        println!("File: {}", path.display());
    } else if path.is_dir() {
        prefix = "Dir";
        println!("Dir: {}", path.display());
    } else if path.is_symlink() {
        prefix = "Inexistent target";
        println!("Link: {}", path.display());
    } else if !path.exists() {
        println!("Does not exist: {}\n", path.display());
        return;
    } else {
        prefix = "Other";
        println!("Other: {:?}", path);
    }

    match reparse_type(path) {
        //Ok(kind) => println!("Reparse point type: {:?}  at1={:08X}  at2={:08X}", kind, attributes1(path),attributes2(path)),
        Ok((kind, detail)) => {
            println!("{} Reparse point type: {:?} {}", prefix, kind, detail);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!();
}

/// Values returned by reparse_type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum ReparseType {
    NO_REPARSE = 0,
    STUB = 1, // Not actually a reparse point, but acts like one...  For OneDrive stubs
    SYMLINK = 2,
    JUNCTION = 3,
    OTHER = 4,
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
        // OneDrive stubs have flag FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS = 0x00400000
        if attributes & FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS != 0 {
            return Ok((ReparseType::STUB, String::new()));
        }
        return Ok((ReparseType::NO_REPARSE, String::new())); // Not a reparse point
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
        IO_REPARSE_TAG_SYMLINK => Ok((ReparseType::SYMLINK, link_string)),
        IO_REPARSE_TAG_MOUNT_POINT => Ok((ReparseType::JUNCTION, link_string)),
        _ => Ok((ReparseType::OTHER, msg)),
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
            "Used by single-instance storage (SIS) filter driver. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000008 => (
            "IO_REPARSE_TAG_WIM",
            "Used by the WIM Mount filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000009 => (
            "IO_REPARSE_TAG_CSV",
            "Obsolete. Used by Clustered Shared Volumes (CSV) version 1 in Windows Server 2008 R2 operating system. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x8000000A => (
            "IO_REPARSE_TAG_DFS",
            "Used by the DFS filter. The DFS is described in the Distributed File System (DFS): Referral Protocol Specification [MS-DFSC]. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x8000000B => ("IO_REPARSE_TAG_FILTER_MANAGER", "Used by filter manager test harness.<4>"),
        0xA000000C => ("IO_REPARSE_TAG_SYMLINK", "Used for symbolic link support. See section 2.1.2.4."),
        0xA0000010 => (
            "IO_REPARSE_TAG_IIS_CACHE",
            "Used by Microsoft Internet Information Services (IIS) caching. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000012 => (
            "IO_REPARSE_TAG_DFSR",
            "Used by the DFS filter. The DFS is described in [MS-DFSC]. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000013 => (
            "IO_REPARSE_TAG_DEDUP",
            "Used by the Data Deduplication (Dedup) filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        0xC0000014 => ("IO_REPARSE_TAG_APPXSTRM", "Not used."),
        0x80000014 => (
            "IO_REPARSE_TAG_NFS",
            "Used by the Network File System (NFS) component. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000015 => (
            "IO_REPARSE_TAG_FILE_PLACEHOLDER",
            "Obsolete. Used by Windows Shell for legacy placeholder files in Windows 8.1. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000016 => (
            "IO_REPARSE_TAG_DFM",
            "Used by the Dynamic File filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000017 => (
            "IO_REPARSE_TAG_WOF",
            "Used by the Windows Overlay filter, for either WIMBoot or single-file compression. Server-side interpretation only, not meaningful over the wire. ",
        ),
        0x80000018 => (
            "IO_REPARSE_TAG_WCI",
            "Used by the Windows Container Isolation filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x90001018 => (
            "IO_REPARSE_TAG_WCI_1",
            "Used by the Windows Container Isolation filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        0xA0000019 => (
            "IO_REPARSE_TAG_GLOBAL_REPARSE",
            "Used by NPFS to indicate a named pipe symbolic link from a server silo into the host silo. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000001A => (
            "IO_REPARSE_TAG_CLOUD",
            "Used by the Cloud Files filter, for files managed by a sync engine such as Microsoft OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000101A => (
            "IO_REPARSE_TAG_CLOUD_1",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000201A => (
            "IO_REPARSE_TAG_CLOUD_2",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000301A => (
            "IO_REPARSE_TAG_CLOUD_3",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000401A => (
            "IO_REPARSE_TAG_CLOUD_4",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000501A => (
            "IO_REPARSE_TAG_CLOUD_5",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000601A => (
            "IO_REPARSE_TAG_CLOUD_6",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000701A => (
            "IO_REPARSE_TAG_CLOUD_7",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000801A => (
            "IO_REPARSE_TAG_CLOUD_8",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000901A => (
            "IO_REPARSE_TAG_CLOUD_9",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000A01A => (
            "IO_REPARSE_TAG_CLOUD_A",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000B01A => (
            "IO_REPARSE_TAG_CLOUD_B",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000C01A => (
            "IO_REPARSE_TAG_CLOUD_C",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000D01A => (
            "IO_REPARSE_TAG_CLOUD_D",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000E01A => (
            "IO_REPARSE_TAG_CLOUD_E",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000F01A => (
            "IO_REPARSE_TAG_CLOUD_F",
            "Used by the Cloud Files filter, for files managed by a sync engine such as OneDrive. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x8000001B => (
            "IO_REPARSE_TAG_APPEXECLINK",
            "Used by Universal Windows Platform (UWP) packages to encode information that allows the application to be launched by CreateProcess. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x9000001C => (
            "IO_REPARSE_TAG_PROJFS",
            "Used by the Windows Projected File System filter, for files managed by a user mode provider such as VFS for Git. Server-side interpretation only, not meaningful over the wire.",
        ),
        0xA000001D => (
            "IO_REPARSE_TAG_LX_SYMLINK",
            "Used by the Windows Subsystem for Linux (WSL) to represent a UNIX symbolic link. section 2.1.2.7.",
        ),
        0x8000001E => (
            "IO_REPARSE_TAG_STORAGE_SYNC",
            "Used by the Azure File Sync (AFS) filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x90000027 => (
            "IO_REPARSE_TAG_STORAGE_SYNC_FOLDER",
            "Used by the Azure File Sync (AFS) filter for folder. Server-side interpretation only, not meaningful over the wire.",
        ),
        0xA000001F => (
            "IO_REPARSE_TAG_WCI_TOMBSTONE",
            "Used by the Windows Container Isolation filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000020 => (
            "IO_REPARSE_TAG_UNHANDLED",
            "Used by the Windows Container Isolation filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000021 => ("IO_REPARSE_TAG_ONEDRIVE", "Not used."),
        0xA0000022 => (
            "IO_REPARSE_TAG_PROJFS_TOMBSTONE",
            "Used by the Windows Projected File System filter, for files managed by a user mode provider such as VFS for Git. Server-side interpretation only, not meaningful over the wire.",
        ),
        0x80000023 => (
            "IO_REPARSE_TAG_AF_UNIX",
            "Used to represent a UNIX domain socket. Server-side interpretation only, not meaningful over the wire. No defined structure.",
        ),
        0x80000024 => (
            "IO_REPARSE_TAG_LX_FIFO",
            "Used by the Windows Subsystem for Linux (WSL) to represent a UNIX FIFO (named pipe). Server-side interpretation only, not meaningful over the wire. No defined structure.",
        ),
        0x80000025 => (
            "IO_REPARSE_TAG_LX_CHR",
            "Used by the Windows Subsystem for Linux (WSL) to represent a UNIX character special file. Server-side interpretation only, not meaningful over the wire. No defined structure.",
        ),
        0x80000026 => (
            "IO_REPARSE_TAG_LX_BLK",
            "Used by the Windows Subsystem for Linux (WSL) to represent a UNIX block special file. Server-side interpretation only, not meaningful over the wire. No defined structure.",
        ),
        0xA0000027 => (
            "IO_REPARSE_TAG_WCI_LINK",
            "Used by the Windows Container Isolation filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        0xA0001027 => (
            "IO_REPARSE_TAG_WCI_LINK_1",
            "Used by the Windows Container Isolation filter. Server-side interpretation only, not meaningful over the wire.",
        ),
        _ => ("IO_REPARSE_TAG_UNKNOWN", "Unknown reparse tag."),
    }
}
