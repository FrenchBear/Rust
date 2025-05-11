// Huffman encoding tests
//
// 2025-05-11   PV      First version

#![cfg(test)]

use super::*;

#[test]
fn test_string() {
    let s1 = "Il était un petit navire\r\nQui n'avait jamais navigué\r\nOhé, ohé, matelot\r\n";
    let tc: Vec<char> = s1.chars().collect();
    let encodings = build_encodings_dictionary(&tc);
    let encoded_bit_string = get_encoded_bit_string(&tc, &encodings);
    let decoded_bit_string = get_decoded_bit_string(&encoded_bit_string, &encodings);
    assert_eq!(s1, decoded_bit_string);
}

#[test]
fn test_empty() {
    let s1 = "";
    let tc: Vec<char> = s1.chars().collect();
    let encodings = build_encodings_dictionary(&tc);
    let encoded_bit_string = get_encoded_bit_string(&tc, &encodings);
    let decoded_bit_string = get_decoded_bit_string(&encoded_bit_string, &encodings);
    assert_eq!(s1, decoded_bit_string);
}

// Note that shource file is large, this test takes about 6s
#[test]
fn test_file() -> io::Result<()> {
    let in_file = r"C:\Development\TestFiles\Text\Les secrets d'Hermione.txt";
    let out_file = r"c:\temp\Les secrets d'Hermione.txh";
    let source = std::fs::read_to_string(in_file)?;
    process_file(in_file, out_file).expect("err");
    let res = decode_encoded_file(out_file).expect("err");
    assert!(source == res);

    Ok(())
}
