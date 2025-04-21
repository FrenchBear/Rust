// l44_macros: Learning Rust
//
// 2025-04-17	PV      First version
// 2025-04-21   PV      Clippy optimizations

#![allow(clippy::eq_op, clippy::erasing_op)]

// Simple macro
macro_rules! say_hello {
    () => {
        // Macro takes no argument
        // Macro will be expanded to the content of this block
        println!("Hello!");
    };
}

macro_rules! create_function {
    // This macro takes an argument of designator `ident` and creates a function named
    // `$func_name`.  The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {}()", stringify!($func_name));
        }
    };
}

// Create some functions
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints it as a string along with
    // its result.  The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{} = {}", stringify!($expression), $expression);
    };
}

// Overload example with different combinations of arguments (macro_rules! works similarly to match block)
// `test!` will compare `$left` and `$right` in different ways depending on how you invoke it:
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

// Macros can use + in the argument list to indicate that an argument may repeat at least once, or *, to indicate that
// the argument may repeat zero or more times.
// In the following example, surrounding the matcher with $(...),+ will match one or more expression, separated by
// commas. Also note that the semicolon is optional on the last case.
// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )   // Last semicolon is optional
}

// Domain-specific languages
// Note the two levels of braces in the macro, the outer one is part of the syntax of macro_rules!
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

// Variadic interfaces
macro_rules! calculate_var {
    // The pattern for a single `eval`
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate_var! { eval $e }
        calculate_var! { $(eval $es),+ }
    }};
}

fn main() {
    say_hello!();
    println!();

    foo();
    bar();
    println!();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
    println!();

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
    println!();

    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
    println!();

    calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }
    calculate! {
        eval (1 + 2) * (3 / 4)
    }
    println!();

    calculate_var! { // Look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
    println!();
}
