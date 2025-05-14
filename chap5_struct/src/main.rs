fn main() {
    println!("Struct!");
    // A struct, or structure, is a custom data type that lets you package together
    // and name multiple related values that make up a meaningful group

    println!("Defining and Instantiating Structs");

    let user1 = User{
        active: true,
        username: String::from("some.username123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("email = {}", user1.email); // someone@example.com

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
