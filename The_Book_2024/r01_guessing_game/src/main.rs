// r01_guessing_game
// Learning rust
//
// 2024-11-02   PV      Re-restart Rust learning
// 2025-02-21   PV      Comment on continue

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devinez le nombre!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // For dev/debug only!
    //println!("Le nombre secret est: {secret_number}");

    loop {
        println!("Entrez votre essai");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Erreur de lecture");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrez une valeur numérique! entre 1 et 100!");
                continue;       // Rust expects continue o return a u32...  But it returns ! (never return) so it's Ok
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Trop petit!"),
            Ordering::Greater => println!("Trop grand!"),
            Ordering::Equal => {
                println!("Gagné!");
                break;
            }
        }
    }
}
