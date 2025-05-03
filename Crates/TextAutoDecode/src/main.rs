// textautodecode
// Read a text file, automatically detecting encoding
//
// Note: Because of this test app, there's a dependency on MyGlob crate in cargo.toml which is not needed by library; I don't know
// when this crate is used by other apps if MyGlob is also included...
// ToDo: verify.
//
// 2025-05-02   PV      First version

#![allow(unused)]

use encoding_rs as _;
pub use textautodecode::*;
use myglob::{MyGlobMatch, MyGlobSearch};

fn main() {
    println!("TextAutoDecode lib version: {}\n", TextAutoDecode::version());

    let gs = MyGlobSearch::build(r"C:\DocumentsOD\Doc tech\Encodings\prenoms*.txt").unwrap();
    for ma in gs.explore_iter() {
        match ma {
            MyGlobMatch::File(pb) => {
                print!("{:60}", pb.display());

                let r = TextAutoDecode::read_text_file(&pb);
                match r {
                    Ok(tad) => {
                        println!("{:?}", tad.encoding);
                    },
                    Err(e) => println!("*** Error {e}"),
                }
            }
            _ => { }
        }
    }   
}

