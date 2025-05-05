// MyMarkup library
// Parse and render my own markup language
//
// 2025-05-05   PV      First version
//
// MyMarkup use pecialized brackets for formatting text:
// ⟪Bold⟫           ~W  ~X
// ⟨Italic⟩         ~w  ~x
// ⌊Underline⌋      ~D  ~F
// ⌈Striketrough⌉   ~Q  ~S
// ⟦Color1⟧         ~c  ~v  Cyan
// ⦃Color2⦄         ~C  ~V  Yellow
// ⟮⟯               ~à  ~)  (Unused for now)
// ¬ (AltGr+7) sets left margin

//#![allow(unused)]

// External crates imports
use terminal_size::{Width, terminal_size};

// -----------------------------------
// Submodules

mod tests;

// -----------------------------------
// Globals

const LIB_VERSION: &str = "1.0.0";
const SHOW_LIMITS: bool = false; // For dev

// Styles
pub const STYLE_CLEAR: &str = "\x1b[0m";
pub const STYLE_BOLD_ON: &str = "\x1b[1m";
pub const STYLE_BOLD_OFF: &str = "\x1b[22m"; // Clears Dim and Bold
pub const STYLE_DIM_ON: &str = "\x1b[2m";
pub const STYLE_DIM_OFF: &str = "\x1b[22m"; // Clears Dim and Bold
pub const STYLE_ITALIC_ON: &str = "\x1b[3m";
pub const STYLE_ITALIC_OFF: &str = "\x1b[23m";
pub const STYLE_UNDERLINE_ON: &str = "\x1b[4m";
pub const STYLE_UNDERLINE_OFF: &str = "\x1b[24m";
pub const STYLE_BLINK_ON: &str = "\x1b[5m";
pub const STYLE_BLINK_OFF: &str = "\x1b[25m";
pub const STYLE_REVERSE_ON: &str = "\x1b[7m";
pub const STYLE_REVERSE_OFF: &str = "\x1b[27m";
pub const STYLE_HIDDEN_ON: &str = "\x1b[8m";
pub const STYLE_HIDDEN_OFF: &str = "\x1b[28m";
pub const STYLE_STRIKETHROUGH_ON: &str = "\x1b[9m";
pub const STYLE_STRIKETHROUGH_OFF: &str = "\x1b[29m";

// Colors
pub const FG_BLACK: &str = "\x1b[30m";
pub const FG_RED: &str = "\x1b[31m";
pub const FG_GREEN: &str = "\x1b[32m";
pub const FG_YELLOW: &str = "\x1b[33m";
pub const FG_BLUE: &str = "\x1b[34m";
pub const FG_MAGENTA: &str = "\x1b[35m";
pub const FG_CYAN: &str = "\x1b[36m";
pub const FG_WHITE: &str = "\x1b[37m";
pub const FG_DEFAULT: &str = "\x1b[39m";
pub const FG_BRIGHT_BLACK: &str = "\x1b[90m";
pub const FG_BRIGHT_RED: &str = "\x1b[91m";
pub const FG_BRIGHT_GREEN: &str = "\x1b[92m";
pub const FG_BRIGHT_YELLOW: &str = "\x1b[93m";
pub const FG_BRIGHT_BLUE: &str = "\x1b[94m";
pub const FG_BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const FG_BRIGHT_CYAN: &str = "\x1b[96m";
pub const FG_BRIGHT_WHITE: &str = "\x1b[97m";

pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE: &str = "\x1b[44m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_CYAN: &str = "\x1b[46m";
pub const BG_WHITE: &str = "\x1b[47m";
pub const BG_DEFAULT: &str = "\x1b[49m";
pub const BG_BRIGHT_BLACK: &str = "\x1b[100m";
pub const BG_BRIGHT_RED: &str = "\x1b[101m";
pub const BG_BRIGHT_GREEN: &str = "\x1b[102m";
pub const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";
pub const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
pub const BG_BRIGHT_CYAN: &str = "\x1b[106m";
pub const BG_BRIGHT_WHITE: &str = "\x1b[107m";

// -----------------------------------
// Structures

#[derive(Debug)]
pub struct MyMarkup {}

impl MyMarkup {
    pub fn version() -> &'static str {
        LIB_VERSION
    }

    pub fn render_markup(txt_str: &str) {
        // To simplify code, ensure that string always ends with \n
        let mut txt_string = String::from(txt_str);
        if !(txt_string.ends_with('\n')) {
            txt_string.push('\n');
        }

        let width = if let Some((Width(w), _)) = terminal_size() {
            w as usize
        } else {
            80usize
        };

        //let width = 40;

        if SHOW_LIMITS {
            for _ in 0..width {
                print!("-");
            }
            println!();
        }

        // ToDo: Tabs expansion

        let mut word = String::new();
        let mut len = 0;

        let mut col = 0;
        let mut tab = 0;
        for c in txt_string.chars() {
            match c {
                '⟪' => {
                    word.push_str(STYLE_BOLD_ON);
                    continue;
                }
                '⟫' => {
                    word.push_str(STYLE_BOLD_OFF);
                    continue;
                }
                '⟨' => {
                    word.push_str(STYLE_ITALIC_ON);
                    continue;
                }
                '⟩' => {
                    word.push_str(STYLE_ITALIC_OFF);
                    continue;
                }
                '⌊' => {
                    word.push_str(STYLE_UNDERLINE_ON);
                    continue;
                }
                '⌋' => {
                    word.push_str(STYLE_UNDERLINE_OFF);
                    continue;
                }
                '⟦' => {
                    word.push_str(FG_CYAN);
                    continue;
                }
                '⟧' => {
                    word.push_str(FG_DEFAULT);
                    continue;
                }
                '⦃' => {
                    word.push_str(FG_YELLOW);
                    continue;
                }
                '⦄' => {
                    word.push_str(FG_DEFAULT);
                    continue;
                }
                '\r' => continue,
                '\n' => {
                    if !word.is_empty() && !is_only_spaces(&word) {
                        if col + len <= width {
                            print!("{}", word);
                            if SHOW_LIMITS {
                                col += len;
                                while col < width {
                                    col += 1;
                                    print!(" ");
                                }
                                print!("|");
                            }
                            println!();
                        } else {
                            if SHOW_LIMITS {
                                while col < width {
                                    col += 1;
                                    print!(" ");
                                }
                                print!("|");
                            }
                            println!();
                            for _ in 0..tab {
                                print!(" ");
                            }
                            while word.starts_with(' ') {
                                word.remove(0);
                                len -= 1;
                            }
                            print!("{}", word);
                            col = tab + len;
                            if SHOW_LIMITS {
                                while col < width {
                                    col += 1;
                                    print!(" ");
                                }
                                print!("|");
                            }
                            println!();
                        }
                    } else {
                        while col < width {
                            col += 1;
                            print!(" ");
                        }
                        if SHOW_LIMITS {
                            print!("|");
                        }
                        println!();
                    }
                    word.clear();
                    len = 0;
                    col = 0;
                    tab = 0;
                }
                '¬' => {
                    print!("{}", word);
                    col += len;
                    tab = col;
                    word.clear();
                    len = 0;
                }
                ' ' => {
                    if !word.is_empty() {
                        if is_only_spaces(&word) {
                            word.push(c);
                            len += 1;
                            continue;
                        }

                        if col + len <= width {
                            print!("{}", word);
                            col += len;
                            word.clear();
                            len = 0;
                        } else {
                            if SHOW_LIMITS {
                                while col < width {
                                    col += 1;
                                    print!(" ");
                                }
                                print!("|");
                            }
                            println!();
                            for _ in 0..tab {
                                print!(" ");
                            }
                            col = tab;
                            while word.starts_with(' ') {
                                word.remove(0);
                                len -= 1;
                            }

                            print!("{}", word);
                            col += len;
                            word.clear();
                            len = 0;
                        }
                    }
                    word.push(' ');
                    len += 1;
                }
                _ => {
                    if tab + len >= width - 1 {
                        // We can't accumulate char, it would be longer than width
                        if col > tab {
                            // if we have already printed some chars, we need to flush and start a new line
                            if SHOW_LIMITS {
                                while col < width {
                                    col += 1;
                                    print!(" ");
                                }
                                print!("|");
                            }
                            println!();

                            for _ in 0..tab {
                                print!(" ");
                            }
                            col = tab;
                        }

                        while word.starts_with(' ') {
                            word.remove(0);
                            len -= 1;
                        }

                        if col + len >= width {
                            println!("{}|", word);

                            word.clear();
                            len = 0;
                            for _ in 0..tab {
                                print!(" ");
                            }
                            col = tab;
                        }
                    }

                    word.push(c);
                    len += 1;
                }
            }
        }
        println!();
    }
}

fn is_only_spaces(word: &str) -> bool {
    word.chars().all(|c| c == ' ')
}
