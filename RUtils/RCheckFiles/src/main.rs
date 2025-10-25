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
// 2025-10-15	PV      2.2.0 Space before ? or !
// 2025-10-15	PV      2.3.0 Refactoring, separated options module, ligatures, no space before/after bracket
// 2025-10-16	PV      2.4.0 Complete set of tests for check_basename
// 2025-10-17	PV      2.4.1 Remove U+FEFF ZERO WIDTH NO-BREAK SPACE
// 2025-10-17	PV      2.4.2 Field prb: is optional when deserializing yaml file (we don't use it, and it's not renerated by rfind -yaml)
// 2025-10-17	PV      2.4.3 With option -y, only output yaml, no header or footer; Don't report space before dot in .NET and .Net
// 2025-10-17	PV      2.5.0 Ends with dot(s) ... -> …, other counts just reported, not fixed
// 2025-10-19	PV      2.5.1 Do not separate basename from extension when processing a directory name
// 2025-10-21	PV      3.0.0 Filtering on specific problems
// 2025-10-21	PV      3.1.0 Detect double extensions; Protect strings in yaml output
// 2025-20-22   PV      Clippy review
// 2025-20-24   PV      3.2.0 Dash confusables and mixed scripts
// 2025-20-25   PV      3.2.1 Space allowed before ¡
// 2025-20-25   PV      3.2.2 πΔΩ allowed with other scripts

// Note: Can't use MyGlob crate since directories names can be updated during recursive enumeration, this is not a
// supported use case of MyGlob, so hierarchical exploration is handled directly

// Standard library imports
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::os::windows::prelude::*;
use std::path::Path;
use std::process;
use std::sync::LazyLock;
use std::time::Instant;

// External crates imports
use getopt::Opt;
use logging::{LogWriter, log, logln};
use regex::Regex;
use serde::Deserialize;
use unicode_ident::{is_xid_continue, is_xid_start};
use unicode_normalization::{UnicodeNormalization, is_nfc};
use unicode_script::{Script, UnicodeScript};

// -----------------------------------
// Submodules

mod options;
mod tests;

use options::*;

// -----------------------------------
// Globals

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// ---------

// Non-ANSI characters not reported as invalid chars
const SPECIAL_CHARS: &str = "€®™©–—…×·•∶⧹⧸／⚹†‽¿🎜🎝♫♪“”‹›⚡♥";

const CHARS_NO_SPACE_AFTER: &str = "([{«‹";
const CHARS_NO_SPACE_BEFORE: &str = ")]}»›¿!‽.,…";

// Confusables for space (note that U+FEFF ZERO WIDTH NO-BREAK SPACE is always removed)
const SPACE_CONFUSABLES: [char; 14] = [
    '\u{00A0}', // U+00A0	NO-BREAK SPACE
    '\u{2000}', // U+2000	EN QUAD                         Not normalized
    '\u{2001}', // U+2001	EM QUAD                         Not normalized
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
const APOSTROPHE_CONFUSABLES: [char; 33] = [
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

const DASH_CONFUSABLES: [char; 14] = [
    '\u{00AD}',  //	- Soft hyphen
    '\u{02D7}',  // ˗ MODIFIER LETTER MINUS SIGN
    '\u{06D4}',  // ۔ ARABIC FULL STOP
    '\u{2010}',  // ‐ HYPHEN
    '\u{2011}',  // ‑ NON-BREAKING HYPHEN
    '\u{2012}',  // ‒ FIGURE DASH
    '\u{2013}',  // – EN DASH
    '\u{2043}',  // ⁃ HYPHEN BULLET
    '\u{2212}',  // − MINUS SIGN
    '\u{2796}',  // ➖ HEAVY MINUS SIGN
    '\u{2CBA}',  // Ⲻ COPTIC CAPITAL LETTER DIALECT-P NI
    '\u{2CBB}',  // ⲻ COPTIC SMALL LETTER DIALECT-P NI
    '\u{FE58}',  // ﹘ SMALL EM DASH
    '\u{10191}', //	𐆑 Roman uncia sign
];

macro_rules! hashmap {
    ($( $key:expr => $val:expr ),* $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}

// Ligatures are nice, but not in a file or directory name
static LIGATURES: LazyLock<HashMap<char, &str>> = LazyLock::new(|| {
    hashmap! {
        'Æ' => "AE",
        'æ' => "ae",
        'Ĳ' => "IJ",
        'ĳ' => "ij",
        'Œ' => "OE",
        'œ' => "oe",
        'ﬀ' => "ff",
        'ﬁ' => "fi",
        'ﬂ' => "fl",
        'ﬃ' => "ffi",
        'ﬄ' => "ffl",
        'ﬅ' => "st",
        'ﬆ' => "st",
    }
});

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
    typ: ItemType,       // dir|file
    prb: Option<String>, // The 'prb' field is optional
    old: String,
    new: String,
}

// ==============================================================================================
// Main

#[derive(Default)]
struct Statistics {
    total: u32, // Total files/dirs processed
    nnn: u32,   // Non-normalized names
    bra: u32,   // Bracket issue
    spc: u32,   // Incorrect space
    apo: u32,   // Incorrect apostrophe
    das: u32,   // Incorrect dash
    car: u32,   // Maybe incorrect char
    sp2: u32,   // Double space
    lig: u32,   // Ligatures
    sba: u32,   // Space after opening bracket or before closing bracket
    ewd: u32,   // Ends with dots
    dex: u32,   // Double extension
    mix: u32,   // Mixed scripts
    fix: u32,   // Number of path fixed
    err: u32,   // Number of errors

    ext_counter: HashMap<String, u32>, // Count of extensions (lowercase)
}

impl Statistics {}

struct TransformationData {
    space_confusables: HashSet<char>,
    apostrophe_confusables: HashSet<char>,
    dash_confusables: HashSet<char>,
    ligatures: HashMap<char, &'static str>,
    no_space_after: HashMap<char, Regex>,
    no_space_before: HashMap<char, Regex>,
}

fn get_transformation_data() -> TransformationData {
    // Transform "simple" constant and static variables defined at the beginning of the file into more elaborate
    // structures, adapted for processing
    TransformationData {
        space_confusables: HashSet::from_iter(SPACE_CONFUSABLES),
        apostrophe_confusables: HashSet::from_iter(APOSTROPHE_CONFUSABLES),
        dash_confusables: HashSet::from_iter(DASH_CONFUSABLES),
        ligatures: LIGATURES.clone(),
        no_space_after: CHARS_NO_SPACE_AFTER
            .chars()
            .map(|ch| (ch, Regex::new(format!("{} +", regex::escape(ch.to_string().as_str())).as_str()).unwrap()))
            .collect::<HashMap<char, Regex>>(),
        no_space_before: CHARS_NO_SPACE_BEFORE
            .chars()
            .map(|ch| (ch, Regex::new(format!(" +{}", regex::escape(ch.to_string().as_str())).as_str()).unwrap()))
            .collect::<HashMap<char, Regex>>(),
    }
}

fn main() {
    // println!("{}", is_single_script_identifier("2πr"));
    // process::exit(0);
    
    // Process options
    let options = Options::new().unwrap_or_else(|err| {
        let msg = format!("{}", err);
        if msg.is_empty() {
            process::exit(0);
        }
        eprintln!("{APP_NAME}: Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let transformation_data = get_transformation_data();

    // Prepare log writer
    let mut writer = logging::new(APP_NAME, APP_VERSION, !options.yaml_output);

    let start = Instant::now();

    if options.yaml_file.is_empty() {
        let mut files_stats = Statistics { ..Default::default() };
        let mut dirs_stats = Statistics { ..Default::default() };

        for source in &options.sources {
            if !options.yaml_output {
                logln(&mut writer, &format!("Analyzing {}", source));
            }
            let p = Path::new(&source);
            if p.is_file() {
                process_file(p, &mut files_stats, &options, &mut writer, &transformation_data);
            } else {
                process_directory(p, &mut dirs_stats, &mut files_stats, &options, &mut writer, &transformation_data);
            }
        }

        let duration = start.elapsed();

        fn final_status(writer: &mut LogWriter, stats: &Statistics, typename: &str) {
            log(writer, &format!("{} {}{} checked", stats.total, typename, s(stats.total)));
            if stats.nnn > 0 {
                log(writer, &format!(", {} non-normalized", stats.nnn));
            }
            if stats.bra > 0 {
                log(writer, &format!(", {} brackets issue{}", stats.bra, s(stats.bra)));
            }
            if stats.spc > 0 {
                log(writer, &format!(", {} wrong space", stats.spc));
            }
            if stats.apo > 0 {
                log(writer, &format!(", {} wrong apostrophe", stats.apo));
            }
            if stats.das > 0 {
                log(writer, &format!(", {} wrong dash", stats.das));
            }
            if stats.sp2 > 0 {
                log(writer, &format!(", {} multiple spaces", stats.sp2));
            }
            if stats.car > 0 {
                log(writer, &format!(", {} wrong character{}", stats.car, s(stats.car)));
            }
            if stats.lig > 0 {
                log(writer, &format!(", {} ligature{}", stats.lig, s(stats.lig)));
            }
            if stats.sba > 0 {
                log(writer, &format!(", {} space{} before/after bracket", stats.sba, s(stats.sba)));
            }
            if stats.ewd > 0 {
                log(writer, &format!(", {} end with 3 dots", stats.ewd));
            }
            if stats.dex > 0 {
                log(writer, &format!(", {} double extension", stats.dex));
            }
            if stats.mix > 0 {
                log(writer, &format!(", {} mixed scripts", stats.mix));
            }
            if stats.fix > 0 {
                log(writer, &format!(", {} problem{} fixed", stats.fix, s(stats.fix)));
            }
            if stats.err > 0 {
                log(writer, &format!(", {} error{}", stats.err, s(stats.err)));
            }
            logln(writer, "");
        }

        if !options.yaml_output {
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
        }
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

// Helper
fn s(n: u32) -> &'static str {
    if n > 1 { "s" } else { "" }
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
    transformation_data: &TransformationData,
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
    if let Some(new_name) = check_name(pa, "dir", dirs_stats, options, writer, transformation_data, false)
        && options.fixit
    {
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
            process_file(&pb, files_stats, options, writer, transformation_data);
        } else if ft.is_dir() {
            process_directory(&pb, dirs_stats, files_stats, options, writer, transformation_data);
        }
    }
}

fn process_file(p: &Path, files_stats: &mut Statistics, options: &Options, writer: &mut LogWriter, transformation_data: &TransformationData) {
    files_stats.total += 1;

    // Count extension
    if options.count_extensions {
        let ext = match p.extension() {
            Some(ext) => ext.to_str().unwrap().to_lowercase(), // Note that extension() returns "txt" without dot, while python returns ".txt"
            None => "(none)".to_string(),
        };
        let e = files_stats.ext_counter.entry(ext).or_insert(0);
        *e += 1;
    }

    if let Some(new_name) = check_name(p, "file", files_stats, options, writer, transformation_data, false)
        && options.fixit
    {
        logln(writer, &format!("  --> rename file \"{new_name}\""));
        let newpath = p.parent().unwrap().join(Path::new(&new_name));
        match fs::rename(p, &newpath) {
            Ok(_) => files_stats.fix += 1,
            Err(e) => logln(writer, &format!("*** Error {e}")), // Rename failed
        }
    }
}

// Either a directly name, or a file name with extension
fn check_name(
    p: &Path,
    pt: &str,
    stats: &mut Statistics,
    options: &Options,
    writer: &mut LogWriter,
    transformation_data: &TransformationData,
    fixit: bool,
) -> Option<String> {
    let fp = p.display();
    let file = p.file_name();
    file?; // file is None with network paths such as \\teraz\photo, that's normal, return None

    let file = file.unwrap().to_str();
    if file.is_none() {
        stats.err += 1;
        if !fixit {
            logln(writer, &format!("*** Invalid {pt} name {fp}, ignored"));
        }
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
    if (options.report_types.is_empty() || options.report_types.contains("bra")) && !is_balanced(&file) {
        if !fixit {
            if options.yaml_output {
                add_problem(&mut problems, "Non-balanced brackets");
            } else {
                logln(writer, &format!("Non-balanced brackets {pt} name {fp}"));
            }
        }
        stats.bra += 1;
    }

    // Check normalization
    if (options.report_types.is_empty() || options.report_types.contains("nnn")) && !is_nfc(&file) {
        if !fixit {
            if options.yaml_output {
                add_problem(&mut problems, "Non-normalized");
            } else {
                logln(writer, &format!("Non-normalized {pt} name {fp}"));
            }
        }
        stats.nnn += 1;
        // Normalize it for the rest to avoid complaining on combining accents as invalid characters
        file = file.nfc().collect();
    }

    // Check for space after opening bracket
    let mut pbnsa = false;
    if options.report_types.is_empty() || options.report_types.contains("sba") {
        for nsa in &transformation_data.no_space_after {
            if nsa.1.is_match(&file) {
                if !pbnsa {
                    if !fixit {
                        if options.yaml_output {
                            add_problem(&mut problems, "Space after opening bracket");
                        } else {
                            logln(writer, &format!("Space after {} in {pt} name {fp}", nsa.0));
                        }
                    }
                    pbnsa = true;
                }
                stats.sba += 1;
                file = nsa.1.replace_all(&file, nsa.0.to_string()).to_string();
            }
        }
    }

    // Check for space before characters such as closing bracket, !, …
    // Problem, a space if valid before .Net or .NET, so we protect these sequences (* is guaranteed not to be part of a file name)
    file = file.replace(" .NET", "*.NET").replace(" .Net", "*.Net");

    // Check for space before closing bracket
    let mut pbnsb = false;
    if options.report_types.is_empty() || options.report_types.contains("sba") {
        for nsb in &transformation_data.no_space_before {
            if nsb.1.is_match(&file) {
                if !pbnsb {
                    if !fixit {
                        if options.yaml_output {
                            add_problem(&mut problems, "Invalid space before character");
                        } else {
                            logln(writer, &format!("Space before {} in {pt} name {fp}", nsb.0));
                        }
                    }
                    pbnsb = true;
                }
                stats.sba += 1;
                file = nsb.1.replace_all(&file, nsb.0.to_string()).to_string();
            }
        }
    }

    // Unprotect (space).Net
    file = file.replace("*.", " .");

    let vc: Vec<char> = file.chars().collect();
    file = String::new();

    let mut pbspc = false;
    let mut pbapo = false;
    let mut pbdas = false;
    let mut pblig = false;

    for c in &vc {
        let mut pushed: bool = false;

        // Check for spaces
        if (options.report_types.is_empty() || options.report_types.contains("spc")) && transformation_data.space_confusables.contains(c) {
            if !fixit {
                if options.yaml_output {
                    if !pbspc {
                        add_problem(&mut problems, "Invalid space");
                    }
                } else {
                    logln(writer, &format!("Invalid space in {pt} name {fp} -> {:04X}", *c as i32));
                }
            }
            if !pbspc {
                pbspc = true;
                stats.spc += 1;
            }
            file.push(' ');
            pushed = true;
        }

        // Check apostrophes
        if (options.report_types.is_empty() || options.report_types.contains("apo")) && transformation_data.apostrophe_confusables.contains(c) {
            if !fixit {
                if options.yaml_output {
                    if !pbapo {
                        add_problem(&mut problems, "Invalid apostrophe");
                    }
                } else {
                    logln(writer, &format!("Invalid apostrophe in {pt} name {fp} -> {c} {:04X}", *c as i32));
                }
            }
            if !pbapo {
                pbapo = true;
                stats.apo += 1;
            }
            file.push('\'');
            pushed = true;
        }

        // Check dashes
        if (options.report_types.is_empty() || options.report_types.contains("das")) && transformation_data.dash_confusables.contains(c) {
            if !fixit {
                if options.yaml_output {
                    if !pbdas {
                        add_problem(&mut problems, "Invalid dash");
                    }
                } else {
                    logln(writer, &format!("Invalid dash in {pt} name {fp} -> {c} {:04X}", *c as i32));
                }
            }
            if !pbdas {
                pbdas = true;
                stats.das += 1;
            }
            file.push('-');
            pushed = true;
        }

        if (options.report_types.is_empty() || options.report_types.contains("lig")) && transformation_data.ligatures.contains_key(c) {
            if !fixit {
                if options.yaml_output {
                    if !pblig {
                        add_problem(&mut problems, "Ligature found");
                    }
                } else {
                    logln(writer, &format!("Ligature found in {pt} name {fp} -> {c} {:04X}", *c as i32));
                }
            }
            if !pblig {
                pblig = true;
                stats.lig += 1;
            }
            file.push_str(transformation_data.ligatures[c]);
            pushed = true;
        }

        if !pushed {
            file.push(*c);
        }
    }

    // Extract basename and extension, moved out "ends with dot" block since it's alsp used by double extension check
    let fc = file.clone();
    let (mut basename, extension) = if pt == "dir" {
        (fc.as_str(), "")
    } else {
        let path = Path::new(&fc);
        let b = path.file_stem().and_then(|s| s.to_str()).unwrap_or(&fc); // Fallback to the full name if no stem
        let e = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        (b, e)
    };

    // Check for file.txt.txt (only for files)
    if pt == "file" && (options.report_types.is_empty() || options.report_types.contains("dex")) && !extension.is_empty() {
        let ext2 = Path::new(basename).extension().and_then(|s| s.to_str()).unwrap_or("");
        if extension.to_lowercase() == ext2.to_lowercase() {
            if !fixit {
                if options.yaml_output {
                    add_problem(&mut problems, "Double extension");
                } else {
                    logln(writer, &format!("Double extension {ext2} in file {fp}"));
                }
            }
            stats.dex += 1;
            file = basename[..basename.len() - extension.len() - 1].to_string() + "." + extension;
            let path = Path::new(&file);
            basename = path.file_stem().and_then(|s| s.to_str()).unwrap(); // No need for a default, we know there is an extension in this block
        }
    }

    // Check if ends by 3 dots (but not 4), replace by single char …
    // But if file ends with 1, 2, 4 or + dots, just report it, don't fix it
    // Use path methits to split into basename/extension, even if it's probably overkill
    // Note: for file name analysis, we only focus on basename, while for directory name, just use the whole name as is
    if options.report_types.is_empty() || options.report_types.contains("ewd") {
        let mut end_dots_count = 0;
        for c in basename.chars().rev() {
            if c == '.' {
                end_dots_count += 1;
            } else {
                break;
            }
        }
        if end_dots_count > 0 {
            if !fixit {
                if options.yaml_output {
                    add_problem(&mut problems, format!("Ends with {end_dots_count} dot{}", s(end_dots_count)).as_str());
                } else {
                    logln(writer, &format!("{pt} ends with {end_dots_count} dot{} in {fp}", s(end_dots_count)));
                }
            }
            stats.ewd += 1;
            // replace ... by …
            if end_dots_count == 3 {
                file = basename[..basename.len() - 3].to_string() + "…" + "." + extension;
            }
        }
    }

    // Check multiple spaces (and space before extension)
    if options.report_types.is_empty() || options.report_types.contains("sp2") {
        let mut pbsp2 = false;
        let mut vc: Vec<char> = Vec::new();
        let mut lastc = '_';
        for c in file.chars() {
            if c == ' ' {
                if lastc == ' ' {
                    if !pbsp2 {
                        if !fixit {
                            if options.yaml_output {
                                add_problem(&mut problems, "Multiple spaces");
                            } else {
                                logln(writer, &format!("Multiple spaces in {pt} name {fp}"));
                            }
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
    }

    // Check characters
    if options.report_types.is_empty() || options.report_types.contains("car") {
        let mut pbchr = false;
        let mut to_fix = false;
        for c in file.chars() {
            if !(c.is_alphanumeric() || (32..127).contains(&(c as i32)) || (160..256).contains(&(c as i32)) || SPECIAL_CHARS.contains(c)) {
                if !pbchr {
                    pbchr = true;
                    stats.car += 1;
                }
                if !fixit {
                    if options.yaml_output {
                        add_problem(&mut problems, &format!("Invalid char {:04X}", c as i32));
                    } else {
                        logln(writer, &format!("Invalid char in {pt} name {fp} -> {c} {:04X}", c as i32));
                    }
                }
                // Special case, fix U+200E by removing it (LEFT-TO-RIGHT MARK)
                if c == '\u{200E}' || c == '\u{FEFF}' {
                    to_fix = true;
                }
            }
        }
        // Some characters are always removed, will only cause problems in paths/files names
        if to_fix {
            file = file.replace("\u{200E}", ""); // LEFT-TO-RIGHT MARK
            file = file.replace("\u{FEFF}", ""); // ZERO WIDTH NO-BREAK SPACE
        }
    }

    // Check for mixed scripts
    if options.report_types.is_empty() || options.report_types.contains("mix") {
        if !is_single_script(&file) {
            if options.yaml_output {
                add_problem(&mut problems, "Mixed scripts");
            } else {
                logln(writer, &format!("Mixed scripts in {pt} name {fp}"));
            }
            stats.mix += 1;
        }
    }

    if !fixit && options.yaml_output && !problems.is_empty() {
        logln(writer, &format!("- typ: {pt}"));
        logln(writer, &format!("  prb: {problems}"));
        logln(writer, &format!("  old: {}", to_yaml_single_quoted(format!("{fp}").as_str())));
        logln(writer, &format!("  new: {}\n", to_yaml_single_quoted(format!("{fp}").as_str())));
    }

    if file == original_file { None } else { Some(file) }
}

/// Wraps a string in single quotes for safe inclusion in a YAML file.
///
/// In YAML, single-quoted strings handle most special characters literally, including '#' and '\'. The only character
/// that must be escaped is the single quote itself, which is done by doubling it (e.g., 'It''s').
///
/// This function always returns a quoted string, which is always valid.
///
/// You must use quotes (like this function provides) if your string:
/// - Contains a # (comment character)
/// - Contains a colon followed by a space (: )
/// - Contains a hyphen followed by a space (- ) at the beginning
/// - Contains a single quote (')
/// - Starts or ends with whitespace
/// - Is empty
/// - Is the word true, false, yes, no, on, off, null, or ~
/// - Looks like a number (e.g., 123, 45.6)
///
/// Writing a function checking all these cases would be complex, so always surrounding with quotes is simpler and
/// quicker
fn to_yaml_single_quoted(s: &str) -> String {
    // 1. Escape any single quotes by replacing them with two single quotes.
    let escaped = s.replace('\'', "''");

    // 2. Wrap the escaped string in single quotes.
    format!("'{}'", escaped)
}

// Single script test for a whole string: split in identifiers (words), and check that all identiers are single script
pub fn is_single_script(s: &str) -> bool {
    extract_identifiers(s).iter().all(|id| is_single_script_identifier(id))
}

/// Extracts identifiers from a string slice based on UAX #31 default identifier definition.
/// An identifier is a sequence of characters starting with a character with the
/// `XID_Start` property, followed by zero or more characters with the `XID_Continue` property.
/// See https://www.unicode.org/reports/tr31/#R1
pub fn extract_identifiers(text: &str) -> Vec<&str> {
    let mut identifiers = Vec::new();
    let mut it = text.char_indices().peekable();
    let mut start_pos: Option<usize> = None;

    while let Some((i, c)) = it.next() {
        if start_pos.is_none() && is_xid_start(c) {
            start_pos = Some(i);
        }

        if let Some(start) = start_pos {
            let next_is_continue = it.peek().map_or(false, |&(_, next_c)| is_xid_continue(next_c));
            if !next_is_continue {
                identifiers.push(&text[start..i + c.len_utf8()]);
                start_pos = None;
            }
        }
    }
    identifiers
}

// My own version to detect whether a string contains mixed scripts, since unicode_security::RestrictionLevelDetection
// rejects runic or arabic strings at anay level on compliance for instance, which is too strict for me.
// Following combinations are allowed (from UTS #39):
// - Latin + Han + Hiragana + Katakana; or equivalently: Latn + Jpan
// - Latin + Han + Bopomofo; or equivalently: Latn + Hanb
// - Latin + Han + Hangul; or equivalently: Latn + Kore
// I should probably allow some exceptions when there is no risk of confusion, such as Δt, Teχ, 2πr or πⁿ
pub fn is_single_script_identifier(s: &str) -> bool {
    // Collect all unique scripts in the string, ignoring common ones.
    let scripts_in_string: HashSet<Script> = s
        .chars()
        .map(|c| if "πΔΩ".contains(c) { Script::Common } else { c.script() })
        .filter(|&sc| sc != Script::Common && sc != Script::Inherited && sc != Script::Unknown)
        .collect();

    // If there are no specific scripts (e.g., string is all digits), it's valid.
    // If there is only one specific script, it's also valid.
    if scripts_in_string.len() <= 1 {
        return true;
    }

    // Define the allowed script combinations as specified in UTS #39
    // for moderately restrictive profiles.
    let allowed_combinations: Vec<HashSet<Script>> = vec![
        // Japanese: Latin + Han + Hiragana + Katakana
        HashSet::from([Script::Latin, Script::Han, Script::Hiragana, Script::Katakana]),
        // Korean: Latin + Han + Hangul
        HashSet::from([Script::Latin, Script::Han, Script::Hangul]),
        // Chinese: Latin + Han + Bopomofo
        HashSet::from([Script::Latin, Script::Han, Script::Bopomofo]),
    ];

    // Check if the set of scripts found in the string is a subset of any allowed combination.
    allowed_combinations.iter().any(|combo| scripts_in_string.is_subset(combo))
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
