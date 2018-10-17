// R03_Structures
// Learning Rust
// 2018-10-17	PV

fn main() {
    let x = "5".parse::<i32>().expect("Not a number");
    println!("x={}", x);
    let mut x = "6";
    println!("x={}", x);
    x = "7";
    println!("x={}", x);
    const X: i32 = 8;
    println!("X={}", X);

    // unsigned integers
    let z8 = 0xff_u8;
    println!("z8={}", z8);
    let z16 = 0xffff_u16;
    println!("z16={}", z16);
    let z32 = 0xffff_ffff_u32;
    println!("z32={}", z32);
    let z64 = 0xffff_ffff_ffff_ffff_u64;
    println!("z64={}", z64);
    let z128 = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_u128;
    println!("z128={}", z128);

    // Base
    let _idec = 51966;
    let _idct = 0o377;
    let _ihex = 0xCAFE;
    let _ibin = 0b110011100010;
    let _ibyt = b'A';           // u8
    let _tiby = b"Really?";     // [u8]

    // In debug build:
    // thread 'main' panicked at 'attempt to add with overflow'
    // let z8 = z8 + 1;
    // println!("z8={}", z8);

    // floats
    let r32 = 3.14159265_f32;
    println!("r32={}", r32);
    let r64 = 3.14159265358979323846_f64;
    println!("r64={}", r64);

    let f1 = 0.1 + 0.1 + 0.1;
    println!("f1={}", f1);

    // bools
    let bt: bool = true;
    println!("bt={}", bt);
    let bf = false;
    println!("bf={}", bf);

    // chars = Unicode Scalar Values, from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
    let c1 = 'A';
    let c2 = 'é';
    let c3 = '♫';
    let c4 = '山';
    let c5 = '𝄞';
    let c6 = '🐗';
    print!("c1..6={}{}{}{}{}{}\n", c1, c2, c3, c4, c5, c6);

    // strings
    let s = "Aé♫山𝄞🐗";
    println!("s={}  {}", s, s.len()); // len() = 17 UTF-8 bytes

    // tuples
    let mut tup1: (i32, f64, u8) = (500, 6.4, 1);
    tup1.2 = 2;
    let (_t1, _t2, _t3) = tup1;
    let tup2 = (43, 'ℤ', "🧝 🧝🏽", true, 1.414);
    let _u1 = tup2.0;
    let _u2 = tup2.1;
    let _u3 = tup2.3;

    // arrays of fixed length, data allocated on the stack (use vector to allow shrink/grow size)
    let mut _a: [i32; 5] = [1, 2, 3, 4, 5];
    _a[0] = 0;

    // functions, can be nested
    fn fact(n: i128) -> i128 {
        return if n < 2 { 1 } else { n * fact(n - 1) };
    }

    // Can't be overloaded
    // error[E0428]: the name `fact` is defined multiple times
    // fn fact(s: &str) -> &str { return s; }

    // can be handled in variables
    let factorial = fact;

    println!("32!={}", factorial(32));

    // expressions: a block is an expression, can end with an expression not followed by ;
    let x = {
        let y = 6;
        y
    };
    let _d = if x < 4 { 1 } else { 2 };

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    fn fibo(mut n: u32) -> u128 {
        let mut f1 = 1_u128;
        let mut f2 = 1_u128;
        while n > 2 {
            n -= 1;
            let copy_f1 = f1;
            f1 = f2;
            f2 = copy_f1 + f2;
        }
        return f2;
    }

    let f14 = fibo(14) as f64;
    let f15 = fibo(15) as f64;
    println!("f15/f14={}", f15/f14);

    let phi = (1.0+5f64.sqrt())/2.0;
    println!("φ={}", phi);

}

