// r28_patterns_matching
// Learning rust 2024, Patterns & matching
//
// 2025-02-15   PV

#![allow(unused)]

fn main() {
    // match arms, mut be exhaustive
    let x = Some(5);
    let y = match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    // Conditional if let, does not need to be exhaustive
    // Can mix if let, else, else if let and else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // While let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // For loops
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // let statements (let PATTERN = EXPRESSION
    let (x, y, z) = (1, 2, 3);

    // Function parameters (also work with closures)
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }
    let point = (3, 5);
    print_coordinates(&point);

    let x = Some(42);
    // let with refutable pattern and else clause
    let Some(y) = x else { panic!("End of the world") };
    // if let with refutable pattern, if pattern doesn't match, ignore.
    if let Some(y) = x { println!("y={}",y);}


    // Matching multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        4..=6 => println!("in [4..6]"),
        _ => println!("anything"),
    }

    // Destructuring strucs
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;       // Sets a and b
    let Point { x, y } = p;             // Sets x and y

    match p {
        Point {x:0, y:0} => println!("Origin"),
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    // Destructuring enums
    let msg = Message::ChangeColor1(0, 160, 255);
    match msg {
        Message::Quit => { println!("The Quit variant has no data to destructure."); }
        Message::Move { x, y } => { println!("Move in the x direction {x} and in the y direction {y}"); }
        Message::Write(text) => { println!("Text message: {text}"); }
        Message::ChangeColor1(r, g, b) => { println!("Change the color to red {r}, green {g}, and blue {b}") }
        // Nested struct/enums
        Message::ChangeColor2(Color::Rgb(r, g, b)) => { println!("Change color to red {r}, green {g}, and blue {b}"); }
        Message::ChangeColor2(Color::Hsv(h, s, v)) => { println!("Change color to hue {h}, saturation {s}, value {v}") }
    }

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring parts with _ (_ doesn't bind at all)
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { println!("Can't overwrite an existing customized value"); }
        _ => { setting_value = new_setting_value; }
    }
    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // Ignoring variable name (and argument name) starting with _ (but this code is compiled with #![allow(unused)] so it's not visible)
    // note that _x binds whereas a simple _ does not bind
    let _unused = 12;

    // Ignoring Remaining Parts of a Value with .., quicker than including missing parts as _ (but must be non ambiguous)
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // Match guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // Match guards can use outer variables (a simple match expression can't)
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    // In case of multiple patterns with |, match guard applies to all patterns
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),      // behaves like (4 | 5 | 6) if y => ...
        _ => println!("no"),
    }

    // @ bindings, will print Found an id in range: 5
    // Using @ lets us test a value and save it in a variable within one pattern
    let msg = MessageHello::Hello { id: 5 };
    match msg {
        MessageHello::Hello { id: id_variable @ 3..=7, } => println!("Found an id in range: {id_variable}"),
        MessageHello::Hello { id: 10..=12 } => { println!("Found an id in another range") }
        MessageHello::Hello { id } => println!("Found some other id: {id}"),
    }

}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },        // struct
    Write(String),
    ChangeColor1(i32, i32, i32),     // tuple
    ChangeColor2(Color),
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum MessageHello {
    Hello { id: i32 },
}
