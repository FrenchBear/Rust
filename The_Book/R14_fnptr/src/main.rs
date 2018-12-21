// fnptr
// Funtions pointers/references in rust
// 2018-12-21	PV

#![allow(non_snake_case)]

// using a reference to Fn trait
fn integ_Fnref(a: f64, b: f64, f: &Fn(f64) -> f64) -> f64 {
    (f(a) + f(b)) / 2.0 * (b - a)
}

// using a function pointer
fn integ_fnptr(a: f64, b: f64, f: fn(f64) -> f64) -> f64 {
    (f(a) + f(b)) / 2.0 * (b - a)
}

fn vio(x: f64) -> f64 {
    2.0 - x
}

struct Memoizer<I, O> {
    func: fn(I) -> O,
    value: Option<O>,
    param: I,
}

impl<I, O> Memoizer<I, O>
where
    I: Copy,
    O: Copy,
{
    fn new(f: fn(i: I) -> O, i: I) -> Memoizer<I, O> {
        Memoizer {
            func: f,
            value: None,
            param: i,
        }
    }

    fn value(self: &mut Memoizer<I, O>) -> O {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.func)(self.param);
                self.value = Some(v);
                v
            }
        }
    }
}

fn triple(x: f64) -> f64 {
    println!("In triple()");
    3.0 * x
}

fn main() {
    let a = integ_Fnref(0.0, 1.0, &|x: f64| -> f64 { x.sqrt() });
    println!("integ {}, {}, {} -> {}", 0, 1, "sqrt", a);

    let a = integ_Fnref(0.0, 1.0, &vio);
    println!("integ {}, {}, {} -> {}", 0, 1, "vio", a);

    let a = integ_fnptr(0.0, 1.0, |x: f64| -> f64 { x.sqrt() });
    println!("integ2 {}, {}, {} -> {}", 0, 1, "sqrt", a);

    let a = integ_fnptr(0.0, 1.0, vio);
    println!("integ2 {}, {}, {} -> {}", 0, 1, "vio", a);

    let mut m = Memoizer::new(triple, 3.0);
    println!("triple(3.0): {}", m.value());
    println!("triple(3.0): {}", m.value());
}
