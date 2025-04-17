// l43_supertraits: Learning Rust
//
// 2025-04-16	PV      First version

#![allow(dead_code, unused_variables)]

trait Person {
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

// Implem

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
}
