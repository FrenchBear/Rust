// fa_dates.rs - File analysis for dates
//
// 2025-10-25   PV      First version

// Standard library imports
use std::{fs, path::Path};

// External imports
use chrono::{DateTime, Local, Utc};

// Crate imports
use crate::Options;

// ---------------------------------

#[derive(Debug)]
pub struct DatesInfo {
    pub created_utc: DateTime<Utc>,
    pub modified_utc: DateTime<Utc>,
    pub accessed_utc: DateTime<Utc>,

    pub created_local: DateTime<Local>,
    pub modified_local: DateTime<Local>,
    pub accessed_local: DateTime<Local>,
}

pub fn get_dates_information(path: &Path, options: &Options) -> Result<DatesInfo, String> {
    if !path.is_dir() && !path.is_file() && !path.is_symlink(){
        return Err(format!("{}: Not found", path.display()));
    }

    let meta_res = if path.is_symlink() && !options.show_link_target_info {
        fs::symlink_metadata(path)
    } else {
        fs::metadata(path)
    };

    let meta = match meta_res {
        Ok(m) => m,
        Err(e) => return Err(e.to_string()),
    };

    let created = meta.created().unwrap(); // Get last created time
    let created_utc: DateTime<Utc> = DateTime::<Utc>::from(created);
    let created_local = created_utc.with_timezone(&Local);

    let modified = meta.modified().unwrap(); // Get last modified time
    let modified_utc: DateTime<Utc> = DateTime::<Utc>::from(modified);
    let modified_local = modified_utc.with_timezone(&Local);

    let accessed = meta.accessed().unwrap(); // Get last accessed time
    let accessed_utc: DateTime<Utc> = DateTime::<Utc>::from(accessed);
    let accessed_local = accessed_utc.with_timezone(&Local);

    Ok(DatesInfo {
        created_utc,
        modified_utc,
        accessed_utc,
        created_local,
        modified_local,
        accessed_local,
    })
}
