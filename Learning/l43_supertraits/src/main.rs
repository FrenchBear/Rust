// l43_supertraits: Learning Rust
//
// 2025-04-16	PV      First version

#![allow(dead_code, unused_variables)]

// Debug is used as a supertrait of Person, do a struct implementing Person must implement debug (or use #[derive(Debug)])
trait Person: core::fmt::Debug {
    fn name(&self) -> String;
}

// Person is a supertrait of Student
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

// Impl

#[derive(Debug)]
struct Etudiant {
    name: String,
    university: String,
    git_username: String,
    fav_language: String,
}

impl Etudiant {
    fn new(name: &str, university: &str, git_username: &str, fav_language: &str) -> Self {
        Etudiant {
            name: name.into(),
            university: university.into(),
            git_username: git_username.into(),
            fav_language: fav_language.into(),
        }
    }
}

impl Person for Etudiant {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for Etudiant {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for Etudiant {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for Etudiant {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn main() {
    let joe = Etudiant::new("Joe", "Harvard", "joe.banana@watermelon.com", "Rust");
    println!("{}", comp_sci_student_greeting(&joe));

    println!();
    test_chien();
}

// -----------------------
// Disambiguating overlapping traits (having member functions with the same name)
// Since each trait is implemented separately, there's no ambiguity during definition

trait Genre {
    fn name(&self) -> String;
}

trait Espèce {
    fn name(&self) -> String;
}

trait SousEspèce {
    fn name(&self) -> String;
}

struct Animal {
    name: String,
    sous_espèce: String,
    espèce: String,
    genre: String,
}

impl Animal {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Genre for Animal {
    fn name(&self) -> String {
        self.genre.clone()
    }
}

impl Espèce for Animal {
    fn name(&self) -> String {
        self.espèce.clone()
    }
}

impl SousEspèce for Animal {
    fn name(&self) -> String {
        self.sous_espèce.clone()
    }
}

fn test_chien() {
    let m = Animal {
        name: "Médor".into(),
        genre: "Canis".to_string(),
        espèce: "Canis lupus".to_string(),
        sous_espèce: "Canis lupus familiaris".to_string(),
    };

    // There are multiple methods name()
    println!("Animal {}:", m.name()); // Be default, it's name() from base type Animal
    // type cast
    println!("  Genre:       {}", <Animal as Genre>::name(&m));
    println!("  Espèce:      {}", <Animal as Espèce>::name(&m));
    // variable cast:
    let se: Box<dyn SousEspèce> = Box::new(m);
    println!("  Sous-espèce: {}", se.name());
}
