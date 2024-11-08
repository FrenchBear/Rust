// r07_options
// Learning rust 2024
//
// 2024-11-08   PV

#![allow(dead_code, unused_variables)]

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(3);

    let y2 = y.expect("None not supported for addition");
    let sum2 = x + y2;
    println!("Sum2: {sum2}");

    let y3 = match y {
        Some(val) => val,
        None => panic!("None not supported for addition"),
    };
    let sum3 = x + y3;
    println!("Sum3: {sum3}");

    // Will take 0 y is None
    let y4 = y.unwrap_or_default();
    let sum4 = x + y4;
    println!("Sum4: {sum4}");

    // Will panic if y is None
    let y5 = y.unwrap();
    let sum5 = x + y5;
    println!("Sum5: {sum5}");

    // if let construct
    if let Some(y6) = y {
        let sum6 = x+y6;
        println!("Sum6: {sum6}");
    } else {        // Else block is optional, without, there's no action when "if let" doesn't match
        println!("Ooops, y is None...");    // Not a panic actually
    }


    // All arms of a match must return the same type
    // A final ; is required to end let statement
    let roll: u8 = 9;
    let res = match roll {
        3 => three(),
        5 => five(),
        _ => 0,
    };

    // But a match statemeny in itself does not require a final ;
    match roll {
        3 => println!("Trois"),
        _ => println!("Autre"),
    }

    // Same for if, a let statement must be closed by a final ;
    let pair = if roll % 2 == 0 {
        true
    } else {
        false
    };

    // But a direct if statement does not
    if roll % 2 == 0 {
        println!("Pair")
    } else {
        println!("Impair")
    }


    let zero = 0;
}

fn three() -> i32 {
    println!("Three");
    3
}

fn five() -> i32 {
    println!("Five");
    5
}
