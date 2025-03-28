// l11a_traits_1
// Learning Rust again
//
// 2023-06-19   PV

#![allow(unused)]

use std::{fmt::Display, iter::Sum};

use l11a_traits_1::{Book, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}\n{}\n", tweet.summarize(), tweet.comment());

    let f = Fruit::new("Fraise", "Rouge", 25.0);
    print_summary_and_comment(&f);

    let b = Book {
        title: "Harry Potter and the Prisonner from Azkaban".to_string(),
        author: "JK Rowling".to_string(),
    };
    print_summary_and_comment(&b);
    print_comment(&b);

    let bs = summarizable(&b);
    //let bt: &impl Summary = &b;       // Not allowed, but the previous line does exactly this...
    bs.comment();


    let pf = Pair::new(1.0, 2.0);
    pf.cmp_display();   // Ok
    let pb = Pair::new(Book {title:"t1".to_string(), author:"a1".to_string()}, Book {title:"t2".to_string(), author:"a2".to_string()});
    //pb.cmp_display();   // Not Ok because Book does not support PartialOrd

    // i32 implements trait Display, so it implements Affichable and its member affiche
    52.affiche();
}

// Trait Bound Syntax
fn print_summary_and_comment<T: Summary>(thing: &T) {
    println!("Summary: {}", thing.summarize());
    println!("Comment: {}", thing.comment());
    println!();
}

// Alternate syntax using where clause
fn print_summary<T>(thing: &T)
where
    T: Summary,
{
    println!("Summary: {}", thing.summarize());
    println!();
}

// Can return a trait...  But only if the code only returns a single type!
fn summarizable(thing: &impl Summary) -> &impl Summary {
    thing
}

// Syntactic sugar for previous Trait Bound Syntax
// A function parameter can use a trait
fn print_comment(thing: &impl Summary) {
    println!("Comment: {}", thing.comment());
    println!();
}

struct Fruit {
    name: String,
    color: String,
    weight: f64,
}

// Comparaisons are exclusively based on weight field
impl Fruit {
    fn new(n: &str, c: &str, w: f64) -> Fruit {
        Fruit {
            name: n.to_string(),
            color: c.to_string(),
            weight: w,
        }
    }
}

impl Summary for Fruit {
    fn summarize(&self) -> String {
        format!(
            "Fruit {}, color {}, weight {}g",
            self.name, self.color, self.weight
        )
    }
}

// Required for PartialOrd
impl std::cmp::PartialEq for Fruit {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl std::cmp::PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        f64::partial_cmp(&self.weight, &other.weight)
    }
}

// ----------------

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This implementation is only available for T supporting PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// ----------------

trait Affichable {
    fn affiche(&self);
}

// Blanket implementation, that is, implement a trait for any type that implement another trait
impl<T:Display> Affichable for T {
    fn affiche(&self) {
        println!("{}", self)
    }
}

fn test_affichable(item: impl Affichable) {     // Takes ownership
    item.affiche();
}

fn demo_affichable() {
    let i=String::from("Hello");
    test_affichable(i);
    //println!("{i}");    // Err borrow of moved value
}