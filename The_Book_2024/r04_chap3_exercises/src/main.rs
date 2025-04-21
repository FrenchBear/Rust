// r04_chap3_exercises
// Learning rust, exercies at the end of chapter 3:
// - Convert temperatures between Fahrenheit and Celsius.
// - Generate the nth Fibonacci number.
// - Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
//
// 2024-11-04   PV

fn main() {
    let t1 = c_to_f(0.0);
    println!("0°C -> {t1}°F");

    let t2 = c_to_f(100.0);
    println!("100°C -> {t2}°F");

    let t3 = f_to_c(212.0);
    println!("212°F -> {t3}°C\n");

    for i in 0..10 {
        let f = fibo(i);
        println!("F#{i} = {f}");
    }
    println!();

    twelve_days();
}

// Conversion °C -> °F
fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

// Conversion °F -> °C
fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn fibo(n: isize) -> isize {
    if n == 0 {
        0
    } else if n <= 2 {
        1
    } else {
        let mut a: isize = 1;
        let mut b: isize = 1;
        for _ in 2..n {
            (a, b) = (b, a + b);
        }
        b
    }
}

/*
Here are the lyrics:

    On the first day of Christmas my true love sent to me
    A partridge in a pear tree

    On the second day of Christmas my true love sent to me
    Two turtle doves
    And a partridge in a pear tree

    On the third day of Christmas my true love sent to me
    Three French hens, two turtle doves
    And a partridge in a pear tree

    On the fourth day of Christmas my true love sent to me
    Four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree

    On the fifth day of Christmas my true love sent to me
    Five gold rings, four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree

    On the sixth day of Christmas my true love sent to me
    Six geese a laying, five gold rings, four calling birds
    Three French hens, two turtle doves
    And a partridge in a pear tree

    On the seventh day of Christmas my true love sent to me
    Seven swans a swimming, six geese a laying, five gold rings
    Four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree

    On the eighth day of Christmas my true love sent to me
    Eight maids a milking, seven swans a swimming, six geese a laying
    Five gold rings, four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree

    On the ninth day of Christmas my true love sent to me
    Nine drummers drumming
    On the tenth day of Christmas my true love sent to me
    Ten pipers piping

    Nine drummers drumming, ten pipers piping
    Drumming, piping, drumming, piping
    Eight maids a milking, seven swans a swimming, six geese a laying
    Five gold rings, four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree

    On the eleventh day of Christmas my true love sent to me
    Eleven ladies dancing, ten pipers piping, nine drummers drumming
    Eight maids a milking, seven swans a swimming, six geese a laying
    Five gold rings, four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree

    On the twelfth day of Christmas my true love sent to me
    Twelve Lords a leaping, eleven ladies dancing, ten pipers piping
    Nine, drummers drumming, eight maids a milking
    Seven swans a swimming, six geese a laying
    And five gold rings, four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree, and a partridge in a pear tree
 */

fn twelve_days() {
    for i in 1..=12 {
        day(i);
        println!();
    }
}

static GIFTS: [&str; 12] = [
    "partridge in a pear tree", // Starts with "A "" or "And a", managed in code
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a laying",
    "Seven swans a swimming",
    "Eight maids a milking",
    "Nine drummers drumming",
    "Ten pipers piping",
    "Eleven ladies dancing",
    "Twelve Lords a leaping",
];

static NUMERAL_ADJECTIVES: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigthth", "ninth", "tenth",
    "eleventh", "twelveth",
];

fn day(d: usize) {
    let an = NUMERAL_ADJECTIVES[d - 1];
    println!("On the {an} day of Christmas my true love sent to me");
    for j in (1..=d).rev() {
        let g = GIFTS[j - 1];
        if j == 1 && d != 1 {
            print!("And a ");
        } else if j == 1 {
            print!("A ");
        }
        println!("{g}");
    }
}
