// l07_Restaurant
// Learning Rust again, modules
//
// 2023-06-10   PV

#![allow(dead_code, unused_variables)]

mod front_of_house; // Moved to a separate file instead of mod fron_of_house { content... }

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast with Tye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind!
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // Can't do this beacuse seasonal_fruit is provate
    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Relative path with super::
        super::deliver_order();
    }

    fn cook_order() {}

    // Only toast is public (struct fields are private by default)
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Since Breakfast.seasonal_fruit is private, it can't be constructed from outside
    // We need a local factory helper that can initialize seasonal_fruit field.
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In a public enum, all variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Use keyword, only for current scope
mod customer {
    use super::front_of_house::hosting;
    use super::front_of_house::hosting::add_to_waitlist as awl;

    fn another_lunch() {
        hosting::add_to_waitlist(); // Accessible thanks to use
        awl();
    }
}

mod another_customer {
    use super::front_of_house::hosting::*; // Using glob operator

    fn eat_too() {
        add_to_waitlist();
    }
}
