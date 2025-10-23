// MyGlob tests - test_search
// Unit tests for MyGlob
//
// 2025-04-09   PV
// 2025-04-23   PV      Added search_error tests
// 2025-07-13   PV      Tests with chinese characters
// 2025-09-06   PV      Tests max_depth
// 2025-10-22   PV      search2 for v2.0 with link support and maxdepth fixed

#![cfg(test)]
use crate::*;
use std::fs::File;
use std::io::{self, Write};
#[cfg(windows)]
use std::os::windows::fs as os_fs;

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
                    MyGlobMatch::File(_pb) => {
                        println!("{}", _pb.display());
                        nf += 1;
                    }
                    MyGlobMatch::Dir(_pb) => {
                        println!("{}\\", _pb.display());
                        nd += 1;
                    }
                    MyGlobMatch::Error(_e) => {
                        println!("{}", _e);
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

fn search_count1(glob_pattern: &str) -> (usize, usize) {
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

fn search_count_max_depth(glob_pattern: &str, max_depth: usize) -> (usize, usize) {
    search_count_base(MyGlobSearch::new(glob_pattern).max_depth(max_depth).compile())
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
    create_directory(r"C:\Temp\search1\我爱你")?;
    create_file(r"C:\Temp\search1\我爱你\你好世界.txt", "Hello world")?;
    create_file(r"C:\Temp\search1\我爱你\tomate.txt", "Hello Tomate")?;
    create_directory(r"C:\Temp\search1\我爱你\Ƥḭҽɾɾҽ ѵìǫłҽղէ")?;
    create_file(r"C:\Temp\search1\我爱你\Ƥḭҽɾɾҽ ѵìǫłҽղէ\tomate.txt", "Hello Tomate")?;
    create_file(r"C:\Temp\search1\我爱你\Ƥḭҽɾɾҽ ѵìǫłҽղէ\Aé♫山𝄞🐗.txt", "Random 1")?;
    create_file(r"C:\Temp\search1\我爱你\Ƥḭҽɾɾҽ ѵìǫłҽղէ\œæĳøß≤≠Ⅷﬁﬆ.txt", "Random 2")?;

    // Basic testing
    assert_eq!(search_count1(r"C:\Temp\search1\info"), (1, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\*"), (2, 3));
    assert_eq!(search_count1(r"C:\Temp\search1\*.*"), (1, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\fruits\*"), (4, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\{fruits,légumes}\p*"), (3, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\**\p*"), (3, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\**\*.txt"), (13, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\**\*.*.*"), (1, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\légumes\*"), (3, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\*s\to[a-z]a{r,s,t}e.t[xX]t"), (2, 0));

    // Multibyte characters
    assert_eq!(search_count1(r"C:\Temp\search1\**\*爱*\*a*.txt"), (1, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\**\*爱*\**\*a*.txt"), (3, 0));
    assert_eq!(search_count1(r"C:\Temp\search1\我爱你\**\*🐗*"), (1, 0));

    // Testing autorecurse
    assert_eq!(search_count1(r"C:\Temp\search1\*.txt"), (1, 0));
    assert_eq!(search_count_autorecurse(r"C:\Temp\search1\*.txt"), (13, 0));
    assert_eq!(search_count1(r"C:\Temp\search1"), (0, 1));
    assert_eq!(search_count_autorecurse(r"C:\Temp\search1"), (14, 4));
    assert_eq!(search_count_autorecurse(r"C:\Temp\search1\"), (14, 4)); // Test with final \

    // Testing ignore
    assert_eq!(search_count_ignore(r"C:\Temp\search1\**\*.txt", &["Légumes"]), (10, 0));
    assert_eq!(search_count_ignore(r"C:\Temp\search1\**\*.txt", &["Légumes", "我爱你"]), (5, 0));

    // Testing max_depth
    assert_eq!(search_count_max_depth(r"C:\Temp\search1\**\*.txt", 1), (1, 0));
    assert_eq!(search_count_max_depth(r"C:\Temp\search1\**\*.txt", 2), (10, 0));

    // Cleanup
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

#[test]
fn search_error_3() {
    let e = MyGlobSearch::build(r"C:\[Hello");
    assert!(matches!(e.unwrap_err(), MyGlobError::GlobError(..)));
}

#[test]
fn search_error_4() {
    let e = MyGlobSearch::build("");
    assert!(e.is_ok());
    let gs = e.unwrap();
    assert_eq!(gs.root, ".");
    assert_eq!(gs.segments.len(), 1);
    assert!(matches!(gs.segments[0], Segment::Filter(..)));
}

fn search_count2(glob_pattern: &str, max_depth: usize, link_mode: usize) -> (usize, usize) {
    let resgs = MyGlobSearch::new(glob_pattern).autorecurse(true).max_depth(max_depth).set_link_mode(link_mode).compile();
    search_count_base(resgs)
}

#[test]
fn zsearch_1() -> io::Result<()> {
    // create_directory(r"C:\Temp\search1")?;
    // create_file(r"C:\Temp\search1\fruits et légumes.txt", "Des fruits et des légumes")?;
    // create_file(r"C:\Temp\search1\info", "Information")?;
    // create_directory(r"C:\Temp\search1\fruits")?;
    // create_file(r"C:\Temp\search1\fruits\pomme.txt", "Pomme")?;
    // create_file(r"C:\Temp\search1\fruits\poire.txt", "Poire")?;
    // create_file(r"C:\Temp\search1\fruits\ananas.txt", "Ananas")?;
    // create_file(r"C:\Temp\search1\fruits\tomate.txt", "Tomate")?;
    // create_directory(r"C:\Temp\search1\légumes")?;
    // create_file(r"C:\Temp\search1\légumes\épinard.txt", "Épinard")?;
    // create_file(r"C:\Temp\search1\légumes\tomate.txt", "Tomate")?;
    // create_file(r"C:\Temp\search1\légumes\pomme.de.terre.txt", "Pomme de terre")?;
    // create_directory(r"C:\Temp\search1\我爱你")?;
    // create_file(r"C:\Temp\search1\我爱你\你好世界.txt", "Hello world")?;
    // create_file(r"C:\Temp\search1\我爱你\tomate.txt", "Hello Tomate")?;
    // create_directory(r"C:\Temp\search1\我爱你\Ƥḭҽɾɾҽ ѵìǫłҽղէ")?;
    // create_file(r"C:\Temp\search1\我爱你\Ƥḭҽɾɾҽ ѵìǫłҽղէ\tomate.txt", "Hello Tomate")?;
    // create_file(r"C:\Temp\search1\我爱你\Ƥḭҽɾɾҽ ѵìǫłҽղէ\Aé♫山𝄞🐗.txt", "Random 1")?;
    // create_file(r"C:\Temp\search1\我爱你\Ƥḭҽɾɾҽ ѵìǫłҽղէ\œæĳøß≤≠Ⅷﬁﬆ.txt", "Random 2")?;

    // // Basic testing
    // assert_eq!(search_count1(r"C:\Temp\search1\info"), (1, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\*"), (2, 3));
    // assert_eq!(search_count1(r"C:\Temp\search1\*.*"), (1, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\fruits\*"), (4, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\{fruits,légumes}\p*"), (3, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\**\p*"), (3, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\**\*.txt"), (13, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\**\*.*.*"), (1, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\légumes\*"), (3, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\*s\to[a-z]a{r,s,t}e.t[xX]t"), (2, 0));

    // // Multibyte characters
    // assert_eq!(search_count1(r"C:\Temp\search1\**\*爱*\*a*.txt"), (1, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\**\*爱*\**\*a*.txt"), (3, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1\我爱你\**\*🐗*"), (1, 0));

    // // Testing autorecurse
    // assert_eq!(search_count1(r"C:\Temp\search1\*.txt"), (1, 0));
    // assert_eq!(search_count_autorecurse(r"C:\Temp\search1\*.txt"), (13, 0));
    // assert_eq!(search_count1(r"C:\Temp\search1"), (0, 1));
    // assert_eq!(search_count_autorecurse(r"C:\Temp\search1"), (14, 4));
    // assert_eq!(search_count_autorecurse(r"C:\Temp\search1\"), (14, 4)); // Test with final \

    // // Testing ignore
    // assert_eq!(search_count_ignore(r"C:\Temp\search1\**\*.txt", &["Légumes"]), (10, 0));
    // assert_eq!(search_count_ignore(r"C:\Temp\search1\**\*.txt", &["Légumes", "我爱你"]), (5, 0));

    // // Testing max_depth
    // assert_eq!(search_count_max_depth(r"C:\Temp\search1\**\*.txt", 1), (1, 0));
    // assert_eq!(search_count_max_depth(r"C:\Temp\search1\**\*.txt", 2), (10, 0));

    // Cleanup
    //fs::remove_dir_all(r"C:\Temp\search1")?;

    Ok(())
}


#[test]
fn zsearch_2() -> io::Result<()> {
    // Setup directories and a file to be linked to
    create_directory(r"C:\Temp\search3")?;
    create_file(r"C:\Temp\search3\File_L0_original.txt", "Content of File_L0_original.txt")?;
    create_directory(r"C:\Temp\search3\SubDirOriginal")?;
    create_file(r"C:\Temp\search3\SubDirOriginal\File_L1_original.txt", "Content of File_L1_original.txt")?;
    create_directory(r"C:\Temp\search3\SubDirOriginal\AnotherSubLevel")?;
    create_file(
        r"C:\Temp\search3\SubDirOriginal\AnotherSubLevel\File_L2_original.txt",
        "Content of File_L2_original.txt",
    )?;

    create_directory(r"C:\Temp\search2")?;
    create_file(r"C:\Temp\search2\file_S0.txt", "Hello")?;
    create_directory(r"C:\Temp\search2\RealSubDir")?;
    create_file(r"C:\Temp\search2\RealSubDir\file_S1.txt", "Hello world")?;
    create_directory(r"C:\Temp\search2\RealSubDir\Cave")?;
    create_file(r"C:\Temp\search2\RealSubDir\Cave\file_S2.txt", "Cave file")?;

    // Create a symbolic link on Windows, equivalent to:
    // mklink C:\Temp\search2\File_L0.txt C:\Temp\search3\File_L0_original.txt
    //#[cfg(windows)]
    if !Path::new(r"C:\Temp\search2\File_L0.txt").exists() {
        os_fs::symlink_file(r"C:\Temp\search3\File_L0_original.txt", r"C:\Temp\search2\File_L0.txt")?;
    }
    if !Path::new(r"C:\Temp\search2\SubDirLink").exists() {
        os_fs::symlink_dir(r"C:\Temp\search3\SubDirOriginal", r"C:\Temp\search2\SubDirLink")?;
    }
    if !Path::new(r"C:\Temp\search2\RealSubDir\Cave\File_L2.txt").exists() {
        os_fs::symlink_file(r"C:\Temp\search3\File_L0_original.txt", r"C:\Temp\search2\RealSubDir\Cave\File_L2.txt")?;
    }

    // // max_depth 0
    // assert_eq!(search_count2(r"C:\Temp\search2", 0, 0), (3, 2));
    assert_eq!(search_count2(r"C:\Temp\search2", 0, 1), (5, 3));
    // assert_eq!(search_count2(r"C:\Temp\search2", 0, 2), (7, 4));

    // // max_depth 1
    // assert_eq!(search_count2(r"C:\Temp\search2", 1, 0), (1, 1));
    // assert_eq!(search_count2(r"C:\Temp\search2", 1, 1), (2, 2));
    // assert_eq!(search_count2(r"C:\Temp\search2", 1, 2), (2, 2));

    // // max_depth 2
    // assert_eq!(search_count2(r"C:\Temp\search2", 2, 0), (2, 2));
    // assert_eq!(search_count2(r"C:\Temp\search2", 2, 1), (3, 3));
    // assert_eq!(search_count2(r"C:\Temp\search2", 2, 2), (4, 4));

    // Cleanup
    fs::remove_dir_all(r"C:\Temp\search2")?;
    fs::remove_dir_all(r"C:\Temp\search3")?;

    Ok(())
}
