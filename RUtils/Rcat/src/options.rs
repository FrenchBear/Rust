// rcat - Module options
// Options processing
//
// 2025-10-24   PV      First version
// 2025-11-16   PV      Options -a+/-a-, -d
// 2025-11-16   PV      Use MyGlobCLOptions

// Application imports
use crate::*;

// Standard library imports
use std::error::Error;

// External crates imports
use getopts::{Fail, Options};
use mymarkup::MyMarkup;

// Dedicated struct to store command line arguments
#[derive(Debug, Default)]
pub struct AppOptions {
    pub sources: Vec<String>,
    pub mgclo: MyGlobCLOptions,
    pub text_encoding: Option<String>,
    pub debug: bool,
    pub verbose: usize,
}

impl AppOptions {
    fn header() {
        println!("{APP_NAME} {APP_VERSION}\n{APP_DESCRIPTION}");
    }

    fn header_copyright() {
        Self::header();
        println!("Copyright ©2025 Pierre Violent");
    }

    fn usage() {
        Self::header();
        println!();
        let text = "⌊Usage⌋: {APP_NAME} [⟨option⟩...] [⟨source⟩...]

⌊Options⌋:
⦃?⦄|⦃-?⦄|⦃help⦄|⦃-help⦄     ¬Show this message
⦃??⦄|⦃-??⦄|⦃--help⦄       ¬Show advanced usage notes
⦃--glob ⟨opt⟩[,⟨opt⟩]⦄... ¬Globbing specific options (see extended help)
⦃-e|--encoding⦄ ⟨enc⟩   ¬Only concatenate text files to output after conversion to specified encoding ⟨enc⟩ (utf-8|utf-8-bom|utf16-le|utf16-be)

⟨source⟩              ¬File or directory to read (globbing supported, autorecurse by default). Without source, read stdin";

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
    }

    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} FILE [options]", program);
        print!("My own version\n{}", opts.usage(&brief));
    }

    fn extended_usage() {
        Self::header_copyright();
        println!();

        MyMarkup::render_markup("⌊Dependencies⌋:");
        println!("- MyGlob: {}", MyGlobSearch::version());
        println!("- MyMarkup: {}", MyMarkup::version());
        println!("- getopts: {}", env!("DEP_GETOPTS_VERSION"));
        println!();

        let text = "⟪⌊Advanced usage notes⌋⟫

⌊Advanced options⌋:
⦃-d⦄       ¬Debug mode, show internal dev information\n\n"
            .to_string()
            + MyGlobCLOptions::options();

        MyMarkup::render_markup(text.replace("{APP_NAME}", APP_NAME).as_str());
        println!();
        MyMarkup::render_markup(MyGlobSearch::glob_syntax());
    }

    /// Build a new struct Options analyzing command line parameters.<br/>
    /// Some invalid/inconsistent options or missing arguments return an error.
    pub fn new() -> Result<AppOptions, Box<dyn Error>> {
        let mut args: Vec<String> = std::env::args().collect();

        // Special options processing
        if args.len() > 1 {
            if args[1] == "?" || args[1] == "-?" || args[1] == "/?" || args[1].to_lowercase() == "help" || args[1].to_lowercase() == "-help" || args[1].to_lowercase() == "/help" {
                Self::usage();
                return Err("".into());
            }

            if args[1] == "??" || args[1] == "-??" || args[1] == "/??" || args[1].to_lowercase() == "--help" {
                Self::extended_usage();
                return Err("".into());
            }
        }

        let mut opts = Options::new();
        // Common options
        opts.optflag("", "version", "print application version");
        opts.opt(
            "v",
            "verbose",
            "verbose mode (can be repeated to increase verbosity level)",
            "",
            getopts::HasArg::No,
            getopts::Occur::Multi,
        );
        opts.optflag("d", "debug", "debug mode (for development)");
        // app-specific options
        opts.optopt("", "glob", "glob command line options", "(see extended help)");
        opts.optopt("e", "encoding", "output text encoding", "utf-8|utf-8-bom|utf16-le|utf16-be");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(error) => {
                return Err(error.into());
            }
        };

        if matches.opt_present("version") {
            Self::header_copyright();
            return Err("".into());
        }

        // Application options processing
        let mut app_options = AppOptions {
            mgclo: MyGlobCLOptions::new(),
            ..Default::default()
        };

        // -v/--verbose can be repeated, even if it's not used in this app
        app_options.verbose = matches.opt_count("v");

        // -d/--debug
        if matches.opt_present("d") {
            app_options.debug = true;
        }

        // --glob command line options can be repeated
        for opt in matches.opt_strs("glob").iter() {
            if let Err(e) = app_options.mgclo.process_options(opt) {
                return Err(e.into());
            }
        }

        //  -e/--encoding only process text files and convert output to specified encoding
        // ToDo: Check value and use it
        if matches.opt_present("e") {
            app_options.text_encoding = matches.opt_str("e");
            println!("*** Warning: option -e/--encoding not implemented yet");
        }

        // Finally process positional arguments
        app_options.sources = matches.free;

        Ok(app_options)
    }
}
