// r11_collections/hashmaps.rs
// Learning rust 2024, The Book §8, common collections
//
// 2024-11-10   PV
// 2025-04-21   PV      Clippy suggestions

use std::collections::HashMap;

pub fn test_hashmaps() {
    println!("\ntest_hashmaps");

    // Creating and filling a new HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let v1 = scores[&String::from("Blue")];
    let v2 = scores["Yellow"];

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    let mut dic = HashMap::new();
    dic.insert("Blue", 30);
    dic.insert("Yellow", 40);

    let v3 = dic["Blue"];
    let ix: &str = &String::from("Yellow");
    let v4 = dic[ix];

    println!("{v1}");
    println!("{v2}");
    println!("{v3}");
    println!("{v4}\n");

    // Iterate over each value-pair: (output in arbitrary order)
    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    // Inerting an element with the Copy trait copies value in the map, for non-Copy, ownership is transferred, both for key and value
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Updating a HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Insert with an existing key overwrites the value
    println!("{scores:?}"); // {"Blue": 25}

    // Insert only if the key doesn't exist
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Does not insert
    println!("{scores:?}"); // {"Yellow": 50, "Blue": 10}

    // Updating based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}"); // {"world": 2, "hello": 1, "wonderful": 1}.

    // alt
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let e = map.entry(word);
        *e.or_default() += 1;
    }
    println!("{map:?}"); // {"world": 2, "hello": 1, "wonderful": 1}.
}
