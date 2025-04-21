// l35_break_path
// Learning Rust, Break a Path in different components
// This is base code, that will panic if any part is missing, it's just demo code to show functions to call
//
// 2025-04-07   PV      First version
// 2025-04-21   PV      Clippy optimizations

use std::path::Path;

fn break_path(p: &Path) {
    let filefp = p.to_str();
    // Here path is None if there's no parent. It's a bit complex because if there's no parent, p.parent() returns Some("")
    let path: Option<&str> = match p.parent() {
        Some(pp) => {
            if pp.as_os_str().is_empty() {
                None
            } else {
                pp.to_str()
            }
        }
        None => None,
    };
    let basename = p.file_name().map(|p| p.to_str().unwrap());
    let stem = p.file_stem().map(|p| p.to_str().unwrap());
    let extension = p.extension().map(|p| p.to_str().unwrap());
    // Prefix extracts drive or UNC prefix or other special prefixes
    let prefix = match p.components().next() {
        Some(std::path::Component::Prefix(pr)) => pr.as_os_str().to_str(),
        _ => None,
    };
    // Basepath is path with prefix removed (or None if there's no path)
    let basepath = match prefix {
        // Some(p) => match path {
        //     Some(pa) => Some(&pa[p.len()..]),
        //     None => None,
        // },
        Some(p) => path.map(|pa| &pa[p.len()..]),
        None => path,
    };

    println!("full name:     {:?}", filefp);
    println!("  path:        {:?}", path);
    println!("    prefix:    {:?}", prefix);
    println!("    basepath:  {:?}", basepath);
    println!("  basename:    {:?}", basename);
    println!("    stem:      {:?}", stem);
    println!("    extension: {:?}", extension);
    println!();
}

fn main() {
    break_path(Path::new(r"C:\path\to\file\example.txt"));
    break_path(Path::new(r"\path\to\file\example.txt"));
    break_path(Path::new(r"path\to\file\example.txt"));
    break_path(Path::new(r"\\server\share\path\to\file\example.txt"));
    break_path(Path::new(r"C:example.txt"));
    break_path(Path::new(r"C:\Temp"));
    break_path(Path::new(r"example.txt"));
    break_path(Path::new(r"example"));
    break_path(Path::new(r""));
}
