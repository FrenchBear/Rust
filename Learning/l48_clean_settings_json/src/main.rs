// l48_clean_settings_json: Remove spellright.language and spellright.documentTypes from .vscode/settings.json
//
// 2025-04-21	PV      First version

#![allow(unused)]

use myglob::{MyGlobMatch, MyGlobSearch};
use serde_json::Value;
use std::{fs, io, path::Path};

fn main() {
    let source = r"C:\Development\**\.vscode\settings.json";
    let mgs = MyGlobSearch::build(source).unwrap();
    println!("Source: {source}");

    for ma in mgs.explore_iter() {
        match ma {
            MyGlobMatch::File(pb) => {
                process_file(&pb);
            }

            MyGlobMatch::Dir(_) => {}

            MyGlobMatch::Error(err) => {
                println!("*** MyGlobMatch error {}", err);
            }
        }
    }
}

fn process_file(file_path: &Path) -> io::Result<()> {
    // Read the JSON file
    let contents = fs::read_to_string(file_path)?;
    let mut json_data: Value = serde_json::from_str(&contents)?;

    // Remove the specified keys if they exist
    if let Some(obj) = json_data.as_object_mut() {
        let mut updated = false;

        for key in [
            "spellright.language",
            "spellright.documentTypes",
            "peacock.affectActivityBar",
            "peacock.affectStatusBar",
            "peacock.affectTitleBar",
        ] {
            updated |= obj.remove(key).is_some();
        }

        if updated {
            // Serialize back to JSON with indentation
            let formatted_json = serde_json::to_string_pretty(&json_data)?;

            // println!(
            //     "\n-----------------------------------------\nFile {}:\n{}",
            //     file_path.display(),
            //     formatted_json
            // );
            println!("{}", file_path.display());
            fs::write(file_path, formatted_json)?;
        }
    }

    Ok(())
}
