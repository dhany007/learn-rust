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

// function is part of our library crate’s public API,
// so we mark it with the pub keyword
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {} // parent module

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order() // Using super allows us to reference an item that we know is in the parent module
    }

    fn cook_order() {}
}