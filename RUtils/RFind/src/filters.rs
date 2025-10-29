// filters.rs, definition of filters
//
// 2025-10-27	PV      First version

use super::*;

// Standard library imports

// ===============================================================
// Empty filter

#[derive(Debug)]
pub struct FilterEmpty {}

impl FilterEmpty {
    pub fn new() -> Self {
        FilterEmpty {}
    }
}

impl Filter for FilterEmpty {
    fn name(&self) -> &'static str {
        "Empty: Select empty files and directories"
    }

    fn filter(&mut self, lw: &mut LogWriter, path: &Path, _verbose: bool) -> bool {
        if path.is_file() {
            fs::metadata(path).unwrap().len() == 0
        } else if path.is_dir() {
            match fs::read_dir(path) {
                Ok(mut p) => p.next().is_none(),
                Err(_) => true,
            }
        } else if path.is_symlink() {
            // An link with invalid target is considered empty
            true
        } else {
            logln(lw, format!("*** Error neither dir not file {}", path.display()).as_str());
            false
        }
    }
}

// ===============================================================
// Alternate Data Streams filter

#[derive(Debug)]
pub struct FilterADS {
    ignore_small_streams: bool,
}

impl FilterADS {
    pub fn new(ignore_small_streams: bool) -> Self {
        FilterADS { ignore_small_streams }
    }
}

impl Filter for FilterADS {
    fn name(&self) -> &'static str {
        if self.ignore_small_streams {
            "adsx: Select files with alternate data streams other than Zone.identification"
        } else {
            "ads: Select files with alternate data streams"
        }
    }

    fn filter(&mut self, _lw: &mut LogWriter, path: &Path, _verbose: bool) -> bool {
        if path.is_file() {
            let streams = match fa_streams::get_streams_list(path, false) {
                Ok(s) => s,
                Err(_) => {
                    return false;
                }
            };

            // If we don't ignore small streams, any stream (other than main stream) is included
            if !self.ignore_small_streams {
                return !streams.is_empty();
            }

            // Only included if there is at least one stream of 2KB or more
            for stream in streams.iter() {
                if stream.size >= 2048 {
                    return true;
                }
            }
            false
        } else {
            // Doesn't filter anything else
            true
        }
    }
}
