fn main() {
    println!("Disambiguating between methods with the same name");

    // Method dengan Parameter self
    let person = Human;
    // Rust called the fly method implemented on Human directly.
    person.fly(); //*waving arms furiously*

    // To call the fly methods from either the Pilot trait or the Wizard trait
    // use more explicit syntax to specify which fly method we mean
    Pilot::fly(&person); // This is your captain speaking.
    Wizard::fly(&person); // up


    // Method tanpa parameter self
    println!(
        "a baby dog is called a {}",
        Dog::baby_name(),
    ); // a baby dog is called a Spot

    // format umumnya
    // <Type as Trait>::function(args...)
    println!(
        "a baby dog is called a {}",
        <Dog as Animal>::baby_name(),
    ); // a baby dog is called a puppy
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("up");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}