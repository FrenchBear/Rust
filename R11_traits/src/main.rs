// R11_traits
// Learning Rust
// 2018-11-10	PV

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]


pub trait Aboyer {
    // Just a prototype (=interface), no default implementation
    fn ouaf(&self);

    // With a default implementation
    fn wif(&self) { 
        println!("wif!");
    }
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

// Simplified declaration
fn woof1(a: &impl Aboyer) {
    a.ouaf();
}

// Normal declaration
fn woof2<T: Aboyer>(a: &T) {
    a.ouaf();
}

// Normal using where
fn woof3<T>(a: &T) where T:Aboyer {
    a.ouaf();
}


fn get_aboyeur() -> impl Aboyer {
    Chien::new("Kim", "Berger allemand")
}

fn main() {
    let chenil;     // Type won't be inferred until initialization

    let athos = Chien::new("Athos", "Charplanina");
    athos.ouaf();
    athos.wif();
    
    let baltik = Chien::new("Baltik", "Charplanina");
    woof1(&baltik);
    woof2(&athos);
    woof3(&get_aboyeur());

    chenil = vec![athos, baltik];

    println!("\nEnChaine");
    println!("{}", EnChaine::en_chaine(&2));
    println!("{}", EnChaine::en_chaine(&3.1416));
    println!("{}", EnChaine::en_chaine(&"hello"));
}



// A simple trait that provides conversion to a String
pub trait EnChaine {
    fn en_chaine(&self) -> String;
}

// Blanket implementation
// Add trait EnChaine and its method en_chaine for any type that has trait Display
impl<T:std::fmt::Display> EnChaine for T {
    fn en_chaine(&self) -> String {
        String::from(format!("{}", self))
    }
}


