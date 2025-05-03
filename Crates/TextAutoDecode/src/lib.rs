// textautodecode library
// Read a text file automatically decoding encoding
//
// 2025-05-02   PV      First version, deep rewrite of decode_encoding module, with bugs fixed and tests
// 2025-05-03   PV      1.0.1 Detection of UTF-16 without BOM is only for files with more than 20 bytes (10 characters)

// ToDo: Add a helper function to determine end of lines style

#![allow(unused_variables, dead_code, unused_imports)]

// I hope that MyGlob is not included in actual library transitive dependencies...
use myglob as _;

// Std library imports
use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek};
use std::path::Path;

// external crates imports
use encoding_rs::{Encoding, UTF_8, UTF_16BE, UTF_16LE, WINDOWS_1252};

// -----------------------------------
// Submodules

mod tests;

// -----------------------------------
// Globals

const LIB_VERSION: &str = "1.0.1";

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
        // read up to 1000 bytes
        let n = file.read(&mut buffer_1000[..])?;

        // Empty file?
        if n == 0 {
            return Ok(TextAutoDecode {
                text: Some(String::new()),
                encoding: TextFileEncoding::Empty,
            });
        }

        let mut buffer_full = Vec::new();
        let mut buffer_full_read = false;

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
                    return Self::final_read(&mut buffer_full_read, &mut buffer_full, &mut file, UTF_8, Some(TextFileEncoding::UTF8BOM));
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
            if let Some(s) = Self::check_utf16(&buffer_1000, n, UTF_16LE) {
                if n < 1000 {
                    return Ok(TextAutoDecode {
                        text: Some(s),
                        encoding: TextFileEncoding::UTF16LEBOM,
                    });
                } else {
                    return Self::final_read(
                        &mut buffer_full_read,
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
            if let Some(s) = Self::check_utf16(&buffer_1000, n, UTF_16BE) {
                if n < 1000 {
                    return Ok(TextAutoDecode {
                        text: Some(s),
                        encoding: TextFileEncoding::UTF16BEBOM,
                    });
                } else {
                    return Self::final_read(
                        &mut buffer_full_read,
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
                let e = if is_ascii_text(s.as_bytes()) {
                    TextFileEncoding::ASCII
                } else {
                    TextFileEncoding::UTF8
                };
                return Ok(TextAutoDecode { text: Some(s), encoding: e });
            } else {
                return Self::final_read(&mut buffer_full_read, &mut buffer_full, &mut file, UTF_8, Some(TextFileEncoding::UTF8));
            }
        }

        // UTF-16 LE? (Windows)
        // Only files with more than 10 characters (20 bytes) are tested and checked for 75% ASCII, or many small binary non text-files will match
        if n > 20 {
            if let Some(s) = Self::check_utf16(&buffer_1000, n, UTF_16LE) {
                if n < 1000 {
                    return Ok(TextAutoDecode {
                        text: Some(s),
                        encoding: TextFileEncoding::UTF16LE,
                    });
                } else {
                    return Self::final_read(
                        &mut buffer_full_read,
                        &mut buffer_full,
                        &mut file,
                        UTF_16LE,
                        Some(TextFileEncoding::UTF16LE),
                    );
                }
            }

            // UTF-16 BE?
            if let Some(s) = Self::check_utf16(&buffer_1000, n, UTF_16BE) {
                if n < 1000 {
                    return Ok(TextAutoDecode {
                        text: Some(s),
                        encoding: TextFileEncoding::UTF16BE,
                    });
                } else {
                    return Self::final_read(
                        &mut buffer_full_read,
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
                    &mut buffer_full_read,
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

    // Check that string s doesn't contain a null char and contains at least 75% of ASCII 32..127, CR, LF, TAB
    // Type std::str::Chars<'_> is just an iterable on chars
    fn is_75percent_ascii(chars: std::str::Chars<'_>) -> bool {
        let mut acount = 0;
        let mut len = 0;
        for c in chars {
            len += 1;
            let b = c as i32;
            if b == 0 {
                return false;
            }
            if (32..128).contains(&b) || b == 9 || b == 10 || b == 13 {
                acount += 1;
            }
        }

        // For very short files, this test is not really relevant. Small file at 10 is empric, could be a bit higher
        if len < 10 { true } else { acount as f64 / len as f64 >= 0.75 }
    }

    fn final_read(
        buffer_full_read: &mut bool,
        buffer_full: &mut Vec<u8>,
        file: &mut File,
        encoding: &'static Encoding,
        my_encoding_opt: Option<TextFileEncoding>,
    ) -> Result<TextAutoDecode, io::Error> {
        if !*buffer_full_read {
            let _ = file.rewind();
            let mut reader = BufReader::new(file);
            reader.read_to_end(buffer_full)?;
            *buffer_full_read = true;
        }

        let (decoded_string, used_encoding, had_errors) = encoding.decode(&buffer_full[..]);

        let mut check_ascii = false;
        let my_encoding = if let Some(e) = my_encoding_opt {
            check_ascii = e == TextFileEncoding::UTF8;
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

        if !had_errors && Self::is_75percent_ascii(decoded_string.chars()) {
            let s = decoded_string.into_owned();
            let e = if check_ascii {
                if is_ascii_text(s.as_bytes()) {
                    TextFileEncoding::ASCII
                } else {
                    TextFileEncoding::UTF8
                }
            } else {
                my_encoding
            };

            return Ok(TextAutoDecode { text: Some(s), encoding: e });
        } else {
            return Ok(TextAutoDecode {
                text: None,
                encoding: TextFileEncoding::NotText,
            });
        }
    }

    fn check_utf8(buffer_1000: &[u8], n: usize) -> Option<Cow<str>> {
        let test_buffer=
            // Since we potentially truncated a UTF-8 sequence at the end, we may have to reduce buffer size to avoid a
            // truncated sequence that would render buffer invalid for UTF-8.
            // A quick way is just to ensure that the buffer ends with a byte<128, any value >=128 could be in the middle
            // of a 2-4 bytes sequence. We could check if the sequence is complete or truncated, but the quick way is good enough.
            if n == 1000 {
                let mut pa = 999;
                while pa > 0 && buffer_1000[pa] >= 128 {
                    pa -= 1;
                }
                // No single byte<128 over 1000 bytes, don't bother decode that as UTF-16, not interesting
                if pa == 0 {
                    return None;
                }
                &buffer_1000[..=pa]
            } else {
                &buffer_1000[..n]
            };

        let (decoded_string, used_encoding, had_errors) = UTF_8.decode(test_buffer);

        // Return decoding succeeded without errors and content is text
        if !had_errors && Self::is_75percent_ascii(decoded_string.chars()) {
            Some(decoded_string)
        } else {
            None
        }
    }

    fn check_utf16(buffer_1000: &[u8], n: usize, encoding: &'static Encoding) -> Option<String> {
        let test_buffer=
            // We have to check whether we truncated reading in the middle of a surrogate sequence when reading 1000 bytes max.
            // Lead surrogate is 0xD800-0xDBFF (and tail surrogate is 0xDC00-0xDFFF), if the byte at index 998 is 0xD8, then
            // we cut a surrogate. Note that optional byte order header (0xFF, 0xFE) is two bytes long, so all UTF-16 words
            // start at even index.
            if n == 1000 {
                let off = if encoding==UTF_16LE {0} else if encoding==UTF_16BE {1} else {unreachable!()};

                let mut pa = 998;
                while pa > 0 && buffer_1000[pa+off] >= 0xD8 && buffer_1000[pa+off] <= 0xDB {
                    pa -= 2;
                }
                if pa == 0 {
                    return None;
                }
                &buffer_1000[..pa+2]
            } else {
                &buffer_1000[..n]
            };
        let (decoded_string, used_encoding, had_errors) = encoding.decode(test_buffer);

        // Return decoding succeeded without errors and content is text
        if !had_errors && Self::is_75percent_ascii(decoded_string.chars()) {
            Some(decoded_string.into_owned())
        } else {
            None
        }
    }

    fn check_eightbit(buffer_1000: &[u8], n: usize) -> Option<String> {
        // 8-bit encodings don't have buffer trucation in the middle of an encoding issue
        let (decoded_string, used_encoding, had_errors) = WINDOWS_1252.decode(buffer_1000);

        // Return decoding succeeded without errors and content is text
        if !had_errors && Self::is_75percent_ascii(decoded_string.chars()) {
            Some(decoded_string.into_owned())
        } else {
            None
        }
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
