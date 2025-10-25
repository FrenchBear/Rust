// fa_name.rs - File analysis for name
//
// 2025-10-25   PV      First version

use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use windows::{
    core::{w, PCWSTR, PWSTR},
    Win32::UI::Shell::{AssocQueryStringW, ASSOCF_NONE, ASSOCSTR, ASSOCSTR_FRIENDLYAPPNAME, ASSOCSTR_FRIENDLYDOCNAME},
};

use crate::Options;

#[derive(Debug)]
pub struct NamesInfo {
    pub filename: String,
    pub original_with_path: String,
    pub canonical_fullpath: String,

    pub file_type_description: Option<String>,
    pub opens_with: Option<String>,
}

pub fn get_names_information(path: &Path, options: &Options) -> Result<NamesInfo, String> {
    if !path.exists() {
        return Err(format!("{}: Not found", path.display()));
    }

    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    let original_with_path = path.to_string_lossy().replace(r"\\?\", "");
    let canonical_fullpath = path.canonicalize().unwrap().to_string_lossy().replace(r"\\?\", "");

    let (file_type_description, opens_with) = match path.extension() {
        Some(ext) => (query_assoc_string(ASSOCSTR_FRIENDLYDOCNAME, ext), query_assoc_string(ASSOCSTR_FRIENDLYAPPNAME, ext)),
        None => (None, None)
    };

    Ok(NamesInfo{filename, original_with_path, canonical_fullpath, file_type_description, opens_with})
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

        if cch == 0 { return None; }

        let mut buffer: Vec<u16> = vec![0; cch as usize];
        let buffer_pwstr = PWSTR(buffer.as_mut_ptr());
        // Second call to get the actual string.
        AssocQueryStringW(ASSOCF_NONE, assoc_str, assoc_pcwstr, None, Some(buffer_pwstr), &mut cch);

        String::from_utf16(&buffer[..cch as usize - 1]).ok()
    }
}
