// l05_Roman1
// Learn Rust again, play with Roman numbers, very first attempt
//
// 2023-05-18   PV
// 2025-04-21   PV      Implemented Display instead of to_string() [clippy suggestion]

#![allow(dead_code)]

use std::fmt::Display;

struct Roman {
    value: i32,
}

impl Roman {
    fn from_int(v: i32) -> Self {
        if v <= 0 || v >= 4000 {
            panic!("Argument out of range 1..3999");
        }

        Roman { value: v }
    }

    // Don't define inherent method to_string(), implement Display instead
    // fn to_string(&self) -> String {...}
}

impl Display for Roman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rs = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let ri = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

        let mut n = self.value;
        let mut s = String::new();
        let mut i = 0;
        while n > 0 {
            while n >= ri[i] {
                s.push_str(rs[i]);
                n -= ri[i];
            }
            i += 1;
        }

        f.write_str(&s)
    }
}

fn main() {
    let r = Roman::from_int(1965);
    println!("1965 = {r}");
    // let s = r.to_string();
    // println!("1965 = {s}");
}
