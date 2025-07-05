// rtc tests
//
// 2025-04-21   PV

#[cfg(test)]

use crate::*;

#[test]
fn test_empty() -> Result<(), io::Error> {
    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&[])?;
    let mut b = DataBag { ..Default::default() };
    let res = process_file(&mut b, temp_file.path(), Path::new("(test empty)"));

    assert_eq!(res.as_str(), "(test empty): «Empty file»");

    assert_eq!(b.files_types.total, 1);
    assert_eq!(b.files_types.empty, 1);
    assert_eq!(b.files_types.ascii, 0);
    assert_eq!(b.files_types.utf8, 0);
    assert_eq!(b.files_types.utf16, 0);
    assert_eq!(b.files_types.eightbit, 0);
    assert_eq!(b.files_types.nontext, 0);

    assert_eq!(b.eol_styles.total, 0);
    assert_eq!(b.eol_styles.windows, 0);
    assert_eq!(b.eol_styles.unix, 0);
    assert_eq!(b.eol_styles.mac, 0);
    assert_eq!(b.eol_styles.mixed, 0);

    Ok(())
}

#[test]
fn test_ascii() -> Result<(), io::Error> {
    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&[b'H', b'e', b'l', b'l', b'o', b'\r', b'\n'])?;
    let mut b = DataBag { ..Default::default() };
    let res = process_file(&mut b, temp_file.path(), Path::new("(test ascii)"));

    assert_eq!(res.as_str(), "(test ascii): ASCII, Windows");

    assert_eq!(b.files_types.total, 1);
    assert_eq!(b.files_types.empty, 0);
    assert_eq!(b.files_types.ascii, 1);
    assert_eq!(b.files_types.utf8, 0);
    assert_eq!(b.files_types.utf16, 0);
    assert_eq!(b.files_types.eightbit, 0);
    assert_eq!(b.files_types.nontext, 0);

    assert_eq!(b.eol_styles.total, 1);
    assert_eq!(b.eol_styles.windows, 1);
    assert_eq!(b.eol_styles.unix, 0);
    assert_eq!(b.eol_styles.mac, 0);
    assert_eq!(b.eol_styles.mixed, 0);

    Ok(())
}

#[test]
fn test_nontext1() -> Result<(), io::Error> {
    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&[0xCA, 0xFE, 0xDE, 0xAD, 0xBE, 0xEF])?;
    let mut b = DataBag { ..Default::default() };
    let res = process_file(&mut b, temp_file.path(), Path::new("(test non-text)"));

    assert_eq!(res.as_str(), "");

    assert_eq!(b.files_types.total, 1);
    assert_eq!(b.files_types.empty, 0);
    assert_eq!(b.files_types.ascii, 0);
    assert_eq!(b.files_types.utf8, 0);
    assert_eq!(b.files_types.utf16, 0);
    assert_eq!(b.files_types.eightbit, 0);
    assert_eq!(b.files_types.nontext, 1);

    assert_eq!(b.eol_styles.total, 0);
    assert_eq!(b.eol_styles.windows, 0);
    assert_eq!(b.eol_styles.unix, 0);
    assert_eq!(b.eol_styles.mac, 0);
    assert_eq!(b.eol_styles.mixed, 0);

    Ok(())
}

#[test]
fn test_nontext2() -> Result<(), io::Error> {
    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&[0xCA, 0xFE, 0xDE, 0xAD, 0xBE, 0xEF])?;
    let mut b = DataBag { ..Default::default() };
    let res = process_file(&mut b, temp_file.path(), Path::new("non-text.rs"));

    assert_eq!(
        res.as_str(),
        "non-text.rs: «Non-text file detected, but extension rs is usually a text file»"
    );

    assert_eq!(b.files_types.total, 1);
    assert_eq!(b.files_types.empty, 0);
    assert_eq!(b.files_types.ascii, 0);
    assert_eq!(b.files_types.utf8, 0);
    assert_eq!(b.files_types.utf16, 0);
    assert_eq!(b.files_types.eightbit, 0);
    assert_eq!(b.files_types.nontext, 1);

    assert_eq!(b.eol_styles.total, 0);
    assert_eq!(b.eol_styles.windows, 0);
    assert_eq!(b.eol_styles.unix, 0);
    assert_eq!(b.eol_styles.mac, 0);
    assert_eq!(b.eol_styles.mixed, 0);

    Ok(())
}

#[test]
fn test_utf8() -> Result<(), io::Error> {
    let model: [u8; 17] = [
        0x41, // A
        0xC3, 0xA9, // é
        0xE2, 0x99, 0xAB, // ♫
        0xE5, 0xB1, 0xB1, // 山
        0xF0, 0x9D, 0x84, 0x9E, // 𝄞
        0xF0, 0x9F, 0x90, 0x97, // 🐗
    ];

    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&model)?;
    let mut b = DataBag { ..Default::default() };
    let res = process_file(&mut b, temp_file.path(), Path::new("(test utf8)"));

    assert_eq!(res.as_str(), "(test utf8): UTF-8, No EOL detected");

    assert_eq!(b.files_types.total, 1);
    assert_eq!(b.files_types.empty, 0);
    assert_eq!(b.files_types.ascii, 0);
    assert_eq!(b.files_types.utf8, 1);
    assert_eq!(b.files_types.utf16, 0);
    assert_eq!(b.files_types.eightbit, 0);
    assert_eq!(b.files_types.nontext, 0);

    assert_eq!(b.eol_styles.total, 0);
    assert_eq!(b.eol_styles.windows, 0);
    assert_eq!(b.eol_styles.unix, 0);
    assert_eq!(b.eol_styles.mac, 0);
    assert_eq!(b.eol_styles.mixed, 0);

    Ok(())
}

#[test]
fn test_utf8bom() -> Result<(), io::Error> {
    let model: [u8; 5] = [
        0xEF, 0xBB, 0xBF,  // UTF-8 BOM
        0x41,  // A
        b'\r', // Mac EOL
    ];

    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&model)?;
    let mut b = DataBag { ..Default::default() };
    let res = process_file(&mut b, temp_file.path(), Path::new("(test utf8bom)"));

    assert_eq!(res.as_str(), "(test utf8bom): UTF-8 «with BOM», Mac");

    assert_eq!(b.files_types.total, 1);
    assert_eq!(b.files_types.empty, 0);
    assert_eq!(b.files_types.ascii, 0);
    assert_eq!(b.files_types.utf8, 1);
    assert_eq!(b.files_types.utf16, 0);
    assert_eq!(b.files_types.eightbit, 0);
    assert_eq!(b.files_types.nontext, 0);

    assert_eq!(b.eol_styles.total, 1);
    assert_eq!(b.eol_styles.windows, 0);
    assert_eq!(b.eol_styles.unix, 0);
    assert_eq!(b.eol_styles.mac, 1);
    assert_eq!(b.eol_styles.mixed, 0);

    Ok(())
}

#[test]
fn test_utf16lebom() -> Result<(), io::Error> {
    let model: [u8; 16] = [
        0xFF, 0xFE, // UTF-16 LE BOM
        0x41, 0x00, // A
        0x42, 0x00, // B
        b'\n', 0x00, // Unix EOL
        0x43, 0x00, // C
        0x44, 0x00, // D
        b'\r', 0x00, b'\n', 0x00, // Windows EOL
    ];

    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&model)?;
    let mut b = DataBag { ..Default::default() };
    let res = process_file(&mut b, temp_file.path(), Path::new("(test utf16lebom)"));

    assert_eq!(res.as_str(), "(test utf16lebom): UTF-16 LE, «Mixed EOL styles»");

    assert_eq!(b.files_types.total, 1);
    assert_eq!(b.files_types.empty, 0);
    assert_eq!(b.files_types.ascii, 0);
    assert_eq!(b.files_types.utf8, 0);
    assert_eq!(b.files_types.utf16, 1);
    assert_eq!(b.files_types.eightbit, 0);
    assert_eq!(b.files_types.nontext, 0);

    assert_eq!(b.eol_styles.total, 1);
    assert_eq!(b.eol_styles.windows, 1);
    assert_eq!(b.eol_styles.unix, 1);
    assert_eq!(b.eol_styles.mac, 0);
    assert_eq!(b.eol_styles.mixed, 1);

    Ok(())
}

#[test]
fn test_utf16le1() -> Result<(), io::Error> {
    let model: [u8; 24] = [
        // Nooed more than 20 bytes
        0x41, 0x00, // A
        0x42, 0x00, // B
        0x43, 0x00, // C
        0x44, 0x00, // D
        0x45, 0x00, // E
        b'\n', 0x00, // Unix EOL
        0x61, 0x00, // a
        0x62, 0x00, // b
        0x63, 0x00, // c
        0x64, 0x00, // d
        0x65, 0x00, // e
        b'\n', 0x00, // Unix EOL
    ];

    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&model)?;
    let mut b = DataBag { ..Default::default() };
    let res = process_file(&mut b, temp_file.path(), Path::new("(test utf16le1)"));

    assert_eq!(res.as_str(), "(test utf16le1): UTF-16 LE «without BOM», Unix");

    assert_eq!(b.files_types.total, 1);
    assert_eq!(b.files_types.empty, 0);
    assert_eq!(b.files_types.ascii, 0);
    assert_eq!(b.files_types.utf8, 0);
    assert_eq!(b.files_types.utf16, 1);
    assert_eq!(b.files_types.eightbit, 0);
    assert_eq!(b.files_types.nontext, 0);

    assert_eq!(b.eol_styles.total, 1);
    assert_eq!(b.eol_styles.windows, 0);
    assert_eq!(b.eol_styles.unix, 1);
    assert_eq!(b.eol_styles.mac, 0);
    assert_eq!(b.eol_styles.mixed, 0);

    Ok(())
}

#[test]
fn test_utf16le2() -> Result<(), io::Error> {
    let model: [u8; 8] = [
        // <= 20 bytes, won't be recognized as a valid UTF-16 file
        0x41, 0x00, // A
        b'\n', 0x00, // Unix EOL
        0x61, 0x00, // a
        b'\n', 0x00, // Unix EOL
    ];

    let mut temp_file = Builder::new().tempfile()?;
    temp_file.write_all(&model)?;
    let mut b = DataBag { ..Default::default() };
    let res = process_file(&mut b, temp_file.path(), Path::new("(test utf16le1)"));

    assert_eq!(res.as_str(), "");

    assert_eq!(b.files_types.total, 1);
    assert_eq!(b.files_types.empty, 0);
    assert_eq!(b.files_types.ascii, 0);
    assert_eq!(b.files_types.utf8, 0);
    assert_eq!(b.files_types.utf16, 0);
    assert_eq!(b.files_types.eightbit, 0);
    assert_eq!(b.files_types.nontext, 1);

    assert_eq!(b.eol_styles.total, 0);
    assert_eq!(b.eol_styles.windows, 0);
    assert_eq!(b.eol_styles.unix, 0);
    assert_eq!(b.eol_styles.mac, 0);
    assert_eq!(b.eol_styles.mixed, 0);

    Ok(())
}
