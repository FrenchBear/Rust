// MyGlob tests - search
// Unit tests for MyGlob
//
// 2025-04-09   PV
// 2025-04-23   PV      Added search_error tests

#![cfg(test)]
use crate::*;
use std::fs::File;
use std::io::{self, Write};

fn create_directory(path: &str) -> io::Result<()> {
    let p = Path::new(path);
    fs::create_dir_all(p)?; // create_dir_all creates parent dirs if needed.
    Ok(())
}

fn create_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn search_count_base(resgs: Result<MyGlobSearch, MyGlobError>) -> (usize, usize) {
    let mut nf = 0;
    let mut nd = 0;

    match resgs {
        Ok(gs) => {
            for ma in gs.explore_iter() {
                match ma {
                    MyGlobMatch::File(pb) => {
                        println!("{}", pb.display());
                        nf += 1;
                    }
                    MyGlobMatch::Dir(pb) => {
                        println!("{}\\", pb.display());
                        nd += 1;
                    }
                    MyGlobMatch::Error(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }

    (nf, nd)
}

fn search_count(glob_pattern: &str) -> (usize, usize) {
    search_count_base(MyGlobSearch::build(glob_pattern))
}

fn search_count_autorecurse(glob_pattern: &str) -> (usize, usize) {
    search_count_base(MyGlobSearch::new(glob_pattern).autorecurse(true).compile())
}

fn search_count_ignore(glob_pattern: &str, ignore_dirs: &[&str]) -> (usize, usize) {
    let mut builder = MyGlobSearch::new(glob_pattern);
    for ignore_dir in ignore_dirs {
        builder = builder.add_ignore_dir(ignore_dir);
    }

    search_count_base(builder.compile())
}

#[test]
fn search_1() -> io::Result<()> {
    create_directory(r"C:\Temp\search1")?;
    create_file(r"C:\Temp\search1\fruits et légumes.txt", "Des fruits et des légumes")?;
    create_file(r"C:\Temp\search1\info", "Information")?;
    create_directory(r"C:\Temp\search1\fruits")?;
    create_file(r"C:\Temp\search1\fruits\pomme.txt", "Pomme")?;
    create_file(r"C:\Temp\search1\fruits\poire.txt", "Poire")?;
    create_file(r"C:\Temp\search1\fruits\ananas.txt", "Ananas")?;
    create_file(r"C:\Temp\search1\fruits\tomate.txt", "Tomate")?;
    create_directory(r"C:\Temp\search1\légumes")?;
    create_file(r"C:\Temp\search1\légumes\épinard.txt", "Épinard")?;
    create_file(r"C:\Temp\search1\légumes\tomate.txt", "Tomate")?;
    create_file(r"C:\Temp\search1\légumes\pomme.de.terre.txt", "Pomme de terre")?;

    assert_eq!(search_count(r"C:\Temp\search1\info"), (1, 0));
    assert_eq!(search_count(r"C:\Temp\search1\*"), (2, 2));
    assert_eq!(search_count(r"C:\Temp\search1\*.*"), (1, 0));
    assert_eq!(search_count(r"C:\Temp\search1\fruits\*"), (4, 0));
    assert_eq!(search_count(r"C:\Temp\search1\{fruits,légumes}\p*"), (3, 0));
    assert_eq!(search_count(r"C:\Temp\search1\**\p*"), (3, 0));
    assert_eq!(search_count(r"C:\Temp\search1\**\*.txt"), (8, 0));
    assert_eq!(search_count(r"C:\Temp\search1\**\*.*.*"), (1, 0));
    assert_eq!(search_count(r"C:\Temp\search1\légumes\*"), (3, 0));
    assert_eq!(search_count(r"C:\Temp\search1\*s\to[a-z]a{r,s,t}e.t[xX]t"), (2, 0));

    // Testing autorecurse
    assert_eq!(search_count(r"C:\Temp\search1\*.txt"), (1, 0));
    assert_eq!(search_count_autorecurse(r"C:\Temp\search1\*.txt"), (8, 0));
    assert_eq!(search_count(r"C:\Temp\search1"), (0, 1));
    assert_eq!(search_count_autorecurse(r"C:\Temp\search1"), (9, 2));

    // Testing ignore
    assert_eq!(search_count_ignore(r"C:\Temp\search1\**\*.txt", &["Légumes"]), (5, 0));

    fs::remove_dir_all(r"C:\Temp\search1")?;

    Ok(())
}

#[test]
fn search_error_1() {
    let e = MyGlobSearch::build(r"C:\**z\\z");
    assert!(matches!(e.unwrap_err(), MyGlobError::GlobError(..)));
}

#[test]
fn search_error_2() {
    let e = MyGlobSearch::build(r"C:\[\d&&\p{ascii]");
    assert!(matches!(e.unwrap_err(), MyGlobError::RegexError(..)));
}
