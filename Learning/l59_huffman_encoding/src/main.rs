// l59_huffman_encoding
// Play with Huffman encoding
//
// 2025-05-10   PV      First version from C#, basically spent 6 hours to convert almost trivial code...

// Difficulties:
//
// Using a trait common to LeafNode and InternalNode: overcomplicated, can't match a trait to instances of actual types,
// you need to add your own discriminant function, and then do unsafe obscure conversions to convert trait -> actual
// struct (see l60_downcast_trait_without_any).
// Bottom line, a trait is not a good candidate to replace an abstract base class.
//
// Using a tree of references, where InternalNode left and right are references doesn't work, you need to create a temp
// InternalNode, but then it doesn't survive to be able to add it to the tree. Solution could be a pseudo-static vector
// with pre-allocated max size to store new InternalNodes and being sure that references stored in the vector won't
// change, but that requires "magic" type coercion... Finally I've decided to store InternalNode directly inside the
// tree, so they have an owner.
//
// Implementing a chained hierarchy with children pointing to Parent is impossible with references (simple &T and
// Option<&T>) and lifetime control, besides the "lifetime cancer", there is no way to create a new parent object and
// update children contained is not possible with simple code. A solution would combine Rc<T>, RefCell<T>, Weak<T>,
// Option<> and other extra-heavy encapsulation of these types, and since they're references, we're back to previous
// issue of a tree of references.
//
// Finally, I've removed the parent reference, and the visitor pattern is building binary encoded representation from
// top to bottom, instead of bottom to top used in C#. Implementation of browsing from LeafNode to root is
// overcomplicated in Rust (and even if my code did compile, it didn't work...). The good news is that the top-down
// approach used here is simpler than C# code, and eliminates parent reference and all lifetimes.
//
// Conclusion: Forget about translating code from a managed memory language to Rust. Lifetimes and ownership block many
// simple constructions allowed by managed memory, so it requires new algorithms a a deep rewrite from scratch using
// different structures and different code and algorithms. It's confusing, frustrating and a huge time waster.
// Short version, porting code from C# to Rust is virtually impossible besides basic code using only static data and
// simple algorithms.

//#![allow(unused)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::time::Instant;

mod tests;

mod huff;
use huff::*;

fn main() {
    //process_file(r"C:\Development\TestFiles\Text\Les secrets d'Hermione.txt", r"c:\temp\outr.txh").expect("err");
    measure_performance().expect("err");
}

#[allow(unused)]
fn basic_test() {
    let s1 = "A_DEAD_DAD_CEDED_A_BAD_BABE_A_BEADED_ABACA_BED";
    process_string(s1, r"c:\temp\outr.txh").expect("err");
    let s2 = decode_encoded_file(r"c:\temp\outr.txh").unwrap(); // expect("Error recoding file");
    println!("\n{s1}\n{s2}");
    assert_eq!(s1, s2);
}

fn measure_performance() -> io::Result<()> {
    let in_file = r"C:\Development\TestFiles\Text\Les secrets d'Hermione.txt";
    //let in_file = r"C:\Development\TestFiles\Text\Harry Potter and the Prisoner of Azkaban.txt";

    println!("Performance measurement");

    let start = Instant::now();
    let s = std::fs::read_to_string(Path::new(in_file))?;
    let tc: Vec<char> = s.chars().collect();
    let elapsed = start.elapsed().as_secs_f64();
    println!("Read {} bytes, {} characters: {:.3}s", s.len(), tc.len(), elapsed);

    let start2 = Instant::now();
    let encodings = build_encodings_dictionary(&tc);
    let elapsed = start2.elapsed().as_secs_f64();
    println!("Build dictionary of {} symbols: {:.3}s", encodings.len(), elapsed);

    let start = Instant::now();
    let encoded_bit_string = get_encoded_bit_string(&tc, &encodings);
    let elapsed = start.elapsed().as_secs_f64();
    println!("Get encoded bit string {} bits: {:.3}s", encoded_bit_string.len(), elapsed);

    let start = Instant::now();
    let res = get_decoded_bit_string(&encoded_bit_string, &encodings);
    let elapsed = start.elapsed().as_secs_f64();
    println!("Decode encoded bit string: {:.3}s", elapsed);

    assert!(s==res);

    Ok(())
}

#[allow(unused)]
fn process_file(in_file: &str, out_file: &str) -> io::Result<()> {
    let s = std::fs::read_to_string(Path::new(in_file))?;
    let _ = process_string(&s, out_file);

    Ok(())
}

fn process_string(s: &str, out_file: &str) -> io::Result<()> {
    let tc: Vec<char> = s.chars().collect();
    let start = Instant::now();
    let encodings = build_encodings_dictionary(&tc);
    let elasped = start.elapsed().as_secs_f64();

    let encoded_bit_string = get_encoded_bit_string(&tc, &encodings);

    // Print encodings table
    println!("Huffman encodings:");
    let mut sorted_keys: Vec<char> = encodings.keys().copied().collect(); // copied() = map(|k| *k)
    sorted_keys.sort_by_key(|&c| encodings[&c].clone());
    sorted_keys.sort_by_key(|&c| encodings[&c].len());
    for k in sorted_keys.iter() {
        println!("{}: {}", char_to_string(*k), encodings[k]);
    }
    println!();

    // Compute some stats
    let source_char_length = s.chars().count();
    let source_byte_length = s.len();
    let max_encoded_symbol_bit_length = encodings.values().map(|e| e.len()).max().unwrap();
    let encoded_bit_length = tc.iter().map(|c| encodings[c].len()).sum();

    println!("{} characters to encode, {} bytes", source_char_length, source_byte_length);
    println!("Original length: {} bits (UTF-8)", source_byte_length * 8);
    println!(
        "Encoded length: {} bits, {:.3} bits per character, {:.1}% of original length",
        encoded_bit_length,
        encoded_bit_length as f64 / source_char_length as f64,
        100.0 * encoded_bit_length as f64 / source_byte_length as f64 / 8.0
    );
    println!("Max encoded bits per symbol: {}", max_encoded_symbol_bit_length);
    println!("Duration: {:.3}s", elasped);

    // Write output file
    let mut file = File::create(out_file)?;

    writeln!(file, "HE 1")?;
    writeln!(file, "SymbolsCount {}", encodings.len())?;
    writeln!(file, "DataLength {}", encoded_bit_length)?;
    writeln!(file, "Begin Encodings")?;
    let mut sorted_keys: Vec<char> = encodings.keys().copied().collect(); // copied() = map(|k| *k)
    sorted_keys.sort_by_key(|&c| encodings[&c].clone());
    sorted_keys.sort_by_key(|&c| encodings[&c].len());
    for k in sorted_keys.iter() {
        writeln!(file, "{}\t{}", char_to_string(*k), encodings[k])?;
    }
    writeln!(file, "End Encodings")?;
    writeln!(file, "Begin Data")?;
    let mut pos = 0;
    while pos < encoded_bit_length {
        let l = if pos + 64 <= encoded_bit_length { 64 } else { encoded_bit_length - pos };
        writeln!(file, "{}", &encoded_bit_string[pos..pos + l])?;
        pos += 64;
    }
    writeln!(file, "End Data")?;

    Ok(())
}

fn char_to_string(c: char) -> String {
    match c {
        '\r' => "<CR>".into(),
        '\n' => "<LF>".into(),
        '\t' => "<Tab>".into(),
        '\0' => "<Nul>".into(),
        x if x < ' ' => format!("<Ctrl+{}>", (64 + c as u8) as char),
        ' ' => "<Space>".into(),
        x if x < '\x7F' => format!("{}", c),
        '\x7F' => "<Del>".into(),
        _ => format!("U+{:04X}", c as i32),
    }
}

fn string_to_char(part: &str) -> char {
    match part {
        "<CR>" => '\r',
        "<LF>" => '\n',
        "<Tab>" => '\t',
        "<Nul>" => '\x00',
        "<Space>" => ' ',
        "<Del>" => '\x7F',
        _ => {
            if let Some(code_hexa) = part.strip_prefix("U+") {
                let val = u32::from_str_radix(code_hexa, 16).unwrap();
                char::from_u32(val).unwrap()
            } else {
                assert_eq!(part.len(), 1);
                part.as_bytes()[0] as char
            }
        }
    }
}

// Helper
macro_rules! get_line {
    ($reader:expr, $line:expr) => {
        $line.clear();
        $reader.read_line(&mut $line)?;
        $line.truncate($line.trim_end().len());
    };
}

#[allow(unused)]
fn decode_encoded_file(file: &str) -> Result<String, io::Error> {
    let mut f = File::open(file)?;
    // Create a BufReader for efficient reading
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    get_line!(reader, line);
    assert_eq!(line, "HE 1");
    get_line!(reader, line);
    let (token, symbols_count) = extract_token_and_value(&line);
    assert_eq!(token, "SymbolsCount");
    get_line!(reader, line);
    let (token, data_length) = extract_token_and_value(&line);
    assert_eq!(token, "DataLength");
    get_line!(reader, line);
    assert_eq!(line, "Begin Encodings");

    let mut encodings: HashMap<char, String> = HashMap::new();
    loop {
        get_line!(reader, line);
        if line == "End Encodings" {
            break;
        }
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        assert_eq!(parts.len(), 2);
        let k = string_to_char(parts[0]);
        let v = String::from(parts[1]);
        encodings.insert(k, v);
    }
    get_line!(reader, line);
    assert_eq!(line, "Begin Data");

    let mut encoded_bit_string = String::new();
    loop {
        get_line!(reader, line);
        if line == "End Data" {
            break;
        }
        encoded_bit_string += line.as_str();
    }

    println!("Symbols count: {}", symbols_count);
    println!("Data length: {}", data_length);
    assert_eq!(data_length, encoded_bit_string.len());

    Ok(get_decoded_bit_string(&encoded_bit_string, &encodings))
}

fn extract_token_and_value<'a>(line: &'a str) -> (&'a str, usize) {
    let parts: Vec<&'a str> = line.split_ascii_whitespace().collect();
    assert_eq!(parts.len(), 2);
    (parts[0], parts[1].parse::<usize>().unwrap())
}
