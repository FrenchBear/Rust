// PVMarkup
// Quick-and-dirty main function to test code during dev
//
// 2025-05-05   PV      First version

#![allow(unused)]

use colored::*;

fn main() {
    println!("{}", "Style Default");
    println!("{}", "Style Bold".bold());
    println!("{}", "Style Underline".underline());
    println!("{}", "Style Dimmed".dimmed());
    println!("{}", "Style Reversed".reversed());
    println!("{}", "Style Italic".italic());
    println!("{}", "Style Blink".italic());
    println!("{}", "Style Hidden".hidden());
    println!("{}", "Style Strikethrough".strikethrough());
    println!();
    println!("{}", "Bright write + Style default".bright_white());
    println!("{}", "Bright write + Style Bold".bright_white().bold());
    println!("{}", "Bright write + Style Underline".bright_white().underline());
    println!("{}", "Bright write + Style Dimmed".bright_white().dimmed());
    println!("{}", "Bright write + Style Reversed".bright_white().reversed());
    println!("{}", "Bright write + Style Italic".bright_white().italic());
    println!("{}", "Bright write + Style Blink".bright_white().italic());
    println!("{}", "Bright write + Style Hidden".bright_white().hidden());
    println!("{}", "Bright write + Style Strikethrough".bright_white().strikethrough());
}
