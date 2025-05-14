fn main() {
    println!("Struct!");
    // A struct, or structure, is a custom data type that lets you package together
    // and name multiple related values that make up a meaningful group

    println!("Defining and Instantiating Structs");

    let mut user1 = User{
        active: true,
        username: String::from("some.username123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("email = {}", user1.email); // someone@example.com

    // If the instance is mutable,
    // we can change a value by using the dot notation and assigning into a particular field
    user1.email = String::from("new.email@example.com");
    println!("new email = {}", user1.email); // new.email@example.com
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
