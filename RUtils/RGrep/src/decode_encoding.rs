// module decode_encoding
// Check for text files with empirical attempt to detect encoding
//
// 2025-03-13   PV      First version
// 2025-04-01   PV      New version, read only the first 1000 bytes at first for detection, faster for large non-text files

use std::fs::File;
use std::io::{self, BufReader, Read, Seek};
use std::path::Path;

// external crates imports
use encoding_rs::{Encoding, UTF_8, UTF_16LE, WINDOWS_1252};

/// Detects encoding of a text file.
/// Faster version than read_text_file, reads only 1000 bytes max at first to detect encoding and
/// check for heuristics, and read the full file only if this stage is successful. This should be
/// more efficient on large binary files that don't need to be fully loaded to detect content.
/// Contrary to read_text_file, only returns an error in case of io::Error
/// If encoding is recognized, returns content as a Ok(String) and encoding as a string
/// If encoding is not recognized, returns None, and "Not text ?" with ? 1-3 indicates test that failed.
pub fn read_text_file(path: &Path) -> Result<(Option<String>, &str), io::Error> {
    let mut file = File::open(path)?;
    let mut buffer_1000 = [0; 1000];
    // read up to 1000 bytes
    let n = file.read(&mut buffer_1000[..])?;

    let mut buffer_full = Vec::new();
    let mut buffer_full_read = false;

    // Check that string s doesn't contain a nul char and contains at least 90% of ASCII 32..127, CR, LF, TAB
    // Type std::str::Chars<'_> is just an iterable on chars
    fn ok_string2(chars: std::str::Chars<'_>) -> bool {
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
        if len < 10 {
            true
        } else {
            acount * 10 >= 9 * len
        }
    }

    // Define the encodings to try, in order of preference.
    let encodings: [&'static Encoding; 3] = [UTF_8, UTF_16LE, WINDOWS_1252];

    for encoding in encodings {
        let test_buffer = if encoding == UTF_8 {
            // Since we potentially truncated a UTF-8 sequence at the end, we may have to reduce buffer size to avoid a
            // truncated sequence that would render buffer invalid for UTF-8.
            // A quick way is just en ensure that the buffer ends with a byte<128, any value >=128 could be in the middle
            // of a 2-4 bytes sequence. We could check if the sequence is complete or truncated, but the quick way is good enough.
            if n == 1000 {
                let mut pa = 999;
                while pa > 0 && buffer_1000[pa] >= 128 {
                    pa -= 1;
                }
                // No single byte<128 over 1000 bytes, don't bother decode that as UTF-16, not interesting
                if pa == 0 {
                    return Ok((None, "Not text 1"));
                }
                &buffer_1000[..=pa]
            } else {
                &buffer_1000[..n]
            }
        } else if encoding == UTF_16LE {
            // We have to check whether we truncated reading in the middle of a surrogate sequence when readind 1000 bytes max.
            // Lead surrogate is 0xD800-0xDBFF (and tail surrogate is 0xDC00-0xDFFF), if the byte at index 998 is 0xD8, then
            // we cut a surrogate. Note that optional byte order header (0xFF, 0xFE) is two bytes long, so all UTF-16 words
            // start at en even index.
            if n == 1000 {
                let mut pa = 998;
                while pa > 0 && buffer_1000[pa] >= 0xD8 && buffer_1000[pa] <= 0xDB {
                    pa -= 2;
                }
                if pa == 0 {
                    return Ok((None, "Not text 2"));
                }
                &buffer_1000[..=pa]
            } else {
                &buffer_1000[..n]
            }
        } else {
            &buffer_1000[..n]
        };

        let (decoded_string, used_encoding, had_errors) = encoding.decode(test_buffer);

        // If decoding succeeded without errors, return the string.
        if !had_errors && ok_string2(decoded_string.chars()) {
            if n < 1000 {
                return Ok((Some(decoded_string.into_owned()), used_encoding.name()));
            } else {
                if !buffer_full_read {
                    let _ = file.rewind();
                    let mut reader = BufReader::new(&file);
                    reader.read_to_end(&mut buffer_full)?;
                    buffer_full_read = true;
                }

                let (decoded_string, used_encoding, had_errors) = encoding.decode(&buffer_full[..]);
                if !had_errors && ok_string2(decoded_string.chars()) {
                    return Ok((Some(decoded_string.into_owned()), used_encoding.name()));
                }
            }
        }
    }

    // None of the encodings worked without error
    Ok((None, "Not text 3"))
}
