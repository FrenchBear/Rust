// PVMarkup library
// Parse and render my own markup language
//
// 2025-05-05   PV      First version

#![allow(unused_variables, dead_code, unused_imports)]

// I hope that MyGlob is not included in actual library transitive dependencies...
use colored as _;

// Std library imports
use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek};
use std::path::Path;

// external crates imports

// -----------------------------------
// Submodules

mod tests;

// -----------------------------------
// Globals

const LIB_VERSION: &str = "1.0.0";

// -----------------------------------
// Structures

#[derive(Debug)]
pub struct PVMarkup {
}

impl PVMarkup {
    pub fn version() -> &'static str {
        LIB_VERSION
    }

    pub fn print_text(txt: &str) {
        
    }
}
