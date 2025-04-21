// r09_restaurant
// Learning rust 2024, The Book §7 example, Play with modules, restaurant library
//
// 2024-11-10   PV

#![allow(dead_code, unused_variables)]

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Because the eat_at_restaurant function is defined in the same module as front_of_house (that is,
    // eat_at_restaurant and front_of_house are siblings), it's possible to refer front_of_house even if
    // were not flagged pub.
    // Here mod front_of_house is pub to be accessible from main.rs

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// -----------------------------
// Use of super

fn deliver_order() {}

// Private module, not accessible from outside
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    // In a pub struct, default for fields is still private
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // Customers can't cange the fruit
    }

    // Because Breakfast contains a private field, we must provide a public associated function that constructs
    // an instance of Breakfast, named summer here
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // On the other hand, for a pub enum, all variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_breakfast_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // seasonal_fruit is a private field, can't be accessed here
    //let f = meal.seasonal_fruit;
    //println!("Selected fruit: {f}"):

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

pub mod reservations {
    use phone_reservations::take_phone_reservation;

    pub enum ReservationMode {
        Phone,
        Email,
        Sms,
    }

    // These submodules are private
    mod email_reservations; // from email_reservations.rs
    mod sms_reservations; // from sms_reservations/mod.rs (old style path)

    mod phone_reservations {
        pub fn take_phone_reservation() {}
    }

    pub fn take_reservation(mode: ReservationMode) {
        match mode {
            ReservationMode::Phone => take_phone_reservation(),
            ReservationMode::Email => email_reservations::take_email_reservation(),
            ReservationMode::Sms => sms_reservations::take_sms_reservation(),
        }
    }
}
