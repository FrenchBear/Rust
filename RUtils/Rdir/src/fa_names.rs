// fa_name.rs - File analysis for name
//
// 2025-10-25   PV      First version
// 2025-10-27   PV      Process correctly \\?\UNC\terazalt\books\...

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{fs, io};
use windows::{
    Win32::UI::Shell::{ASSOCF_NONE, ASSOCSTR, ASSOCSTR_FRIENDLYAPPNAME, ASSOCSTR_FRIENDLYDOCNAME, AssocQueryStringW},
    core::{PCWSTR, PWSTR, w},
};

use crate::Options;

#[derive(Debug)]
pub struct NamesInfo {
    pub filename: String,
    pub parent: String,
    pub original_with_path: String,
    pub canonical_fullpath: String,

    pub file_type_description: Option<String>,
    pub opens_with: Option<String>,
}

pub fn get_names_information(path: &Path, options: &Options) -> Result<NamesInfo, String> {
    // if !path.exists() {
    //     return Err(format!("{}: Not found", path.display()));
    // }

    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    let parent = path.parent().unwrap_or_else(|| Path::new(".")).to_str().unwrap().to_string();

    let (original_with_path, canonical_fullpath) = if path.is_symlink() {
        let can = canonicalize_link(path).unwrap();
        (prp(&can), prp(&can))
    } else {
        (prp(path), prp(&path.canonicalize().unwrap()))
    };

    let (file_type_description, opens_with) = match path.extension() {
        Some(ext) => (
            query_assoc_string(ASSOCSTR_FRIENDLYDOCNAME, ext),
            query_assoc_string(ASSOCSTR_FRIENDLYAPPNAME, ext),
        ),
        None => (None, None),
    };

    Ok(NamesInfo {
        filename,
        parent,
        original_with_path,
        canonical_fullpath,
        file_type_description,
        opens_with,
    })
}

fn prp(s: &Path) -> String {
    s.to_string_lossy().replace(r"\\?\UNC", r"\").replace(r"\\?\", "")
}

/// A helper function to call the `AssocQueryStringW` Windows API function.
fn query_assoc_string(assoc_str: ASSOCSTR, ext: &OsStr) -> Option<String> {
    // Prepend a dot to the extension for the query.
    let assoc_str_with_dot = format!(".{}", ext.to_string_lossy());
    let mut wide_assoc: Vec<u16> = assoc_str_with_dot.encode_utf16().chain(std::iter::once(0)).collect();
    let assoc_pcwstr = PCWSTR(wide_assoc.as_ptr());

    unsafe {
        let mut cch: u32 = 0;

        // First call to get the required buffer size.
        // We check the result to handle errors, like an unknown extension.
        AssocQueryStringW(ASSOCF_NONE, assoc_str, assoc_pcwstr, None, None, &mut cch);

        if cch == 0 {
            return None;
        }

        let mut buffer: Vec<u16> = vec![0; cch as usize];
        let buffer_pwstr = PWSTR(buffer.as_mut_ptr());
        // Second call to get the actual string.
        AssocQueryStringW(ASSOCF_NONE, assoc_str, assoc_pcwstr, None, Some(buffer_pwstr), &mut cch);

        String::from_utf16(&buffer[..cch as usize - 1]).ok()
    }
}

/// Gets the canonical path of a symlink file itself,
/// without resolving the link.
pub fn canonicalize_link(path: &Path) -> io::Result<PathBuf> {
    // 1. Get the parent directory of the path.
    // If the path has no parent (e.g., it's just "my_link"),
    // use "." (the current directory) as the parent.
    let parent = path.parent().unwrap_or_else(|| Path::new("."));

    // 2. Get the filename of the path.
    // This will return an error if the path ends in ".." or ".".
    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path has no filename"))?;

    // 3. Canonicalize the parent path.
    // This resolves all symlinks, `..`, and `.` components
    // *leading up to* the link.
    let canonical_parent = parent.canonicalize()?;

    // 4. Join the canonical parent with the link's filename.
    let canonical_link_path = canonical_parent.join(file_name);

    Ok(canonical_link_path)
}
