// r11_kollections
// Learning rust 2024, The Book §8, common collections
//
// 2024-11-10   PV
// 2025-04-21   PV      Clippy suggestions

#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

pub mod hashmaps;
pub mod strings;
pub mod vectors;

fn main() {
    // vectors::test_vectors();
    // strings::test_strings();
    // hashmaps::test_hashmaps();

    println!("\n\n---------------------\nExercises\n");
    exercice_median();
    exercise_mod();
    exercise_pig_latin();
    exercise_text_interface();
}

fn exercice_median() {
    let values = vec![3.5, 1.2, 6.6, 3.9, 4.3];
    println!("Median {:?} = {}", values, median(&values)); // expected: 3.9

    let values = vec![3.5, 1.2, 6.6, 3.9, 4.3, 3.7];
    println!("Median {:?} = {}", values, median(&values)); // expected: 3.8 = (3.9+3.7)/2.0
}

fn median(v: &[f64]) -> f64 {
    let l = v.len();
    if l == 0 {
        return f64::NAN;
    }
    let mut v2 = v.to_owned();
    v2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if l % 2 == 0 {
        (v2[l >> 1] + v2[(l >> 1) - 1]) / 2.0
    } else {
        v2[l >> 1]
    }
}

fn exercise_mod() {
    let values = vec![1, 3, 6, 6, 6, 6, 7, 7, 12, 12, 17];
    println!("Mode {:?} = {:?}", &values, mode(&values)); // 6

    let values = vec![1, 2, 3, 4];
    println!("Mode {:?} = {:?}", &values, mode(&values)); // Undefined

    let values = vec![1, 1, 2, 2];
    println!("Mode {:?} = {:?}", &values, mode(&values)); // MultiMode

    let values = vec![1, 1, 1, 1];
    println!("Mode {:?} = {:?}", &values, mode(&values)); // 1

    let values = vec![1];
    println!("Mode {:?} = {:?}", &values, mode(&values)); // 1

    let values = vec![];
    println!("Mode {:?} = {:?}", &values, mode(&values)); // Undefined
}

#[derive(Debug)]
enum ModeResult {
    SingleMode(i32),
    MultiMode,
    Undefined,
}

fn mode(v: &Vec<i32>) -> ModeResult {
    if v.is_empty() {
        return ModeResult::Undefined;
    }; // v is empty

    let mut counter: HashMap<i32, i32> = HashMap::new();
    for value in v {
        let e = counter.entry(*value).or_insert(0);
        *e += 1;
    }

    let mut vc: Vec<(&i32, &i32)> = counter.iter().collect();
    vc.sort_by(|kv1, kv2| kv2.1.cmp(kv1.1)); // Sort by count descending

    if vc.len() == 1 {
        return ModeResult::SingleMode(v[0]);
    } // v contains a single value (must be checked before following test)
    if *vc[0].1 == 1 {
        return ModeResult::Undefined;
    } // Max count=1 and more than 1 different value, mode is not defined
    if vc[1].1 == vc[0].1 {
        return ModeResult::MultiMode;
    } // If the first two max counts are identical, it's a multimode series
    ModeResult::SingleMode(*vc[0].0) // Basic case, just return the most present value
}

fn exercise_pig_latin() {
    pig_latin("apple fisrt");
    pig_latin("Once upon a time was a king and a prince in a remote kingdom");
}

fn pig_latin(sentence: &str) {
    println!("Original sentence: {sentence}");
    print!("Pig latin version: ");
    for word in sentence.split(" ") {
        let c1 = word.chars().next().unwrap().to_ascii_lowercase();
        if "aeiouy".contains(c1) {
            print!("{word}-hay ");
        } else {
            print!("{}-{}ay ", &word[1..], c1);
        }
    }
    println!();
}

fn exercise_text_interface() {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    text_command(&mut m, "Add Sally to Engineering");
    text_command(&mut m, "Add Sarah to Accounting");
    text_command(&mut m, "Add Pierre to Engineering");
    text_command(&mut m, "Add Sophie to Engineering");
    text_command(&mut m, "Add John to Legal");
    text_command(&mut m, "Add Annie to Accounting");
    text_command(&mut m, "Department Accounting");
    text_command(&mut m, "Department Engineering");
    text_command(&mut m, "Department Sales");
    text_command(&mut m, "Company");
}

fn text_command(m: &mut HashMap<String, Vec<String>>, cmd: &str) {
    let ts: Vec<&str> = cmd.split(" ").collect();

    match ts[0] {
        // Add <person> to <department>
        "Add" => {
            assert_eq!(ts.len(), 4);
            assert_eq!(ts[2], "to");
            let person = String::from(ts[1]); // Take a copy to avoid lifetime issues
            let department = String::from(ts[3]);
            let e = m.entry(department).or_default();
            e.push(person);
        }

        // Department <department> -> retrieve a list of all people in a department sorted alphabetically
        "Department" => {
            assert_eq!(ts.len(), 2);
            let department = ts[1];
            let vopt = m.get(department);
            match vopt {
                Some(v) => {
                    println!("Persons in {department}:");
                    let mut persons = v.clone();
                    persons.sort();
                    for person in persons {
                        println!("- {person}");
                    }
                }
                None => println!("There's nobody in department {department}"),
            }
        }

        // Company -> Full list of company members sorted alphabetically
        "Company" => {
            assert_eq!(ts.len(), 1);

            let mut persons: Vec<&str> = Vec::new();
            for pvec in m.values() {
                for p in pvec {
                    persons.push(p);
                }
                // persons.extend(pvec.iter().map(|rs| &rs[..]));       // Same thing, but harder to read
            }
            persons.sort();
            println!("All company persons:");
            for person in persons {
                println!("- {person}");
            }
        }

        _ => panic!("Unsupported command {}", ts[0]),
    };
}
