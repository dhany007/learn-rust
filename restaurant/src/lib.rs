mod front_of_house; // load a file using a mod declaration once in your module tree.

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
