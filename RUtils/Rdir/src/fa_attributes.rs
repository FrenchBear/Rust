// fa_attributes.rs - File analysis for attributes
//
// 2025-10-25   PV      First version

use std::{fs, os::windows::fs::MetadataExt, path::Path};

use crate::Options;

#[derive(Debug)]
pub struct AttributesInfo {
    // Standard attributes
    pub normal: bool,
    pub archive: bool,
    pub readonly: bool,
    pub hidden: bool,
    pub system: bool,
    pub directory: bool,
    pub tempoary: bool,
    pub sparse_file: bool,
    pub reparse_point: bool,
    pub compressed: bool,
    pub offline: bool,
    pub not_content_indexed: bool,
    pub encrypted: bool,
    pub integrity_stream: bool,
    pub isvirtual: bool,
    pub no_scrub_data: bool,
    pub pinned: bool,
    pub unpinned: bool,
    pub recall_on_open: bool,
    pub recall_on_data_access: bool,
}

// Windows file attributes
// https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
const FILE_ATTRIBUTE_READONLY: u32 = 0x00000001; // A file that is read-only. Applications can read the file, but cannot write to it or delete it. This attribute is not honored on directories.
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002; // The file or directory is hidden. It is not included in an ordinary directory listing.
const FILE_ATTRIBUTE_SYSTEM: u32 = 0x00000004; // A file or directory that the operating system uses a part of, or uses exclusively.
const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x00000010; // The handle that identifies a directory.
const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x00000020; // A file or directory that is an archive file or directory. Applications typically use this attribute to mark files for backup or removal.
//const FILE_ATTRIBUTE_DEVICE: u32 = 0x00000040; // This value is reserved for system use.
const FILE_ATTRIBUTE_NORMAL: u32 = 0x00000080; // A file that does not have other attributes set. This attribute is valid only when used alone.
const FILE_ATTRIBUTE_TEMPORARY: u32 = 0x00000100; // A file that is being used for temporary storage. File systems avoid writing data back to mass storage if sufficient cache memory is available, because typically, an application deletes a temporary file after the handle is closed. In that scenario, the system can entirely avoid writing the data. Otherwise, the data is written after the handle is closed.
const FILE_ATTRIBUTE_SPARSE_FILE: u32 = 0x00000200; // A file that is a sparse file.
const FILE_ATTRIBUTE_REPARSE_POINT_TYPE: u32 = 0x00000400; // A file or directory that has an associated reparse point, or a file that is a symbolic link.
const FILE_ATTRIBUTE_COMPRESSED: u32 = 0x00000800; // A file or directory that is compressed. For a file, all of the data in the file is compressed. For a directory, compression is the default for newly created files and subdirectories.
const FILE_ATTRIBUTE_OFFLINE: u32 = 0x00001000; // The data of a file is not available immediately. This attribute indicates that the file data is physically moved to offline storage. This attribute is used by Remote Storage, which is the hierarchical storage management software. Applications should not arbitrarily change this attribute.
const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: u32 = 0x00002000; // The file or directory is not to be indexed by the content indexing service.
const FILE_ATTRIBUTE_ENCRYPTED: u32 = 0x00004000; // A file or directory that is encrypted. For a file, all data streams in the file are encrypted. For a directory, encryption is the default for newly created files and subdirectories.
const FILE_ATTRIBUTE_INTEGRITY_STREAM: u32 = 0x00008000; // The directory or user data stream is configured with integrity (only supported on ReFS volumes). It is not included in an ordinary directory listing. The integrity setting persists with the file if it's renamed. If a file is copied the destination file will have integrity set if either the source file or destination directory have integrity set.
const FILE_ATTRIBUTE_VIRTUAL: u32 = 0x00010000; // This value is reserved for system use.
const FILE_ATTRIBUTE_NO_SCRUB_DATA: u32 = 0x00020000; // The user data stream not to be read by the background data integrity scanner (AKA scrubber). When set on a directory it only provides inheritance. This flag is only supported on Storage Spaces and ReFS volumes. It is not included in an ordinary directory listing.
//const FILE_ATTRIBUTE_EA: u32 = 0x00040000; // A file or directory with extended attributes. (Internal use only)
const FILE_ATTRIBUTE_PINNED: u32 = 0x00080000; // This attribute indicates user intent that the file or directory should be kept fully present locally even when not being actively accessed. This attribute is for use with hierarchical storage management software.
const FILE_ATTRIBUTE_UNPINNED: u32 = 0x00100000; // This attribute indicates that the file or directory should not be kept fully present locally except when being actively accessed. This attribute is for use with hierarchical storage management software.
const FILE_ATTRIBUTE_RECALL_ON_OPEN: u32 = 0x00040000; // This attribute only appears in directory enumeration classes (FILE_DIRECTORY_INFORMATION, FILE_BOTH_DIR_INFORMATION, etc.). When this attribute is set, it means that the file or directory has no physical representation on the local system; the item is virtual. Opening the item will be more expensive than normal, e.g. it will cause at least some of it to be fetched from a remote store.
const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 0x00400000; // When this attribute is set, it means that the file or directory is not fully present locally. For a file that means that not all of its data is on local storage (e.g. it may be sparse with some data still in remote storage). For a directory it means that some of the directory contents are being virtualized from another location.

pub fn get_attributes_information(path: &Path, options: &Options) -> Result<AttributesInfo, String> {
    if !path.is_dir() && !path.is_file() && !path.is_symlink() {
        return Err(format!("{}: Not found", path.display()));
    }
    
    let meta_res = if path.is_symlink() && !options.show_link_target_info 
    {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };

    let meta = match meta_res {
        Ok(m) => m,
        Err(e) => return Err(e.to_string()),
    };

    let at = meta.file_attributes();
    let ai = AttributesInfo {
        normal: at & FILE_ATTRIBUTE_NORMAL != 0,
        readonly: at & FILE_ATTRIBUTE_READONLY != 0,
        hidden: at & FILE_ATTRIBUTE_HIDDEN != 0,
        system: at & FILE_ATTRIBUTE_SYSTEM != 0,
        directory: at & FILE_ATTRIBUTE_DIRECTORY != 0,
        archive: at & FILE_ATTRIBUTE_ARCHIVE != 0,
        tempoary: at & FILE_ATTRIBUTE_TEMPORARY != 0,
        sparse_file: at & FILE_ATTRIBUTE_SPARSE_FILE != 0,
        reparse_point: at & FILE_ATTRIBUTE_REPARSE_POINT_TYPE != 0,
        compressed: at & FILE_ATTRIBUTE_COMPRESSED != 0,
        offline: at & FILE_ATTRIBUTE_OFFLINE != 0,
        not_content_indexed: at & FILE_ATTRIBUTE_NOT_CONTENT_INDEXED != 0,
        encrypted: at & FILE_ATTRIBUTE_ENCRYPTED != 0,
        integrity_stream: at & FILE_ATTRIBUTE_INTEGRITY_STREAM != 0,
        isvirtual: at & FILE_ATTRIBUTE_VIRTUAL != 0,
        no_scrub_data: at & FILE_ATTRIBUTE_NO_SCRUB_DATA != 0,
        pinned: at & FILE_ATTRIBUTE_PINNED != 0,
        unpinned: at & FILE_ATTRIBUTE_UNPINNED != 0,
        recall_on_open: at & FILE_ATTRIBUTE_RECALL_ON_OPEN != 0,
        recall_on_data_access: at & FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS !=0,
    };

    Ok(ai)
}
