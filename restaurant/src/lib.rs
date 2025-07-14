/*
    In the restaurant industry, some parts of a restaurant are referred to as front of house and others as back of house.
    Front of house is where customers are;
    this encompasses where the hosts seat customers, servers take orders and payment, and bartenders make drinks.
    Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.
 */

mod front_of_house {
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

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

fn deliver_order() {} // parent module

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order() // Using super allows us to reference an item that we know is in the parent module
    }

    // public struct
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // for enum only need pub before enum
    pub enum Appetizer {
        Soup,
        Salad
    }

    fn cook_order() {}
}

// use list
use std::{camp::Ordering, io};

// global operator
use std::collections::*;

