// l68_rxls
// Extract large streams of a NTFS file with large ADS
//
// 2025-10-27   PV

// NOTE - UNRELIABLE CODE !!! 
// This program randomly exits with -> Error: The data area passed to a system call is too small. (os error -2147024774)
// For the same input parameter (W:\Livres\@Springer\Religion and Philosophy\Argumentation Theory), sometimes it works,
// sometimes not It's the call to FindFirstStreamW that fails, but the output structure is fixed size
// (https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findfirststreamw) and anyway we don't pass
// the output size as an input parameter, so there is no way to change it. 
// Initially the code used a statically allocated WIN32_FIND_STREAM_DATA structure with cStreamName an array of
// 296*0_u16, this caused frequent errors. Switching to a dynamically allocated structure reduces the problem frequency,
// but it still appears from time to time.
// Looks like reeally a bug to me, though it's difficult to tell if it's a Rust problem, a Windows crate problem, a
// Windows OS problem, a SMB protocol error, or a Synology SMB protocol implementation error, or a Synology ext4
// filesystem error...

// Anyway, since it's really a single-use program, for now I'll teave it as is since I can't find a solution (and Gemini
// suggestions about this but are useless since it trying to allocate more than the fixed size for
// WIN32_FIND_STREAM_DATA structure)

//#![allow(unused)]

use std::fs::File;
use std::io::{self, *};
use std::path::{Path, PathBuf};

mod fa_streams;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // let args = vec![
    //     String::from(r"W:\Livres\@Springer\Intelligent Technologies and Robotics\Control Engineering")
    // ];

    for filename in args {
        println!("{}", filename);
        match process_file(&filename) {
            Ok(_) => {}
            Err(e) => println!("-> Error: {e}"),
        }
    }

    Ok(())
}

fn process_file(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    if !path.is_file() {
        return Err(io::Error::other("*** Not a file"));
    }

    let streams = fa_streams::get_streams_list(path, false)?;

    // Only extract streams of 2KB or more
    for stream in streams.iter() {
        if stream.size >= 2048 {
            // Do stream extaction
            let streampath = PathBuf::from(String::from(filename) + &stream.name);
            let bytes = std::fs::read(&streampath)?;

            let sn = stream.name.replace(":$DATA", "").replace(":", " ");
            let targetname = singlespace_trim(format!("{filename} - {}", sn));
            println!("Extracting «{}»", targetname);

            let targetpath = Path::new(&targetname);
            let mut file = File::create(&targetpath)?;
            file.write(&bytes)?;
            file.flush()?;
        }
    }

    Ok(())
}

// trim string (remove heading and trailing spaces), and remplace groups of 2 or more spaces by a single space
// Note that this is probably very inneficient and costrly, but in this app, it's Ok
fn singlespace_trim(replace: String) -> String {
    replace.trim().split_whitespace().collect::<Vec<&str>>().join(" ")
}
