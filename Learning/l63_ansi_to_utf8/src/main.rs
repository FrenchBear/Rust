// l63_ansi_to_utf8
// Convert files from a list in c:\temp\f1.txt from ANSI to UTF-8 encoding
//
// 2025-05-15   PV

#![allow(unused)]

use textautodecode::{TextAutoDecode, TextFileEncoding};

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let m = build_windows1252_to_utf8_map();

    let common_ansi_chars: [u8; 24] = [
        146u8, // ’
        150u8, // –
        160u8, //
        169u8, // ©
        176u8, // °
        178u8, // ²
        182u8, // ¶
        183u8, // ·
        192u8, // À
        194u8, // Â
        201u8, // É
        223u8, // ß
        224u8, // à
        226u8, // â
        231u8, // ç
        232u8, // è
        233u8, // é
        234u8, // ê
        238u8, // î
        239u8, // ï
        244u8, // ô
        249u8, // ù
        251u8, // û
        252u8, // ü
    ];

    let cac: HashSet<u8> = HashSet::from_iter(common_ansi_chars);

    // First check that files exist, they contain at least 1 8-bit character, and each 8-bit character
    // is contained in common_ansi_chars
    let list_files = fs::read_to_string(r"c:\temp\f1.txt").unwrap();
    for file in list_files.lines() {
        //println!("<{}>", file);
        let mut res = get_non_ascii_chars(file);
        assert!(res.len() > 0);
        let zz = res.difference(&cac);
        assert!(zz.count() == 0);
    }

    // Now convert
    for file in list_files.lines() {
        let file_path = Path::new(file);
        let tad_res = TextAutoDecode::read_text_file(file_path).unwrap();
        assert!(tad_res.encoding == TextFileEncoding::EightBit);

        let ext = file_path.extension().unwrap();
        let bak_path = PathBuf::from(file.to_string() + ".bak");
        println!("{} -> {}", file_path.display(), bak_path.display());
        std::fs::rename(file_path, bak_path).unwrap();

        let mut new_file = File::create(file_path).unwrap();
        new_file.write_all(tad_res.text.unwrap().as_bytes()).unwrap();
        //return;
    }
}

fn get_non_ascii_chars(source: &str) -> HashSet<u8> {
    // let args: Vec<String> = std::env::args().collect();
    // if args.len()<=1 {
    //     println!("Usage: app ")
    // }
    // let source = &args[1];

    let mut res = HashSet::<u8>::new();
    let buf = fs::read(source).unwrap();

    let mut line = 1;
    for c in buf {
        if c == b'\n' {
            line += 1;
        }
        if !((32..127).contains(&c) || c == b'\r' || c == b'\n' || c == b'\t') {
            //println!("Non ASCII line {line}: {:02X}", c as u8);
            res.insert(c);
        }
    }

    res
}

fn build_windows1252_to_utf8_map() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();

    // Wikipedia https://en.wikipedia.org/wiki/Windows-1252
    // According to the information on Microsoft's and the Unicode Consortium's websites, positions 81, 8D, 8F, 90, and
    // 9D are unused; however, the Windows API MultiByteToWideChar maps these to the corresponding C1 control codes. The
    // "best fit" mapping documents this behavior, too.[22]

    map.insert(128, "\u{20AC}"); // Euro Sign
    // No 129=0x81
    map.insert(130, "\u{201A}"); // Single Low-9 Quotation Mark
    map.insert(131, "\u{0192}"); // Latin Small Letter F With Hook
    map.insert(132, "\u{201E}"); // Double Low-9 Quotation Mark
    map.insert(133, "\u{2026}"); // Horizontal Ellipsis
    map.insert(134, "\u{2020}"); // Dagger
    map.insert(135, "\u{2021}"); // Double Dagger
    map.insert(136, "\u{02C6}"); // Modifier Letter Circumflex Accent
    map.insert(137, "\u{2030}"); // Per Mille Sign
    map.insert(138, "\u{0160}"); // Latin Capital Letter S With Caron
    map.insert(139, "\u{2039}"); // Single Left-Pointing Angle Quotation Mark
    map.insert(140, "\u{0152}"); // Latin Capital Ligature OE
    // No 141=0x8D
    map.insert(142, "\u{017D}"); // Latin Capital Letter Z With Caron
    // No 143=0x8F

    // No 144=0x90
    map.insert(145, "\u{2018}"); // Left Single Quotation Mark
    map.insert(146, "\u{2019}"); // Right Single Quotation Mark
    map.insert(147, "\u{201C}"); // Left Double Quotation Mark
    map.insert(148, "\u{201D}"); // Right Double Quotation Mark
    map.insert(149, "\u{2022}"); // Bullet
    map.insert(150, "\u{2013}"); // En Dash
    map.insert(151, "\u{2014}"); // Em Dash
    map.insert(152, "\u{02DC}"); // Small Tilde
    map.insert(153, "\u{2122}"); // Trade Mark Sign
    map.insert(154, "\u{0161}"); // Latin Small Letter S With Caron
    map.insert(155, "\u{203A}"); // Single Right-Pointing Angle Quotation Mark
    map.insert(156, "\u{0153}"); // Latin Small Ligature OE
    // No 157=0x9D
    map.insert(158, "\u{017E}"); // Latin Small Letter Z With Caron
    map.insert(159, "\u{0178}"); // Latin Capital Letter Y With Diaeresis

    map.insert(160, "\u{00A0}"); // No-Break Space
    map.insert(161, "\u{00A1}"); // Inverted Exclamation Mark
    map.insert(162, "\u{00A2}"); // Cent Sign
    map.insert(163, "\u{00A3}"); // Pound Sign
    map.insert(164, "\u{00A4}"); // Currency Sign
    map.insert(165, "\u{00A5}"); // Yen Sign
    map.insert(166, "\u{00A6}"); // Broken Bar
    map.insert(167, "\u{00A7}"); // Section Sign
    map.insert(168, "\u{00A8}"); // Diaeresis
    map.insert(169, "\u{00A9}"); // Copyright Sign
    map.insert(170, "\u{00AA}"); // Feminine Ordinal Indicator
    map.insert(171, "\u{00AB}"); // Left-Pointing Double Angle Quotation Mark
    map.insert(172, "\u{00AC}"); // Not Sign
    map.insert(173, "\u{00AD}"); // Soft Hyphen
    map.insert(174, "\u{00AE}"); // Registered Sign
    map.insert(175, "\u{00AF}"); // Macron

    map.insert(176, "\u{00B0}"); // Degree Sign
    map.insert(177, "\u{00B1}"); // Plus-Minus Sign
    map.insert(178, "\u{00B2}"); // Superscript Two
    map.insert(179, "\u{00B3}"); // Superscript Three
    map.insert(180, "\u{00B4}"); // Acute Accent
    map.insert(181, "\u{00B5}"); // Micro Sign
    map.insert(182, "\u{00B6}"); // Pilcrow Sign
    map.insert(183, "\u{00B7}"); // Middle Dot
    map.insert(184, "\u{00B8}"); // Cedilla
    map.insert(185, "\u{00B9}"); // Superscript One
    map.insert(186, "\u{00BA}"); // Masculine Ordinal Indicator
    map.insert(187, "\u{00BB}"); // Right-Pointing Double Angle Quotation Mark
    map.insert(188, "\u{00BC}"); // Vulgar Fraction One Quarter
    map.insert(189, "\u{00BD}"); // Vulgar Fraction One Half
    map.insert(190, "\u{00BE}"); // Vulgar Fraction Three Quarters
    map.insert(191, "\u{00BF}"); // Inverted Question Mark

    map.insert(192, "\u{00C0}"); // Latin Capital Letter A With Grave
    map.insert(193, "\u{00C1}"); // Latin Capital Letter A With Acute
    map.insert(194, "\u{00C2}"); // Latin Capital Letter A With Circumflex
    map.insert(195, "\u{00C3}"); // Latin Capital Letter A With Tilde
    map.insert(196, "\u{00C4}"); // Latin Capital Letter A With Diaeresis
    map.insert(197, "\u{00C5}"); // Latin Capital Letter A With Ring Above
    map.insert(198, "\u{00C6}"); // Latin Capital Ligature AE
    map.insert(199, "\u{00C7}"); // Latin Capital Letter C With Cedilla
    map.insert(200, "\u{00C8}"); // Latin Capital Letter E With Grave
    map.insert(201, "\u{00C9}"); // Latin Capital Letter E With Acute
    map.insert(202, "\u{00CA}"); // Latin Capital Letter E With Circumflex
    map.insert(203, "\u{00CB}"); // Latin Capital Letter E With Diaeresis
    map.insert(204, "\u{00CC}"); // Latin Capital Letter I With Grave
    map.insert(205, "\u{00CD}"); // Latin Capital Letter I With Acute
    map.insert(206, "\u{00CE}"); // Latin Capital Letter I With Circumflex
    map.insert(207, "\u{00CF}"); // Latin Capital Letter I With Diaeresis

    map.insert(208, "\u{00D0}"); // Latin Capital Letter Eth
    map.insert(209, "\u{00D1}"); // Latin Capital Letter N With Tilde
    map.insert(210, "\u{00D2}"); // Latin Capital Letter O With Grave
    map.insert(211, "\u{00D3}"); // Latin Capital Letter O With Acute
    map.insert(212, "\u{00D4}"); // Latin Capital Letter O With Circumflex
    map.insert(213, "\u{00D5}"); // Latin Capital Letter O With Tilde
    map.insert(214, "\u{00D6}"); // Latin Capital Letter O With Diaeresis
    map.insert(215, "\u{00D7}"); // Multiplication Sign
    map.insert(216, "\u{00D8}"); // Latin Capital Letter O With Stroke
    map.insert(217, "\u{00D9}"); // Latin Capital Letter U With Grave
    map.insert(218, "\u{00DA}"); // Latin Capital Letter U With Acute
    map.insert(219, "\u{00DB}"); // Latin Capital Letter U With Circumflex
    map.insert(220, "\u{00DC}"); // Latin Capital Letter U With Diaeresis
    map.insert(221, "\u{00DD}"); // Latin Capital Letter Y With Acute
    map.insert(222, "\u{00DE}"); // Latin Capital Letter Thorn
    map.insert(223, "\u{00DF}"); // Latin Small Letter Sharp S

    map.insert(224, "\u{00E0}"); // Latin Small Letter A With Grave
    map.insert(225, "\u{00E1}"); // Latin Small Letter A With Acute
    map.insert(226, "\u{00E2}"); // Latin Small Letter A With Circumflex
    map.insert(227, "\u{00E3}"); // Latin Small Letter A With Tilde
    map.insert(228, "\u{00E4}"); // Latin Small Letter A With Diaeresis
    map.insert(229, "\u{00E5}"); // Latin Small Letter A With Ring Above
    map.insert(230, "\u{00E6}"); // Latin Small Ligature AE
    map.insert(231, "\u{00E7}"); // Latin Small Letter C With Cedilla
    map.insert(232, "\u{00E8}"); // Latin Small Letter E With Grave
    map.insert(233, "\u{00E9}"); // Latin Small Letter E With Acute
    map.insert(234, "\u{00EA}"); // Latin Small Letter E With Circumflex
    map.insert(235, "\u{00EB}"); // Latin Small Letter E With Diaeresis
    map.insert(236, "\u{00EC}"); // Latin Small Letter I With Grave
    map.insert(237, "\u{00ED}"); // Latin Small Letter I With Acute
    map.insert(238, "\u{00EE}"); // Latin Small Letter I With Circumflex
    map.insert(239, "\u{00EF}"); // Latin Small Letter I With Diaeresis

    map.insert(240, "\u{00F0}"); // Latin Small Letter Eth
    map.insert(241, "\u{00F1}"); // Latin Small Letter N With Tilde
    map.insert(242, "\u{00F2}"); // Latin Small Letter O With Grave
    map.insert(243, "\u{00F3}"); // Latin Small Letter O With Acute
    map.insert(244, "\u{00F4}"); // Latin Small Letter O With Circumflex
    map.insert(245, "\u{00F5}"); // Latin Small Letter O With Tilde
    map.insert(246, "\u{00F6}"); // Latin Small Letter O With Diaeresis
    map.insert(247, "\u{00F7}"); // Division Sign
    map.insert(248, "\u{00F8}"); // Latin Small Letter O With Stroke
    map.insert(249, "\u{00F9}"); // Latin Small Letter U With Grave
    map.insert(250, "\u{00FA}"); // Latin Small Letter U With Acute
    map.insert(251, "\u{00FB}"); // Latin Small Letter U With Circumflex
    map.insert(252, "\u{00FC}"); // Latin Small Letter U With Diaeresis
    map.insert(253, "\u{00FD}"); // Latin Small Letter Y With Acute
    map.insert(254, "\u{00FE}"); // Latin Small Letter Thorn
    map.insert(255, "\u{00FF}"); // Latin Small Letter Y With Diaeresis

    map
}
