// guessing_game
// Learning rust
//
// 2018-10-12	PV
// 2023-05-01   PV      Restart Rust learning

use std::io;
use rand::Rng;

fn main() {
    println!("Devinez le nombre!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("Le nombre secret est: {secret_number}");                    // Pour tricher :-)

    let mut trial = 0;
    loop {
        trial += 1;
        println!("Entrez votre essai (#{trial}):");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Erreur de lecture");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Vous devriez entrer un nombre entier");
                continue;
            }
        };

        println!("Votre essai: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Trop petit"),
            std::cmp::Ordering::Greater => println!("Trop grand"),
            std::cmp::Ordering::Equal => {
                println!("Gagné!");
                break;
            }
        }
    }

    println!("Vous avez gagné avec {trial} essais");
}
