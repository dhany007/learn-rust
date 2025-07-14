/*
    In the restaurant industry, some parts of a restaurant are referred to as front of house and others as back of house.
    Front of house is where customers are;
    this encompasses where the hosts seat customers, servers take orders and payment, and bartenders make drinks.
    Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.
 */

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}