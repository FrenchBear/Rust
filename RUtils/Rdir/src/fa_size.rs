// fa_size.rs - File analysis for size
//
// 2025-10-25   PV      First version

use std::{fs, path::Path};

use crate::Options;

#[derive(Debug)]
pub struct SizeInfo {
    pub size: u64,
}

pub fn get_size_information(path: &Path, options: &Options) -> Result<SizeInfo, String> {
    if !path.exists() {
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

    // This should't work for directories
    Ok(SizeInfo{size: meta.len()})
}
