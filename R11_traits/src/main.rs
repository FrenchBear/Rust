// R11_traits
// Learning Rust
// 2018-11-10	PV

#![allow(dead_code)]
#![allow(unused_variables)]

pub trait Aboyer {
    fn ouaf(&self);
}

struct Chien {
    nom: String,
    race: String,
}

impl Chien {
    fn new(nom: &str, race: &str) -> Chien {
        Chien{nom: String::from(nom), race: String::from(race)}
    }
}

impl Aboyer for Chien {
    fn ouaf(&self) {
        println!("{}: ouaf!", self.nom);
    }
}

fn woof(a: impl Aboyer) {
    a.ouaf();
}

fn main() {
    let athos = Chien::new("Athos", "Charplanina");
    athos.ouaf();

    let baltik = Chien::new("Baltik", "Charplanina");
    woof(baltik);
}
