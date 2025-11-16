// l69_getopts
// Play with getopts crate to implement standard options parsing for most applications (first: rcat)
// Better than getopt crate that doesn't handle double-dashed long options
//
// 2025-11-16   PV

#![allow(unused)]

//extern crate getopts;
use getopts::{Options, Fail};
use std::env;

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("My own version\n{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "version", "print application version");
    opts.optopt("a", "", "autorecurse", "+|-");
    opts.optflag("j", "", "simple option j");
    opts.optflag("w", "write", "write output");
    opts.opt("v", "", "berbose mode (can be repeated to increase verbosity level)", "v", getopts::HasArg::No, getopts::Occur::Multi);
    opts.opt("", "glob", "globbing options", "globopt", getopts::HasArg::Yes, getopts::Occur::Multi);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(error) => {
            println!("*** Error during options parsing: {}", error.to_string());
            // let error_message = match error {
            //     Fail::UnrecognizedOption(opt) => {
            //         format!("Opción no reconocida: '{}'", opt)
            //     }
            //     Fail::ArgumentMissing(opt) => {
            //         format!("Falta un argumento para la opción: '{}'", opt)
            //     }
            //     Fail::OptionMissing(opt) => {
            //         format!("La opción requerida falta: '{}'", opt)
            //     }
            //     Fail::OptionDuplicated(opt) => {
            //         format!("La opción no se puede repetir: '{}'", opt)
            //     }
            //     Fail::UnexpectedArgument(opt) => {
            //         format!("La opción no esperaba un argumento: '{}'", opt)
            //     }
            // };
            // println!("Error: {}", error_message);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("version") {
        println!("Version 1.0");
        return;
    }
    // if matches.opt_present("k") {
    //     println!("Simple option k");
    // }
    if matches.opt_present("j") {
        println!("Simple option j");
    }
    if matches.opt_present("w") {
        println!("Write output");
    }
    let verbosity = matches.opt_count("v");
    println!("verbosity: {}", verbosity);

    let globopt = matches.opt_strs("glob");
    if globopt.is_empty() {
        println!("No globbing options");
    } else {
        println!("globbing options: {:?}", globopt);
    }

    if matches.opt_present("a") {
        let x = matches.opt_str("a");
        println!("Autorecurse: {}", x.unwrap());
    }

    let output = matches.opt_str("o");

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    do_work(&input, output);
}
