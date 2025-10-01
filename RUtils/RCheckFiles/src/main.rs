// rcheckfiles: Detect and optionally fix incorrect filenames
//
// 2025-03-23	PV      First version
// 2025-03-25	PV      1.1 Simplified code, less calls to meta(), about twice faster
// 2025-03-25	PV      1.2 Use DirEntry::file_type() to check whether entry is a dir or a file 3 times faster than path.is_file()/is_dir() !!!
// 2025-03-28	PV      1.2.1 Handle gracefully errors about inexistent dirs such as \\teraz\videos rather than panicking. No error for network root (no basename)
// 2025-03-29	PV      1.2.2 Renamed rcheckfiles
// 2025-04-03	PV      1.3.0 Code reorganization, module logging
// 2025-04-08	PV      1.4.0 Check brackets (incl. unit tests)
// 2025-05-05   PV      1.4.2 Use MyMarkup crate to format usage
// 2025-05-05	PV      1.4.3 Logging crate
// 2025-09-26	PV      2.0.0 Option -y for yaml output, option -F <file> to apply chages from a yaml file
// 2025-10-01	PV      2.1.0 Option -e to count extensions

// Standard library imports
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::os::windows::prelude::*;
use std::path::Path;
use std::process;
use std::time::Instant;

// External crates imports
use getopt::Opt;
use logging::{LogWriter, log, logln};
use mymarkup::MyMarkup;
use serde::Deserialize;
use unicode_normalization::{UnicodeNormalization, is_nfc};

// -----------------------------------
// Submodules

pub mod tests;

// -----------------------------------
// Globals

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const SPECIAL_CHARS: &str = "€®™©–—…×·•∶⧹⧸／⚹†‽¿🎜🎝♫♪“”⚡♥";

// Confusables for space
const CONF_SPC: [char; 13] = [
    '\u{2000}', // U+2000	EN QUAD
    '\u{2001}', // U+2001	EM QUAD
    '\u{2002}', // U+2002	EN SPACE
    '\u{2003}', // U+2003	EM SPACE
    '\u{2004}', // U+2004	THREE-PER-EM SPACE
    '\u{2005}', // U+2005	FOUR-PER-EM SPACE
    '\u{2006}', // U+2006	SIX-PER-EM SPACE
    '\u{2007}', // U+2007	FIGURE SPACE
    '\u{2008}', // U+2008	PUNCTUATION SPACE
    '\u{2009}', // U+2009	THIN SPACE
    '\u{200A}', // U+200A	HAIR SPACE
    '\u{202F}', // U+202F	NARROW NO-BREAK SPACE
    '\u{205F}', // U+205F	MEDIUM MATHEMATICAL SPACE
];

// Confusables for apostrophe
const CONF_APO: [char; 33] = [
    '\u{00B4}', // ´ U+00B4	ACUTE ACCENT
    '\u{02B9}', // ʹ U+02B9	MODIFIER LETTER PRIME
    '\u{02BB}', // ʻ U+02BB	MODIFIER LETTER TURNED COMMA
    '\u{02BC}', // ʼ U+02BC	MODIFIER LETTER APOSTROPHE
    '\u{02BD}', // ʽ U+02BD	MODIFIER LETTER REVERSED COMMA
    '\u{02BE}', // ʾ U+02BE	MODIFIER LETTER RIGHT HALF RING
    '\u{02C8}', // ˈ U+02C8	MODIFIER LETTER VERTICAL LINE
    '\u{02CA}', // ˊ U+02CA	MODIFIER LETTER ACUTE ACCENT
    '\u{02CB}', // ˋ U+02CB	MODIFIER LETTER GRAVE ACCENT
    '\u{02F4}', // ˴ U+02F4	MODIFIER LETTER MIDDLE GRAVE ACCENT
    '\u{0374}', // ʹ U+0374	GREEK NUMERAL SIGN
    '\u{0384}', // ΄ U+0384	GREEK TONOS
    '\u{055A}', // ՚ U+055A	ARMENIAN APOSTROPHE
    '\u{055D}', // ՝ U+055D	ARMENIAN COMMA
    '\u{05D9}', // י U+05D9	HEBREW LETTER YOD
    '\u{05F3}', // ׳ U+05F3	HEBREW PUNCTUATION GERESH
    '\u{07F4}', // ߴ U+07F4	NKO HIGH TONE APOSTROPHE
    '\u{07F5}', // ߵ U+07F5	NKO LOW TONE APOSTROPHE
    '\u{144A}', // ᑊ U+144A	CANADIAN SYLLABICS WEST-CREE P
    '\u{16CC}', // ᛌ U+16CC	RUNIC LETTER SHORT-TWIG-SOL S
    '\u{1FBD}', // ᾽ U+1FBD	GREEK KORONIS
    '\u{1FBF}', // ᾿ U+1FBF	GREEK PSILI
    '\u{1FEF}', // ` U+1FEF	GREEK VARIA
    '\u{1FFD}', // ´ U+1FFD	GREEK OXIA
    '\u{1FFE}', // ῾ U+1FFE	GREEK DASIA
    '\u{2018}', // ‘ U+2018	LEFT SINGLE QUOTATION MARK
    '\u{2019}', // ’ U+2019	RIGHT SINGLE QUOTATION MARK
    '\u{201B}', // ‛ U+201B	SINGLE HIGH-REVERSED-9 QUOTATION MARK
    '\u{2032}', // ′ U+2032	PRIME
    '\u{2035}', // ‵ U+2035	REVERSED PRIME
    '\u{A78C}', // ꞌ U+A78C	LATIN SMALL LETTER SALTILLO
    '\u{FF07}', // ＇ U+FF07	FULLWIDTH APOSTROPHE
    '\u{FF40}', // ｀ U+FF40	FULLWIDTH GRAVE ACCENT
];

// ==============================================================================================
// Structures for deserialization of yaml file

// Use an enum to represent the 'typ' field for type safety.
// `serde(rename_all = "lowercase")` tells serde to match 'dir' with Dir and 'file' with File.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ItemType {
    Dir,
    File,
}

// This struct represents a single entry in the YAML list.
// The field names match the keys in the YAML file.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RenameItem {
    typ: ItemType,
    prb: String, // The 'prb' field is parsed but we won't use it
    old: String,
    new: String,
}

// ==============================================================================================
// Options processing

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct Options {
    sources: Vec<String>,
    fixit: bool,
    yaml_output: bool,
    yaml_file: String,
    count_extensions: bool,
}

/// Checks if a path exists and is a file.
/// Returns `true` only if the path points to an existing file.
/// Returns `false` for directories, symlinks, or if the path doesn't exist.
fn file_exists(path: &str) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_file()).unwrap_or(false)
}

/// Checks if a path exists and is a directory.
/// Returns `true` only if the path points to an existing directory.
/// Returns `false` for files, symlinks, or if the path doesn't exist.
fn dir_exists(path: &str) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_dir()).unwrap_or(false)
}

impl Options {
    fn header() {
        println!(
            "{APP_NAME} {APP_VERSION}\n\
             {APP_DESCRIPTION}"
        );
    }

    fn usage() {
        Options::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} ¬[⦃?⦄|⦃-?⦄|⦃-h⦄] [⦃-f⦄] [⦃-y⦄] [⦃-F⦄ ⟨yamlfile⟩] [⦃-e⦄] ⟨source⟩...
⦃?⦄|⦃-?⦄|⦃-h⦄     ¬Show this message
⦃-f⦄          ¬Automatic problems fixing
⦃-y⦄          ¬Yaml output
⦃-F⦄ ⟨yamlfile⟩ ¬Rename files using old/new fields of provided yaml file
⦃-e⦄          ¬Count extensions
⟨source⟩      ¬File or directory to analyze (note: glob pattern is not supported)";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    fn new() -> Result<Options, Box<dyn Error>> {
        let mut args: Vec<String> = std::env::args().collect();
        if args.len() > 1 && args[1].to_lowercase() == "help" {
            Self::usage();
            return Err("".into());
        }

        let mut options = Options { ..Default::default() };
        let mut opts = getopt::Parser::new(&args, "h?fyF:e");

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => {
                        Self::usage();
                        return Err("".into());
                    }

                    Opt('f', None) => {
                        options.fixit = true;
                    }

                    Opt('e', None) => {
                        options.count_extensions = true;
                    }

                    Opt('y', None) => {
                        options.yaml_output = true;
                    }

                    Opt('F', yamlfile) => {
                        if yamlfile.is_none() {
                            return Err("Option -f requires about yaml file as an argument".into());
                        }
                        options.yaml_file = yamlfile.unwrap();
                        if !file_exists(&options.yaml_file) {
                            return Err(format!("Can't find yaml file {}", options.yaml_file).into());
                        }
                    }

                    _ => unreachable!(),
                },
            }
        }

        // Check for extra argument
        for arg in args.split_off(opts.index()) {
            if arg == "?" || arg == "help" {
                Self::usage();
                return Err("".into());
            }

            if arg.starts_with("-") {
                return Err(format!("Invalid/unsupported option {}", arg).into());
            }

            options.sources.push(arg);
        }

        // Validate options
        let count_true = (options.yaml_output as u8) + (options.fixit as u8) + (!options.yaml_file.is_empty() as u8);
        if count_true > 1 {
            return Err("Options -y, -f and -F are exclusive".into());
        }
        if options.count_extensions && !options.yaml_file.is_empty() {
            return Err("Options -F and -e are exclusive".into());
        }

        if options.yaml_file.is_empty() {
            if options.sources.is_empty() {
                return Err("Without option -F, at least one source is required".into());
            }
        } else {
            if !options.sources.is_empty() {
                return Err("With option -F, no source is allowed".into());
            }
        }

        Ok(options)
    }
}

// -----------------------------------
// Main

#[derive(Default)]
struct Statistics {
    total: i32,                        // Total files/dirs processed
    nnn: i32,                          // Non-normalized names
    bra: i32,                          // Bracket issue
    apo: i32,                          // Incorrect apostrophe
    spc: i32,                          // Incorrect space
    car: i32,                          // Maybe incorrect char
    sp2: i32,                          // Double space
    fix: i32,                          // Number of path fixed
    err: i32,                          // Number of errors
    ext_counter: HashMap<String, u32>, // Count of extensions (lowercase)
}

impl Statistics {}

struct Confusables {
    space: HashSet<char>,
    apostrophe: HashSet<char>,
}

fn main() {
    // Process options
    let options = Options::new().unwrap_or_else(|err| {
        let msg = format!("{}", err);
        if msg.is_empty() {
            process::exit(0);
        }
        eprintln!("{APP_NAME}: Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let confusables = Confusables {
        space: HashSet::from_iter(CONF_SPC),
        apostrophe: HashSet::from_iter(CONF_APO),
    };

    // Prepare log writer
    let mut writer = logging::new(APP_NAME, APP_VERSION, true);

    let start = Instant::now();

    if options.yaml_file.is_empty() {
        let mut files_stats = Statistics { ..Default::default() };
        let mut dirs_stats = Statistics { ..Default::default() };

        for source in &options.sources {
            logln(&mut writer, &format!("Analyzing {}", source));
            let p = Path::new(&source);
            if p.is_file() {
                process_file(p, &mut files_stats, &options, &mut writer, &confusables);
            } else {
                process_directory(p, &mut dirs_stats, &mut files_stats, &options, &mut writer, &confusables);
            }
        }

        let duration = start.elapsed();

        fn s(n: i32) -> &'static str {
            if n > 1 { "s" } else { "" }
        }

        fn final_status(writer: &mut LogWriter, stats: &Statistics, typename: &str) {
            log(writer, &format!("{} {}{} checked", stats.total, typename, s(stats.total)));
            if stats.nnn > 0 {
                log(writer, &format!(", {} non-normalized", stats.nnn));
            }
            if stats.bra > 0 {
                log(writer, &format!(", {} brackets issue{}", stats.bra, s(stats.bra)));
            }
            if stats.apo > 0 {
                log(writer, &format!(", {} wrong apostrophe", stats.apo));
            }
            if stats.spc > 0 {
                log(writer, &format!(", {} wrong space", stats.spc));
            }
            if stats.sp2 > 0 {
                log(writer, &format!(", {} multiple space", stats.sp2));
            }
            if stats.car > 0 {
                log(writer, &format!(", {} wrong character", stats.car));
            }
            if stats.fix > 0 {
                log(writer, &format!(", {} problem{} fixed", stats.fix, s(stats.fix)));
            }
            if stats.err > 0 {
                log(writer, &format!(", {} error{}", stats.err, s(stats.err)));
            }
            logln(writer, "");
        }

        logln(&mut writer, "");
        final_status(&mut writer, &dirs_stats, "dir");
        final_status(&mut writer, &files_stats, "file");

        if options.count_extensions {
            // Print extensions counter by decreasing order of count
            let mut extensions: Vec<_> = files_stats.ext_counter.iter().collect();
            extensions.sort_by(|a, b| b.1.cmp(a.1));    

            logln(&mut writer, "Extensions:");
            for (ext, cnt) in extensions {
                logln(&mut writer, format!("  {ext}: {cnt}").as_str());
            }
            logln(&mut writer, "");
        }

        logln(&mut writer, &format!("Total duration: {:.3}s", duration.as_secs_f64()));
    } else {
        let res = process_yaml_file(&mut writer, &options);

        match res {
            Ok(_) => {}
            Err(e) => {
                logln(&mut writer, &format!("Error processing yaml file: {}", e));
            }
        }

        let duration = start.elapsed();
        logln(&mut writer, &format!("Total duration: {:.3}s", duration.as_secs_f64()));
    }
}

fn process_yaml_file(writer: &mut LogWriter, options: &Options) -> Result<(), Box<dyn Error>> {
    let yaml_content = fs::read_to_string(&options.yaml_file)?;

    // Deserialize the YAML string into a vector of `RenameItem` structs.
    // `serde_yaml` will return an error if the format is incorrect.
    let items: Vec<RenameItem> = serde_yaml::from_str(&yaml_content)?;

    for item in items {
        if item.old == item.new {
            logln(writer, format!("old==new, for file «{}»", item.old).as_str());
            continue;
        }

        match item.typ {
            ItemType::Dir => {
                if !dir_exists(item.old.as_str()) {
                    // If already renamed, don(t complain
                    if !dir_exists(item.new.as_str()) {
                        logln(writer, format!("*** Can't find dir to rename «{}»", item.old).as_str());
                    }
                    continue;
                }
                match fs::rename(item.old.as_str(), item.new.as_str()) {
                    Err(e) => {
                        logln(
                            writer,
                            format!("*** Renaming dir «{}» into «{}» caused error: {}", item.old, item.new, e).as_str(),
                        );
                    }
                    Ok(_) => {
                        logln(writer, format!("Success renaming dir «{}» into «{}»", item.old, item.new).as_str());
                    }
                }
            }
            ItemType::File => {
                if !file_exists(item.old.as_str()) {
                    // If already renamed, don(t complain
                    if !file_exists(item.new.as_str()) {
                        logln(writer, format!("*** Can't find file to rename «{}»", item.old).as_str());
                    }
                    continue;
                }
                match fs::rename(item.old.as_str(), item.new.as_str()) {
                    Err(e) => {
                        logln(
                            writer,
                            format!("*** Renaming file «{}» into «{}» caused error: {}", item.old, item.new, e).as_str(),
                        );
                    }
                    Ok(_) => {
                        logln(writer, format!("Success renaming file «{}» into «{}»", item.old, item.new).as_str());
                    }
                }
            }
        };
    }

    Ok(())
}

fn process_directory(
    pa: &Path,
    dirs_stats: &mut Statistics,
    files_stats: &mut Statistics,
    options: &Options,
    writer: &mut LogWriter,
    pconfusables: &Confusables,
) {
    let mut pb = pa.to_path_buf();

    // Silently ignore hidden or system directories
    let resattributes = pa.metadata();
    match resattributes {
        Ok(md) => {
            let attributes = md.file_attributes();
            if (attributes & 0x2/* Hidden */) > 0 || (attributes & 0x4/* System */) > 0 {
                return;
            }
        }
        Err(e) => {
            logln(writer, &format!("*** Error {e}")); // Rename failed, but we continue anyway, don't really know if it's Ok or not...
            return;
        }
    }

    // First check directoru basename
    dirs_stats.total += 1;
    if let Some(new_name) = check_basename(pa, "dir", dirs_stats, options, writer, pconfusables) {
        if options.fixit {
            logln(writer, &format!("  --> rename directory \"{new_name}\""));
            let newpath = pb.parent().unwrap().join(Path::new(&new_name));
            match fs::rename(&pb, &newpath) {
                Ok(_) => {
                    dirs_stats.fix += 1;
                    pb = newpath;
                }
                Err(e) => logln(writer, &format!("*** Error {e}")), // Rename failed, but we continue anyway, don't really know if it's Ok or not...
            }
        }
    }

    // Then process directory content
    let contents = fs::read_dir(&pb);
    if contents.is_err() {
        logln(writer, &format!("*** Error enumerating directory {}: {:?}", pb.display(), contents.err()));
        return;
    }
    for entry in contents.unwrap() {
        if entry.is_err() {
            logln(writer, &format!("*** Error accessing directory entry: {:?}", entry.err()));
            continue;
        }
        let entry = entry.unwrap();
        let pb = entry.path();
        let ft = entry.file_type().unwrap();
        if ft.is_file() {
            process_file(&pb, files_stats, options, writer, pconfusables);
        } else if ft.is_dir() {
            process_directory(&pb, dirs_stats, files_stats, options, writer, pconfusables);
        }
    }
}

fn process_file(p: &Path, files_stats: &mut Statistics, options: &Options, writer: &mut LogWriter, pconfusables: &Confusables) {
    files_stats.total += 1;

    // Count extension
    if options.count_extensions {
        let ext = match p.extension() {
            Some(ext) => ext.to_str().unwrap().to_lowercase(),
            None => "(none)".to_string(),
        };
        let e = files_stats.ext_counter.entry(ext).or_insert(0);
        *e += 1;
    }

    if let Some(new_name) = check_basename(p, "file", files_stats, options, writer, pconfusables) {
        if options.fixit {
            logln(writer, &format!("  --> rename file \"{new_name}\""));
            let newpath = p.parent().unwrap().join(Path::new(&new_name));
            match fs::rename(p, &newpath) {
                Ok(_) => files_stats.fix += 1,
                Err(e) => logln(writer, &format!("*** Error {e}")), // Rename failed
            }
        }
    }
}

fn check_basename(
    p: &Path,
    pt: &str,
    stats: &mut Statistics,
    options: &Options,
    writer: &mut LogWriter,
    pconfusables: &Confusables,
) -> Option<String> {
    let fp = p.display();
    let file = p.file_name();
    file?; // file is None with network paths such as \\teraz\photo, that's normal, return None

    let file = file.unwrap().to_str();
    if file.is_none() {
        stats.err += 1;
        logln(writer, &format!("*** Invalid {pt} name {fp}, ignored"));
        return None;
    }

    let mut file = file.unwrap().to_string();
    let original_file = file.clone();
    let mut problems = String::new();

    fn add_problem(problems: &mut String, problem: &str) {
        if !problems.is_empty() {
            problems.push_str(", ");
        }
        problems.push_str(problem);
    }

    // Check for balanced brackets, but don't attempt a correction
    if !is_balanced(&file) {
        if options.yaml_output {
            add_problem(&mut problems, "Non-balanced brackets");
        } else {
            logln(writer, &format!("Non-balanced brackets {pt} name {fp}"));
        }
        stats.bra += 1;
    }

    // Check normalization
    if !is_nfc(&file) {
        if options.yaml_output {
            add_problem(&mut problems, "Non-normalized");
        } else {
            logln(writer, &format!("Non-normalized {pt} name {fp}"));
        }
        stats.nnn += 1;
        // Normalize it for the rest to avoid complaining on combining accents as invalid characters
        file = file.nfc().collect();
    }

    let mut vc: Vec<char> = file.chars().collect();

    // Check apostrophes
    let mut pbapo = false;
    for c in &mut vc {
        if pconfusables.apostrophe.contains(c) {
            if options.yaml_output {
                if !pbapo {
                    add_problem(&mut problems, "Invalid apostrophe");
                }
            } else {
                logln(writer, &format!("Invalid apostrophe in {pt} name {fp} -> {c} {:04X}", *c as i32));
            }
            if !pbapo {
                pbapo = true;
                stats.apo += 1;
            }
            *c = '\'';
        }
    }

    // Check spaces
    let mut pbspc = false;
    for c in &mut vc {
        //if CONF_SPC.contains(c) {
        if pconfusables.space.contains(c) {
            if options.yaml_output {
                if !pbspc {
                    add_problem(&mut problems, "Invalid space");
                }
            } else {
                logln(writer, &format!("Invalid space in {pt} name {fp} -> {:04X}", *c as i32));
            }
            if !pbspc {
                pbspc = true;
                stats.spc += 1;
            }
            *c = ' ';
        }
    }

    if pbapo || pbspc {
        file = vc.into_iter().collect();
    }

    // Check multiple spaces (and space before extension)
    let mut pbsp2 = false;
    let mut vc: Vec<char> = Vec::new();
    let mut lastc = '_';
    for c in file.chars() {
        if c == ' ' {
            if lastc == ' ' {
                if !pbsp2 {
                    if options.yaml_output {
                        add_problem(&mut problems, "Multiple spaces");
                    } else {
                        logln(writer, &format!("Multiple spaces in {pt} name {fp}"));
                    }
                    pbsp2 = true;
                    stats.sp2 += 1;
                }
            } else {
                vc.push(c);
            }
        } else if c == '.' {
            if lastc == ' ' {
                vc.pop();
            }
            vc.push(c);
        } else {
            vc.push(c);
        }
        lastc = c;
    }
    if pbsp2 {
        file = vc.iter().collect();
    }

    // Check characters
    let mut pbchr = false;
    let mut to_fix = false;
    for c in file.chars() {
        if !(c.is_alphanumeric() || (32..127).contains(&(c as i32)) || (160..256).contains(&(c as i32)) || SPECIAL_CHARS.contains(c)) {
            if !pbchr {
                pbchr = true;
                stats.car += 1;
            }
            if options.yaml_output {
                add_problem(&mut problems, &format!("Invalid char {:04X}", c as i32));
            } else {
                logln(writer, &format!("Invalid char in {pt} name {fp} -> {c} {:04X}", c as i32));
            }
            // Special case, fix U+200E by removing it (LEFT-TO-RIGHT MARK)
            if c == '\u{200E}' {
                to_fix = true;
            }
        }
    }
    if to_fix {
        file = file.replace("\u{200E}", "");
    }

    if options.yaml_output && !problems.is_empty() {
        logln(writer, &format!("- typ: {pt}"));
        logln(writer, &format!("  prb: {problems}"));
        logln(writer, &format!("  old: {fp}"));
        logln(writer, &format!("  new: {fp}\n"));
    }

    if file == original_file { None } else { Some(file) }
}

/// Checks that () [] {} «» ‹› pairs are correctly embedded and closed in a string
pub fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::<char>::new();
    let mut current_state = ' ';

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '«' | '‹' => {
                stack.push(current_state);
                current_state = c;
            }
            ')' | ']' | '}' | '»' | '›' => {
                if stack.is_empty() {
                    return false;
                }

                let opener = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '»' => '«',
                    '›' => '‹',
                    _ => unreachable!(),
                };
                if current_state == opener {
                    current_state = stack.pop().unwrap();
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    current_state == ' '
}
