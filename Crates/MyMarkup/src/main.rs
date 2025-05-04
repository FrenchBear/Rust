// MyMarkup
// Quick-and-dirty main function to test code during dev
//
// 2025-05-05   PV      First version

#![allow(unused)]

use terminal_size as _;

use mymarkup::*;

fn main() {
    // test_own();
    // return;

    let text = "⌊Usage⌋: rgrep ¬[⦃?⦄|⦃-?⦄|⦃-h⦄|⦃??⦄] [⦃-i⦄] [⦃-w⦄] [⦃-F⦄] [⦃-r⦄] [⦃-v⦄] [⦃-c⦄] [⦃-l⦄] pattern [source...]
⦃?⦄|⦃-?⦄|⦃-h⦄  ¬Show this message
⦃??⦄       ¬Show advanced usage notes
⦃-i⦄       ¬Ignore case during search
⦃-w⦄       ¬Whole word search
⦃-F⦄       ¬Fixed string search (no regexp interpretation), also for patterns starting with - ? or help
⦃-r⦄       ¬Use autorecurse, see advanced help
⦃-c⦄       ¬Suppress normal output, show count of matching lines for each file
⦃-l⦄       ¬Suppress normal output, show matching file names only
⦃-v⦄       ¬Verbose output
pattern  ¬Regular expression to search
source   ¬File or folder where to search, glob syntax supported. Without source, search stdin

⟪⌊Advanced usage notes⌋⟫

Counts include with and without BOM variants.
8-bit text files are likely Windows 1252/Latin-1/ANSI or OEM 850/OEM 437, there is no detailed analysis.
Files without BOM must be more than 10 characters for auto-detection of UTF-8 or UTF-16.

⌊EOL Styles⌋
- ¬⟪Windows⟫: \\r\\n
- ¬⟪Unix⟫: \\n
- ¬⟪Mac⟫: \\r

⌊Warnings reported⌋
• ¬Empty files.
• ¬Source text files (based on extension) that should contain text, but with unrecognized content.
• ¬UTF-8 files with BOM.
• ¬UTF-16 files without BOM.
• ¬Different encodings for a given file type (extension) in a folder.
• ¬Mixed EOL styles in a file.
• ¬Different EOL styles for a given file type (extension) in a folder.

⌊Glob pattern rules⌋
• ¬⟦?⟧ matches any single character.
• ¬⟦*⟧ matches any (possibly empty) sequence of characters.
• ¬⟦**⟧ matches the current directory and arbitrary subdirectories. To match files in arbitrary subdirectories, use ⟦**\\*⟧. This sequence must form a single path component, so both **a and b** are invalid and will result in an error.
• ¬⟦[...]⟧ matches any character inside the brackets. Character sequences can also specify ranges of characters, as ordered by Unicode, so e.g. ⟦[0-9]⟧ specifies any character between 0 and 9 inclusive. Special cases: ⟦[[]⟧ represents an opening bracket, ⟦[]]⟧ represents a closing bracket. 
• ¬⟦[!...]⟧ is the negation of ⟦[...]⟧, i.e. it matches any characters not in the brackets.
• ¬The metacharacters ⟦?⟧, ⟦*⟧, ⟦[⟧, ⟦]⟧ can be matched by using brackets (e.g. ⟦[?]⟧). When a ⟦]⟧ occurs immediately following ⟦[⟧ or ⟦[!⟧ then it is interpreted as being part of, rather then ending, the character set, so ⟦]⟧ and NOT ⟦]⟧ can be matched by ⟦[]]⟧ and ⟦[!]]⟧ respectively. The ⟦-⟧ character can be specified inside a character sequence pattern by placing it at the start or the end, e.g. ⟦[abc-]⟧.
• ¬⟦{choice1,choice2...}⟧  match any of the comma-separated choices between braces. Can be nested, and include ⟦?⟧, ⟦*⟧ and character classes.
• ¬Character classes ⟦[ ]⟧ accept regex syntax such as ⟦[\\d]⟧ to match a single digit, see https://docs.rs/regex/latest/regex/#character-classes for character classes and escape sequences supported.

⌊Autorecurse glob pattern transformation⌋
• ¬⟪Constant pattern (no filter, no **⟧) pointing to a folder⟫: ⟦\\**\\*⟧ is appended at the end to search all files of all subfolders.
• ¬⟪Patterns without ⟦**⟧ and ending with a filter⟫: ⟦\\**⟧ is inserted before final filter to find all matching files of all subfolders.
";

    //let text = "to match a single digit, see https://docs.rs/regex/latest/regex/#character-classes for character classes and escape sequences supported.";

    MyMarkup::render_markup(text);
    //test_own();
}

fn test_own() {
    println!("Style Default");
    println!("Style {}Bold{}, and default", STYLE_BOLD_ON, STYLE_BOLD_OFF);
    println!("Style {}Underline{}, and default", STYLE_UNDERLINE_ON, STYLE_UNDERLINE_OFF);
    println!("Style {}Dimmed{}, and default", STYLE_DIM_ON, STYLE_DIM_OFF);
    println!("Style {}Italic{}, and default", STYLE_ITALIC_ON, STYLE_ITALIC_OFF);
    println!("Style {}Underline{}, and default", STYLE_UNDERLINE_ON, STYLE_UNDERLINE_OFF);
    println!("Style {}Blink{}, and default", STYLE_BLINK_ON, STYLE_BLINK_OFF);
    println!("Style {}Reverse{}, and default", STYLE_REVERSE_ON, STYLE_REVERSE_OFF);
    println!("Style {}Hidden{}, and default", STYLE_HIDDEN_ON, STYLE_HIDDEN_OFF);
    println!("Style {}Strikethrough{}, and default", STYLE_STRIKETHROUGH_ON, STYLE_STRIKETHROUGH_OFF);

    println!("\nColors");
    let fga = [
        ("Black", FG_BLACK),
        ("Red", FG_RED),
        ("Green", FG_GREEN),
        ("Yellow", FG_YELLOW),
        ("Blue", FG_BLUE),
        ("Magenta", FG_MAGENTA),
        ("Cyan", FG_CYAN),
        ("White", FG_WHITE),
        ("Default", FG_DEFAULT),
        ("Bright Black", FG_BRIGHT_BLACK),
        ("Bright Red", FG_BRIGHT_RED),
        ("Bright Green", FG_BRIGHT_GREEN),
        ("Bright Yellow", FG_BRIGHT_YELLOW),
        ("Bright Blue", FG_BRIGHT_BLUE),
        ("Bright Magenta", FG_BRIGHT_MAGENTA),
        ("Bright Cyan", FG_BRIGHT_CYAN),
        ("Bright White", FG_BRIGHT_WHITE),
    ];

    for (name, fg) in fga {
        println!("{}{:?}{}", fg, name, FG_DEFAULT);
    }
}
