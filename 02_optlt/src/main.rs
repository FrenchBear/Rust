// optlt - Options lifetime
// Learning Rust
// How to use lifetime storing an external ref to Options in a new Laby structure
// Without liketimes, rust is complaining
//
// 2018-12-16	PV	    First version

// Basic options structure.
// No lifetime is required here
struct Options {
    n: i32,
}

impl Options {
    fn new(n: i32) -> Options {
        Options {
            n: n,
        }
    }
}

// First part, field opt in Laby has same lfetime that Laby structure
struct Laby<'a> {
    opt: &'a Options,
}

// For impl, doc says: 
// §10.4.9 10.4.9 Lifetime Annotations in Method Definitions:
// Lifetime names for struct fields always need to be declared after the impl keyword and then used
// after the struct’s name, because those lifetimes are part of the struct’s type. 
impl<'a> Laby<'a> {
    // Note that "fn new(opt: &'a Options) -> Laby {" also works, probably implicit
    // §10.4.9 10.4.9 Lifetime Annotations in Method Definitions:
    // In method signatures inside the impl block, references might be tied to the lifetime of references 
    // in the struct’s fields, or they might be independent. In addition, the lifetime elision rules often
    // make it so that lifetime annotations aren’t necessary in method signatures. 
    fn new(opt: &Options) -> Laby {
        Laby {
            opt: opt,
        }
    } 
}

fn main() {
    let o = Options::new(5);
    let l = Laby::new(&o);
    println!("l.opt.n: {}", l.opt.n);
}

