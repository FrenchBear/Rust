// textautodecode library
// Read a text file automatically decoding encoding
//
// 2025-05-02   PV      First version, deep rewrite of decode_encoding module, with bugs fixed and tests
// 2025-05-03   PV      1.0.1 Detection of UTF-16 without BOM is only for files with more than 20 bytes (10 characters)
// 2025-05-06   PV      1.1.0 is_75percent_ascii is only for 8-bit files, use no_binary for other encodings
// 2025-05-06   PV      1.2.0 check_eightbit fixed (was converting the whole buffer_1000 regardless of actual length)
// 2025-06-24   PV      1.3.0 check_utf8 checks correctly for a possibly truncated UTF-8 sequence at the end of a 1000 bytes buffer

#![allow(unused_variables, dead_code, unused_imports)]

// Std library imports
use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek};
use std::path::Path;

// external crates imports
use encoding_rs::{Encoding, UTF_8, UTF_16BE, UTF_16LE, WINDOWS_1252};
use tempfile as _;

// -----------------------------------
// Submodules

mod tests;

// -----------------------------------
// Globals

const LIB_VERSION: &str = "1.3.0";

// -----------------------------------
// Structures

/// Main struct of textautodecode
#[derive(Debug)]
pub struct TextAutoDecode {
    pub text: Option<String>,
    pub encoding: TextFileEncoding,
}

#[derive(Debug, PartialEq)]
pub enum TextFileEncoding {
    NotText,  // Binary or unrecognized text (for instance, contains chars in 0..31 other than \r \n \t)
    Empty,    // File is empty
    ASCII,    // Only 7-bit characters
    EightBit, // ANSI/Windows 1525 or other
    UTF8,
    UTF8BOM,    // Starts with EF BB BF
    UTF16LE,    // No BOM, but UTF-16 LE detected
    UTF16BE,    // No BOM, but UTF-16 BE detected
    UTF16LEBOM, // Starts with FF FE (Windows)
    UTF16BEBOM, // Starts with FE FF
}

// Automatically provide ToString conversion
impl TextAutoDecode {
    pub fn version() -> &'static str {
        LIB_VERSION
    }

    /// Detects encoding of a text file.
    ///
    /// Reads only 1000 bytes max at first to detect encoding and check for heuristics, and read the full file only if
    /// this stage is successful. This is more efficient on large binary files that don't need to be fully loaded to
    /// detect content.
    /// Returns an error in case of io::Error
    /// If encoding is recognized, returns TextAutoDecode with text as a Some(String) and encoding as TextFileEncoding.
    /// If encoding is not recognized, returns TextAutoDecode with text:None, and encoding NotText.
    pub fn read_text_file(path: &Path) -> Result<TextAutoDecode, io::Error> {
        let mut file = File::open(path)?;
        let mut buffer_1000 = [0; 1000];
        // Read up to 1000 bytes
        let n = file.read(&mut buffer_1000[..])?;

        // Empty file?
        if n == 0 {
            return Ok(TextAutoDecode {
                text: Some(String::new()),
                encoding: TextFileEncoding::Empty,
            });
        }

        let mut buffer_full = Vec::new();
        let mut is_buffer_full_read = false;

        // First, check for BOM that will directly indicate encoding. If BOM is present but detection fails, no need to
        // continue testing other possible formats.

        // UTF-8 BOM?
        // Since we have a BOM, no need to check for ASCII subset
        if n >= 3 && buffer_1000[0] == 0xEF && buffer_1000[1] == 0xBB && buffer_1000[2] == 0xBF {
            if let Some(cow) = Self::check_utf8(&buffer_1000, n) {
                if n < 1000 {
                    return Ok(TextAutoDecode {
                        text: Some(cow.into_owned()),
                        encoding: TextFileEncoding::UTF8BOM,
                    });
                } else {
                    return Self::final_read(&mut is_buffer_full_read, &mut buffer_full, &mut file, UTF_8, Some(TextFileEncoding::UTF8BOM));
                }
            } else {
                return Ok(TextAutoDecode {
                    text: None,
                    encoding: TextFileEncoding::NotText,
                });
            }
        }

        // UTF-16 LE BOM? (Windows)
        if n >= 2 && buffer_1000[0] == 0xFF && buffer_1000[1] == 0xFE {
            if let Some(s) = Self::check_utf16(&buffer_1000, n, UTF_16LE, false) {
                if n < 1000 {
                    return Ok(TextAutoDecode {
                        text: Some(s),
                        encoding: TextFileEncoding::UTF16LEBOM,
                    });
                } else {
                    return Self::final_read(
                        &mut is_buffer_full_read,
                        &mut buffer_full,
                        &mut file,
                        UTF_16LE,
                        Some(TextFileEncoding::UTF16LEBOM),
                    );
                }
            } else {
                return Ok(TextAutoDecode {
                    text: None,
                    encoding: TextFileEncoding::NotText,
                });
            }
        }

        // UTF-16 BE BOM?
        if n >= 2 && buffer_1000[0] == 0xFE && buffer_1000[1] == 0xFF {
            if let Some(s) = Self::check_utf16(&buffer_1000, n, UTF_16BE, false) {
                if n < 1000 {
                    return Ok(TextAutoDecode {
                        text: Some(s),
                        encoding: TextFileEncoding::UTF16BEBOM,
                    });
                } else {
                    return Self::final_read(
                        &mut is_buffer_full_read,
                        &mut buffer_full,
                        &mut file,
                        UTF_16LE,
                        Some(TextFileEncoding::UTF16BEBOM),
                    );
                }
            } else {
                return Ok(TextAutoDecode {
                    text: None,
                    encoding: TextFileEncoding::NotText,
                });
            }
        }

        // Then check encodings without BOM
        
        // UTF-8 without BOM?
        // Note that if string is only ASCII text, then type is assumed ASCII instead of UTF-8
        if let Some(cow) = Self::check_utf8(&buffer_1000, n) {
            if n < 1000 {
                let s = cow.into_owned();
                let e = if Self::is_ascii_text(s.as_bytes()) {
                    TextFileEncoding::ASCII
                } else {
                    TextFileEncoding::UTF8
                };
                return Ok(TextAutoDecode { text: Some(s), encoding: e });
            } else {
                // Special case, first 1000 bytes are ASCII so we got there, but after 1000 bytes, we get 8-bit
                // characters so we can't return if we didn't recognize the whole file as UTF-8
                let res = Self::final_read(&mut is_buffer_full_read, &mut buffer_full, &mut file, UTF_8, Some(TextFileEncoding::UTF8));
                match &res {
                    Ok(e) => {
                        if e.encoding != TextFileEncoding::NotText {
                            return res;
                        }
                    }
                    _ => return res,
                }
                // We skip checking UTF-16, since it's a match for UTF-8/ASCII on the furst 1000 chars
                return Self::final_read(
                    &mut is_buffer_full_read,
                    &mut buffer_full,
                    &mut file,
                    WINDOWS_1252,
                    Some(TextFileEncoding::EightBit),
                );
            }
        }

        // UTF-16 LE? (Windows)
        // Only files with more than 10 characters (20 bytes) are tested and checked for 75% ASCII, or many small binary non text-files will match
        if n > 20 {
            if let Some(s) = Self::check_utf16(&buffer_1000, n, UTF_16LE, true) {
                if n < 1000 {
                    return Ok(TextAutoDecode {
                        text: Some(s),
                        encoding: TextFileEncoding::UTF16LE,
                    });
                } else {
                    return Self::final_read(
                        &mut is_buffer_full_read,
                        &mut buffer_full,
                        &mut file,
                        UTF_16LE,
                        Some(TextFileEncoding::UTF16LE),
                    );
                }
            }

            // UTF-16 BE?
            if let Some(s) = Self::check_utf16(&buffer_1000, n, UTF_16BE, true) {
                if n < 1000 {
                    return Ok(TextAutoDecode {
                        text: Some(s),
                        encoding: TextFileEncoding::UTF16BE,
                    });
                } else {
                    return Self::final_read(
                        &mut is_buffer_full_read,
                        &mut buffer_full,
                        &mut file,
                        UTF_16BE,
                        Some(TextFileEncoding::UTF16BE),
                    );
                }
            }
        }

        // 8-bit?
        if let Some(s) = Self::check_eightbit(&buffer_1000, n) {
            if n < 1000 {
                return Ok(TextAutoDecode {
                    text: Some(s),
                    encoding: TextFileEncoding::EightBit,
                });
            } else {
                return Self::final_read(
                    &mut is_buffer_full_read,
                    &mut buffer_full,
                    &mut file,
                    WINDOWS_1252,
                    Some(TextFileEncoding::EightBit),
                );
            }
        }

        // None of the encodings worked without error
        return Ok(TextAutoDecode {
            text: None,
            encoding: TextFileEncoding::NotText,
        });
    }

    // The 75% ASCII test is too restrictive, some valid UTF-8 files are rejected (ex: output of tree command)
    // So we only detect control characters that should not be present in a text file
    // Old text files may contain FF (Form Feed, 12) or VT (Vertical Tab, 11), but it's unlikely for common files
    fn contains_binary_chars(chars: std::str::Chars<'_>, also_check_block_c1: bool) -> bool {
        for c in chars {
            let b = c as i32;
            // In C0 block, only three usual characters are accepted
            if b < 32 && (b != 9 && b != 10 && b != 13) {
                return true;
            }
            // If requested, no characters of C1 is accepted (for all encodings but 8-bit)
            if also_check_block_c1 && b >= 128 && b < 160 {
                return true;
            }
        }

        false
    }

    // Check that string s doesn't contain a null char and contains at least 75% of ASCII 32..127, CR, LF, TAB
    // Type std::str::Chars<'_> is just an iterable on chars
    fn is_75percent_ascii(chars: std::str::Chars<'_>) -> bool {
        let mut acount = 0;
        let mut len = 0;
        for c in chars {
            len += 1;
            let b = c as i32;
            // For 8-bit files, we only exclude non-comon elements of C0 block, and DEL (127) char
            // Anything in [128..255] is accepted
            if b==127 || b < 32 && (b != 9 && b != 10 && b != 13) {
                return false;
            }
            if (32..127).contains(&b) || b == 9 || b == 10 || b == 13 {
                acount += 1;
            }
        }

        // For very short files, this test is not really relevant. Small file at 10 is empric, could be a bit higher
        if len < 10 { true } else { acount as f64 / len as f64 >= 0.75 }
    }

    
    fn final_read(
        is_buffer_full_read: &mut bool,
        buffer_full: &mut Vec<u8>,
        file: &mut File,
        encoding: &'static Encoding,
        my_encoding_opt: Option<TextFileEncoding>,
    ) -> Result<TextAutoDecode, io::Error> {
        if !*is_buffer_full_read {
            let _ = file.rewind();
            let mut reader = BufReader::new(file);
            reader.read_to_end(buffer_full)?;
            *is_buffer_full_read = true;
        }

        let (decoded_string, used_encoding, had_errors) = encoding.decode(&buffer_full[..]);

        // No need to continue if decoding failed
        if had_errors {
            return Ok(TextAutoDecode {
                text: None,
                encoding: TextFileEncoding::NotText,
            });
        }

        let my_encoding = if let Some(e) = my_encoding_opt {
            e
        } else if used_encoding == UTF_8 {
            TextFileEncoding::UTF8
        } else if used_encoding == UTF_16LE {
            TextFileEncoding::UTF16LE
        } else if used_encoding == UTF_16BE {
            TextFileEncoding::UTF16BE
        } else if used_encoding == WINDOWS_1252 {
            TextFileEncoding::EightBit
        } else {
            unreachable!();
        };

        let check_ascii = my_encoding == TextFileEncoding::UTF8;
        let check_75percent_text =
            my_encoding == TextFileEncoding::EightBit || my_encoding == TextFileEncoding::UTF16BE || my_encoding == TextFileEncoding::UTF16BE;

        // Special heuristics to be sure it's a valid text files
        if check_75percent_text && !Self::is_75percent_ascii(decoded_string.chars()) {
            return Ok(TextAutoDecode {
                text: None,
                encoding: TextFileEncoding::NotText,
            });
        }

        if my_encoding != TextFileEncoding::EightBit && Self::contains_binary_chars(decoded_string.chars(), my_encoding == TextFileEncoding::EightBit) {
            return Ok(TextAutoDecode {
                text: None,
                encoding: TextFileEncoding::NotText,
            });
        };

        let s = decoded_string.into_owned();
        let e = if check_ascii {
            if Self::is_ascii_text(s.as_bytes()) {
                TextFileEncoding::ASCII
            } else {
                TextFileEncoding::UTF8
            }
        } else {
            my_encoding
        };

        return Ok(TextAutoDecode { text: Some(s), encoding: e });
    }

    pub fn check_utf8(buffer_1000: &[u8], n: usize) -> Option<Cow<str>> {
        let test_buffer=
            // Since we potentially truncated a UTF-8 sequence at the end, we may have to reduce buffer size to avoid a
            // truncated sequence that would render buffer invalid for UTF-8.
            // A quick way is just to ensure that the buffer ends with a byte<128, any value >=128 could be in the middle
            // of a 2-4 bytes sequence. We could check if the sequence is complete or truncated, but the quick way is good enough.
            if n == 1000 {
                let mut pa = 999;
                loop {
                    // If buffer_1000[pa] is a valid beginning for UTF-8 encoding, we can stop here
                    if buffer_1000[pa]<128 || (buffer_1000[pa] & 0b11100000)==0b11000000 || (buffer_1000[pa] & 0b11110000)==0b11100000 || (buffer_1000[pa] & 0b11111000)==0b11110000 {
                        break
                    }
                    // If it's a continuation character, we can continue, but at most three continuation characters are valid
			        if (buffer_1000[pa] & 0b11000000)==0b10000000 && pa>=997 {
                        pa -= 1;
                        continue
                    }
                    // Sorry, that's not valid UTF-8...
                    return None
                }
                // If last character is <128, it's not truncated and can be kept
                if buffer_1000[pa]<128 {
                    pa+=1;
                }
                &buffer_1000[..pa]
            } else {
                &buffer_1000[..n]
            };

        let (decoded_string, used_encoding, had_errors) = UTF_8.decode(test_buffer);

        // Return decoding succeeded without errors and content is text
        if !had_errors && !Self::contains_binary_chars(decoded_string.chars(), true) {
            Some(decoded_string)
        } else {
            None
        }
    }

    fn check_utf16(buffer_1000: &[u8], n: usize, encoding: &'static Encoding, no_bom: bool) -> Option<String> {
        let test_buffer=
            // We have to check whether we truncated reading in the middle of a surrogate sequence when reading 1000 bytes max.
            // Lead surrogate is 0xD800-0xDBFF (and tail surrogate is 0xDC00-0xDFFF), if the byte at index 998 is 0xD8, then
            // we cut a surrogate. Note that optional byte order header (0xFF, 0xFE) is two bytes long, so all UTF-16 words
            // start at even index.
            if n == 1000 {
                let off = if encoding==UTF_16LE {0} else if encoding==UTF_16BE {1} else {unreachable!()};

                let mut pa = 998;
                if buffer_1000[pa+off] >= 0xD8 && buffer_1000[pa+off] <= 0xDB {
                    pa -= 2;
                }
                &buffer_1000[..pa+2]
            } else {
                &buffer_1000[..n]
            };
        let (decoded_string, used_encoding, had_errors) = encoding.decode(test_buffer);

        if had_errors {
            return None;
        }

        // If there is no BOM, actually UTF-16 BE can be decoded as UTF-16 LE and also the reverse in most of cases.
        // To be sure there is no confusion, add an extra heuristics to check that content is 75% ASCII
        if no_bom && !Self::is_75percent_ascii(decoded_string.chars()) {
            return None;
        }

        // Return decoding succeeded without if there are no binary chars in text (C0 and C1)
        if !Self::contains_binary_chars(decoded_string.chars(), true) {
            Some(decoded_string.into_owned())
        } else {
            None
        }
    }

    fn check_eightbit(buffer_1000: &[u8], n: usize) -> Option<String> {
        // 8-bit encodings don't have buffer trucation in the middle of an encoding issue
        let (decoded_string, used_encoding, had_errors) = WINDOWS_1252.decode(&buffer_1000[..n]);

        // Return decoding succeeded without errors and content is text
        if !had_errors && Self::is_75percent_ascii(decoded_string.chars()) {
            Some(decoded_string.into_owned())
        } else {
            None
        }
    }

    // Stricter version of encoding_rs::mem::is_ascii, presence of chars <32  other than \r, \n or \t are not considered ASCII here
    fn is_ascii_text(bytes: &[u8]) -> bool {
        for &b in bytes.iter() {
            if b > 126 || (b < 32 && b != b'\r' && b != b'\n' && b != b'\t') {
                return false;
            }
        }
        true
    }
}
