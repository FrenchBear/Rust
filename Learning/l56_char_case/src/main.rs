// l56_char_case
// Char case conversions that change length
// Just some curiosity... Note that C#13/.Net9 doesn't care about these, uppercase(ﬀ) -> ﬀ for instance
//
// Only one conversion to_lowercase() returns 2 characters:
// tl: İ -> i̇, len=2
//
// 102 to_uppercase() conversions return 2 chars:
// tu: ß -> SS, len=2
// tu: ŉ -> ʼN, len=2
// tu: ǰ -> J̌, len=2
// tu: ΐ -> Ϊ́, len=3
// tu: ΰ -> Ϋ́, len=3
// tu: և -> ԵՒ, len=2
// ...
// tu: ﬀ -> FF, len=2
// tu: ﬁ -> FI, len=2
// tu: ﬂ -> FL, len=2
// tu: ﬃ -> FFI, len=3
// tu: ﬄ -> FFL, len=3
// tu: ﬅ -> ST, len=2
// tu: ﬆ -> ST, len=2
// ...
//
// 2025_04_28   PV

#![allow(unused)]

fn main() {
    for i in 0..(2 << 21) {
        let c = char::from_u32(i);
        if let Some(c) = c {
            let cl = c.to_lowercase();
            let le = cl.len();
            if le != 1 {
                println!("tl: {c} -> {cl}, len={le}");
            }

            let cu = c.to_uppercase();
            let le = cu.len();
            if le != 1 {
                println!("tu: {c} -> {cu}, len={le}");
            }
        }
    }
}
